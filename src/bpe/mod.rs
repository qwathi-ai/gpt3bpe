//! # Byte-Pair Encoding (BPE)
//!
//! This module provides the core implementation for Byte-Pair Encoding, a tokenization
//! strategy used by GPT models. It includes functionality for:
//!
//! - Splitting text into initial tokens based on a regex pattern.
//! - Mapping raw bytes to a "safe" set of Unicode characters to handle arbitrary byte sequences.
//! - Applying BPE merges based on a predefined rank table.
//! - Encoding text into token IDs and decoding them back into text.
//!
//! The implementation is inspired by Andrej Karpathy's [picoGPT](https://github.com/jaymody/picoGPT) project.

pub(crate) mod unit;
pub(crate) mod vocabulary;
use regex::bytes::Regex;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::convert::From;
use std::fmt::Debug;
use std::fmt::Display;
use std::sync::LazyLock;
use unicode_segmentation::UnicodeSegmentation;

/// A type alias representing a pair of bytes and its rank during the BPE merge process.
/// The `usize` is the index of the first byte in the pair, and `Type` is its merge rank.
type BytePair<Type> = (usize, Type);

/// A regex pattern for the initial splitting of text into processable chunks.
///
/// This pattern is designed to handle various text structures found in GPT-2/3 tokenization,
/// including:
/// - Contractions (e.g., "'s", "'t", "'re").
/// - Words composed of letters (`\p{L}+`).
/// - Sequences of numbers (`\p{N}+`).
/// - Punctuation and other non-alphanumeric characters.
/// - Whitespace.
const TOKENS_RE: &str =
    r"(u)'s|'t|'re|'ve|'m|'l l|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(\S)|\s+";

/// A set of "safe" Unicode characters used for the reversible BPE mapping.
///
/// The BPE algorithm works on Unicode strings, but raw byte streams can contain values
/// that are not valid Unicode or are control characters that can cause issues. To handle
/// any possible byte value from 0 to 255, this scheme maps each byte to a distinct, printable
/// Unicode character. This avoids "unknown token" errors and makes the tokenization process
/// fully reversible.
///
/// This array defines the initial set of characters, which is then extended to cover all 256 byte values.
const GPT_UNICODES: [u16; 188] = [
    33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103,
    104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
    123, 124, 125, 126, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 174, 175, 176,
    177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195,
    196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214,
    215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233,
    234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252,
    253, 254, 255,
];

/// Lazily initialized map from a raw byte value (as a `u16`) to its "safe" Unicode representation.
///
/// This is part of the reversible BPE scheme. It ensures that every possible byte can be
/// represented as a unique, printable Unicode character before tokenization.
static UNICODE_TO_BYTES: LazyLock<BTreeMap<u16, Vec<u8>>> = LazyLock::new(|| {
    let mut x = GPT_UNICODES.to_vec();
    let mut y: Vec<u16> = x.clone();
    let mut n: u16 = 0;
    for i in 0..=256 {
        if !x.contains(&i) {
            x.push(i);
            y.push(256 + n);
            n += 1;
        };
    }

    let mut tree = BTreeMap::new();
    for (i, unicode) in x.iter().enumerate() {
        let symbol = String::from_utf16_lossy(&[y[i]]);
        tree.insert(*unicode, symbol.into_bytes());
    }
    tree
});

/// Lazily initialized map from a "safe" Unicode representation back to its original raw byte value.
///
/// This is the inverse of `UNICODE_TO_BYTES` and is used during the decoding process to
/// reconstruct the original byte stream from the tokenized Unicode characters.
static BYTES_TO_UNICODE: LazyLock<BTreeMap<Vec<u8>, u16>> = LazyLock::new(|| {
    let mut x = GPT_UNICODES.to_vec();
    let mut y: Vec<u16> = x.clone();
    let mut n: u16 = 0;
    for i in 0..=256 {
        if !x.contains(&i) {
            x.push(i);
            y.push(256 + n);
            n += 1;
        };
    }

    let mut tree = BTreeMap::new();
    for (i, unicode) in x.iter().enumerate() {
        let symbol = String::from_utf16_lossy(&[y[i]]);
        tree.insert(symbol.into_bytes(), *unicode);
    }
    tree
});

/// Lazily initialized map of BPE merge rules.
///
/// This map contains the core logic of the Byte-Pair Encoding algorithm. It maps a pair of
/// subword units (as a byte vector) to its merge rank (a `u32`). The lower the rank, the
/// earlier the pair is merged.
/// The `merges.txt` file is embedded at compile time, but can be overridden at runtime by setting the `MERGES` environment variable.
static MERGES: LazyLock<HashMap<Vec<u8>, u32>> = LazyLock::new(|| {
    let merges_contents: &str = match std::env::var("MERGES") {
        Ok(l) => {
            // return the contents of the file by dynamically reading in the file.
            &std::fs::read_to_string(l).unwrap()
        }
        Err(_) => include_str!("merges.txt"),
    };
    merges_contents
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                Some([parts[0].as_bytes(), parts[1].as_bytes()].concat())
            } else {
                None
            }
        })
        .enumerate()
        .map(|(i, bytes)| (bytes, i as u32))
        .collect()
});

