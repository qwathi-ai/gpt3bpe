//! Module inspired by [PicoGPT](https://github.com/jaymody/picoGPT) project.

mod unit;
pub(crate) mod vocabulary;
use regex::bytes::Regex;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::convert::From;
use std::fmt::Debug;
use std::fmt::Display;
use std::sync::LazyLock;
use unicode_segmentation::UnicodeSegmentation;

/// Data structure for byte pairings of type `[T]`.
///
/// ## Byte Pair
type BytePair<Type> = (usize, Type);

/// Regular expression pattern for finding token contractions.
///
/// ## Tokens regular expression
const TOKENS_RE: &str =
    r"(u)'s|'t|'re|'ve|'m|'l l|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(\S)|\s+";

/// I like the original comment on this. So I'm keeping it.
///
///
/// > Returns list of utf-8 byte and a corresponding list of unicode strings.
/// > The reversible bpe codes work on unicode strings.
/// > This means you need a large # of unicode characters in your vocab if you want to avoid UNKs.
/// > When you're at something like a 10B token dataset you end up needing around 5K for decent coverage.
/// > This is a significant percentage of your normal, say, 32K bpe vocab.
/// > To avoid that, we want lookup tables between utf-8 bytes and unicode strings.
/// > And avoids mapping to whitespace/control characters the bpe code barfs on.
///    
///  ```python
/// bs = list(range(ord("!"), ord("~") + 1)) + list(range(ord("¡"), ord("¬") + 1)) + list(range(ord("®"), ord("ÿ") + 1))
///  ```
///
/// ## UNICODES
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

/// Maps GPT unicode scheme to u8 byte vector.
///
/// ## Unicode to bytes
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

/// Maps u8 byte vector to GPT unicode scheme.
///
/// ## Bytes to unicode
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

/// ## Merges
static MERGES: LazyLock<HashMap<Vec<u8>, u32>> = LazyLock::new(|| {
    const MERGES_CONTENTS: &str = include_str!("merges.txt");
    MERGES_CONTENTS
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

///
/// ## Grapheme
/// ### Arguments
/// * `slice` - byte vector
///
/// ### Returns
/// * GPT Unicode characters.
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

/// Find token contractions in a byte vector.
/// See [token regular expression](crate::tokenizer::TOKENS_RE) for implementation.
///
/// ## Tokenizer
/// ### Arguments
/// * `slice` - byte vector
///
/// ### Returns
/// * token contractions.
fn tokens(slice: &[u8]) -> Vec<&[u8]> {
    Regex::new(TOKENS_RE)
        .unwrap()
        .find_iter(slice)
        .map(|m| -> &[u8] { m.as_bytes() })
        .collect()
}

/// Responsible for encoding and decoding text using the Byte Pair Encoding method, commonly used for tokenization.
struct BytePairEncoder {
    ///
    /// ## Slice
    pub grapheme: Vec<Vec<u8>>,

    ///
    /// ## Pairs
    pairs: Vec<BytePair<u32>>,

    ///
    /// ## Encoder
    encoder: BTreeMap<Vec<u8>, u32>,
}

impl BytePairEncoder {
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
                        "[WARNING]: Encoding value for {:?} not found at {:?}:{:?} for index {:?}",
                        String::from_utf8_lossy(&self.grapheme[start..end].concat()),
                        start,
                        end,
                        i
                    );
                    return None;
                }
            }
        }
        Some(result)
    }
}

/// Encodes a given byte slice into a token vector.
/// ## Encode
///
/// ### Arguments
/// * `slice` - a byte vector.
/// * `lookup` - a lookup table with vocabulary scheme (slice to tokens).
///
/// ### Returns
/// * a [token](crate::tokenizer::tokens) vector equivalent of slice.
pub(crate) fn encode<T: Copy + Ord + Debug + Into<u32>>(
    slice: &[u8],
    lookup: &LazyLock<BTreeMap<Vec<u8>, T>>,
) -> Vec<u32> {
    let mut result = vec![];

    for piece in tokens(slice) {
        let graph = grapheme(piece);
        if let Some(token) = lookup.get(&graph.concat()) {
            result.push(<T as Into<u32>>::into(*token));
            continue;
        }

        let mut merge = graph
            .iter()
            .flat_map(|g| g.iter().map(|r| *r as u32))
            .collect();
        let encoder = BytePairEncoder::new(graph, lookup);
        for m in encoder {
            if !m.is_empty() {
                merge = m;
            }
        }
        result.extend(merge)
    }
    result
}

/// Decode a given token vector into a byte slice.
/// ## Decode
///
/// ### Arguments
/// * `tokens` - a token vector.
/// * `lookup` - a lookup table with vocabulary scheme (tokens to slice).
///
/// ### Returns
/// * a byte slice.
pub(crate) fn decode<T: Copy + Ord + Debug + Display>(
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
            let gpt_unicode_string = String::from_utf8_lossy(&gpt_unicode_bytes);

            UnicodeSegmentation::graphemes(gpt_unicode_string.as_ref(), true)
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