/// Splits a byte slice into a sequence of GPT-style Unicode graphemes.
///
/// This function first segments the input text into Unicode grapheme clusters to correctly
/// handle multi-byte characters. It then converts each raw byte of the graphemes into its
/// corresponding "safe" Unicode representation using the `UNICODE_TO_BYTES` map.
///
/// This is a crucial pre-processing step before applying BPE merges.
pub fn grapheme(slice: &[u8]) -> Vec<Vec<u8>> {
    let char_to_unicode = |char: &str| -> Vec<Vec<u8>> {
        char.chars()
            .flat_map(|c| -> Vec<u8> { String::from(c).into_bytes() })
            .map(|bytes| -> Vec<u8> {
                match UNICODE_TO_BYTES.get(&(bytes as u16)) {
                    Some(unicode) => unicode.to_vec(),
                    None => panic!("[ERROR]: Encoding value for '{bytes:?}' not found!"),
                }
            })
            .collect()
    };

    let text = String::from_utf8_lossy(slice);
    UnicodeSegmentation::graphemes(format!("{text}").as_str(), true)
        .flat_map(|char| -> Vec<Vec<u8>> { char_to_unicode(char) })
        .collect()
}

/// Splits a byte slice into initial token chunks based on the `TOKENS_RE` regex.
///
/// This function performs the first pass of tokenization, breaking the input text into
/// larger, more manageable pieces like words, numbers, punctuation, and contractions before
/// the BPE merging process begins.
pub fn tokens(slice: &[u8]) -> Vec<&[u8]> {
    Regex::new(TOKENS_RE)
        .unwrap()
        .find_iter(slice)
        .map(|m| -> &[u8] { m.as_bytes() })
        .collect()
}

/// An iterator that performs the Byte-Pair Encoding merge process.
///
/// It iteratively finds the byte pair with the lowest merge rank, merges it,
/// and yields the new sequence of token IDs. This continues until no more
/// merges are possible.
struct BytePairEncoder {
    /// The sequence of graphemes to be merged.
    pub grapheme: Vec<Vec<u8>>,
    /// A vector where each element is a tuple containing the start index of a unit in `grapheme`
    /// and the rank of the pair formed by this unit and the next.
    pairs: Vec<BytePair<u32>>,
    /// A map from a byte sequence (a potential token) to its rank or token ID.
    encoder: BTreeMap<Vec<u8>, u32>,
}

impl BytePairEncoder {
    /// Creates a new `BytePairEncoder`.
    pub fn new<T: Into<u32> + Copy + Ord + Debug>(
        grapheme: Vec<Vec<u8>>,
        lookup: &BTreeMap<Vec<u8>, T>,
    ) -> BytePairEncoder {
        let mut encoder: BTreeMap<Vec<u8>, u32> = std::collections::BTreeMap::new();

        for (key, value) in MERGES.iter() {
            encoder.insert(key.to_vec(), *value);
        }
        for (key, value) in lookup.iter() {
            encoder.insert(key.to_vec(), (*value).into());
        }

        let mut pairs: Vec<BytePair<u32>> = (0..grapheme.len()).map(|i| (i, u32::MAX)).collect();
        for i in 0..pairs.len() - 1 {
            if let Some(rank) = encoder.get(&grapheme[pairs[i].0..pairs[i + 1].0 + 1].concat()) {
                pairs[i].1 = *rank;
            }
        }

        BytePairEncoder {
            grapheme,
            pairs,
            encoder,
        }
    }

    /// Gets the rank of a potential merged pair starting at `start_idx` with a given `length`.
    fn get_rank(&self, start_idx: usize, length: usize) -> Option<u32> {
        if start_idx + length <= self.pairs.len() {
            self.encoder
                .get(
                    &self.grapheme
                        [self.pairs[start_idx].0..self.pairs[start_idx + length - 1].0 + 1]
                        .concat(),
                )
                .copied()
        } else {
            None
        }
    }
}

impl Iterator for BytePairEncoder {
    type Item = Vec<u32>;

    /// Finds the next best pair to merge and returns the new sequence of token IDs.
    ///
    /// On each call, it identifies the pair with the lowest rank, merges them into a single unit,
    /// and updates the ranks of the adjacent pairs. It then returns the full list of current token IDs.
    fn next(&mut self) -> Option<Self::Item> {
        if self.pairs.len() == 1 {
            return None;
        }

        let mut rank: (u32, usize) = (u32::MAX, 0);
        for (idx, &(_, r)) in self.pairs[..self.pairs.len() - 1].iter().enumerate() {
            if r < rank.0 {
                rank = (r, idx);
            }
        }

        if rank.0 == u32::MAX {
            return None;
        }

        // The rank of the new merged pair will be the rank of it with its new right neighbor.
        // The original right neighbor was at rank.1 + 2. After removing rank.1 + 1, it will be at rank.1 + 1.
        if rank.1 < self.pairs.len() - 2 {
            self.pairs[rank.1].1 = self.get_rank(rank.1, 3).unwrap_or(u32::MAX);
        } else {
            self.pairs[rank.1].1 = u32::MAX;
        }

        if rank.1 > 0 {
            self.pairs[rank.1 - 1].1 = self.get_rank(rank.1 - 1, 2).unwrap_or(u32::MAX);
        }
        self.pairs.remove(rank.1 + 1);

        let mut result = Vec::with_capacity(self.pairs.len());
        for i in 0..self.pairs.len() {
            let start = self.pairs[i].0;
            let end = if i < self.pairs.len() - 1 {
                self.pairs[i + 1].0
            } else {
                self.grapheme.len()
            };
            match self
                .encoder
                .get(&self.grapheme[start..end].concat())
                .copied()
            {
                Some(v) => result.push(v),
                None => {
                    // If a token is not found, it implies that the BPE process cannot proceed further with valid merges.
                    // By returning `None`, we stop the iteration. The `encode` function will use the last successfully
                    // generated token list from the previous `next()` call.
                    #[cfg(debug_assertions)]
                    println!(
                        "[WARNING]: Encoding value for {:?} not found.",
                        String::from_utf8_lossy(&self.grapheme[start..end].concat())
                    );
                    return None;
                }
            }
        }
        Some(result)
    }
}

/// Encodes a byte slice into a vector of token ID vectors using BPE.
///
/// This function orchestrates the entire encoding process:
/// 1. It splits the input `slice` into initial chunks using `tokens`.
/// 2. For each chunk, it checks if it exists as a whole token in the `lookup` table.
/// 3. If not, it converts the chunk to graphemes and uses `BytePairEncoder` to
///    iteratively merge subword units until no more merges are possible.
/// 4. The final token IDs for each chunk are collected and returned.
///
/// Each inner `Vec<u32>` corresponds to the tokens from one of the initial chunks.
pub fn encode<T: Copy + Ord + Debug + Into<u32>>(
    slice: &[u8],
    lookup: &LazyLock<BTreeMap<Vec<u8>, T>>,
) -> Vec<Vec<u32>> {
    let mut result = vec![];

    for piece in tokens(slice) {
        let graph = grapheme(piece);
        if let Some(token) = lookup.get(&graph.concat()) {
            result.push(vec![<T as Into<u32>>::into(*token)]);
            continue;
        }

        let merge = graph
            .iter()
            .flat_map(|g| g.iter().map(|r| *r as u32))
            .collect();
        let encoder = BytePairEncoder::new(graph, lookup);
        match encoder.last() {
            None => result.push(merge),
            Some(merge) => result.push(merge),
        }
    }
    result
}

/// Decodes a slice of token IDs back into a byte vector.
///
/// This function reverses the encoding process:
/// 1. It looks up each token ID in the `lookup` table to get its corresponding "safe"
///    Unicode characters.
/// 2. It segments the resulting Unicode string into graphemes.
/// 3. Each grapheme is then mapped back to its original raw byte value using the
///    `BYTES_TO_UNICODE` map.
pub fn decode<T: Copy + Ord + Debug + Display>(
    tokens: &[T],
    lookup: &LazyLock<BTreeMap<T, Vec<u16>>>,
) -> Vec<u8> {
    tokens
        .iter()
        .flat_map(|token| {
            let unicode_chars = lookup
                .get(token)
                .unwrap_or_else(|| panic!("[ERROR]: Token ID {token:?} not found."));

            let gpt_unicode_bytes: Vec<u8> = unicode_chars.iter().map(|&c| c as u8).collect();
            let gpt_unicode_string = String::from_utf8(gpt_unicode_bytes).unwrap();

            UnicodeSegmentation::graphemes(gpt_unicode_string.as_str(), true)
                .map(|grapheme_str| {
                    let grapheme_bytes = grapheme_str.as_bytes();
                    *BYTES_TO_UNICODE.get(grapheme_bytes).unwrap_or_else(|| {
                        panic!("[ERROR]: Decoding value for '{grapheme_str}' not found!")
                    }) as u8
                })
                .collect::<Vec<u8>>()
        })
        .collect()
}
