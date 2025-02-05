//! # GPT Tokenizer
//!
//! ## Overview
//! This module provides utility functions for encoding and decoding text using a Generative Pre-trained Transformer (GPT-2) tokenizer.
//! These functions are designed to facilitate the preprocessing of text data for natural language processing tasks and the postprocessing of tokenized data back into human-readable text.
//!
//! ## Functions
//!
//! ### Encode
//! ``
//!     encode(text: &[u8]) -> Result<Vec<u16>, _>
//! ``
//!
//! Encodes a given byte string into a list of (GPT-2) token IDs.
//! #### Parameters:
//! `text (&[u8])` : The input bytes string to be tokenized.
//! #### Returns:
//! `Result<Vec<u16>, _>` : A list of (GTP-2) token IDs representing the input byte string.
//!
//!
//! ### Decode
//! ``
//!     decode(encoding: &[u16]) -> Result<Vec<Vec<u8>>, _>
//! ``
//!
//! Decodes a list of (GPT-2) token IDs back into byte string.
//! #### Parameters:
//! `tokens (&[u8])` : The input bytes string to be tokenized.
//! #### Returns:
//! `Result<Vec<Vec<u8>>, _>` : The decoded byte string.
//!
//!
//! ## Usage Notes
//! *
//!
//! This module simplifies working with GPT tokenization, enabling efficient data preparation and interpretation for language models.
//!
mod unit;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::mem::swap;
use std::sync::LazyLock;
use regex::bytes::Regex;
use unicode_segmentation::UnicodeSegmentation;

/// Data structure of a byte pair.
///
/// ## Byte Pairing
type BytePair<T> = [Vec<T>; 2];

/// Data structure for storing byte pairings for a GPT UNICODE.
///
/// ## GPT token byte pairing
type TokenPairing<T> = (u16, BytePair<T>);

/// Data structure for storing a text grapheme in u8.
///
/// ## Grapheme
type Grapheme<T> = Vec<Vec<T>>;

/// Regular expression pattern for finding token contractions.
///
/// ## GPT2 Token Regex
pub(crate) const TOKEN_RE: &str =
    r"(?u)'s|'t|'re|'ve|'m|'l l|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(\S)|\s+";

/// Returns list of utf-8 byte and a corresponding list of unicode strings.
///     The reversible bpe codes work on unicode strings.
///     This means you need a large # of unicode characters in your vocab if you want to avoid UNKs.
///     When you're at something like a 10B token dataset you end up needing around 5K for decent coverage.
///     This is a significant percentage of your normal, say, 32K bpe vocab.
///     To avoid that, we want lookup tables between utf-8 bytes and unicode strings.
///     And avoids mapping to whitespace/control characters the bpe code barfs on.
///    
///  ```python
/// bs = list(range(ord("!"), ord("~") + 1)) + list(range(ord("¡"), ord("¬") + 1)) + list(range(ord("®"), ord("ÿ") + 1))
///  ```
///
/// ## GPT2 UNICODE
const UNICODE: [u16; 188] = [
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

/// Converts a sequence of bytes into a custom Unicode string using the specified encoding.
/// The BTreeMap uses a lazy static to initialize only on usage.
///
/// ## Bytes to unicode
pub(crate) static BYTES_TO_UNICODE: LazyLock<BTreeMap<u16, Vec<u8>>> = LazyLock::new(|| {
    let mut x = UNICODE.to_vec();
    let mut y: Vec<u16> = x.clone();
    let mut n: u16 = 0;
    for i in 0..=256 {
        if !x.contains(&i) {
            x.push(i);
            y.push(256 + n);
            n += 1;
        };
    }

    let mut unicodes = BTreeMap::new();
    for (i, c) in x.iter().enumerate() {
        let decoded =
            String::from_utf16(&[y[i]]).expect("[ERROR]: Could not load BYTES_TO_UNICODE.");
        unicodes.insert(*c, decoded.into_bytes());
    }
    unicodes
});

/// Converts a sequence of bytes into tokens suitable for use with a generative pre-trained transformer (GPT) model.
/// This conversion typically involves:
/// 1.  decoding the bytes to text using a specified encoding
/// 2.  followed by tokenization using a GPT-compatible tokenizer.
///
/// The BTreeMap uses a lazy static to initialize only on usage.
///
/// ## Bytes to token (GPT2)
static BYTES_TO_TOKEN: LazyLock<BTreeMap<Vec<u8>, u16>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/tokenizer/bytepairs.jsonl")
        .expect("[ERROR]: Unable to open file tokenizer/bytepairs.jsonl");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        let mut data: BTreeMap<String, u16> = serde_json::from_str(_line.as_str())
            .expect("[ERROR]: Unable to read file tokenizer/bytepairs.jsonl");
        while let Some((key, value)) = data.pop_first() {
            encoder.insert(key.into_bytes(), value);
        }
    }
    encoder
});

/// Converts a list of tokens generated by a GPT model back into a byte sequence.
/// This conversion typically involves:
/// 1.  decoding the tokens into text using a GPT-compatible tokenizer
/// 2.  then encoding the resulting text into bytes using a specified encoding.
///
/// The BTreeMap uses a lazy static to initialize only on usage.
///
/// ## Token to bytes (GPT2)
pub(crate) static TOKEN_TO_BYTES: LazyLock<BTreeMap<u16, Vec<u8>>> = LazyLock::new(|| {
    let mut decode = std::collections::BTreeMap::new();

    for (key, value) in BYTES_TO_TOKEN.iter() {
        let string: String = String::from_utf8(key.to_vec())
            .expect("[ERROR]: Unable to split file tokenizer/bytepairs.jsonl");

        if string.split_whitespace().count() == 1 {
            decode.insert(*value, key.to_vec());
        };
    }
    decode
});

/// The `contractions` function takes a sequence of bytes and splits it into a list of byte sequences, each representing a contraction.
/// The function uses a regular expression to represent contractions.
/// ```regex
/// (?u)'s|'t|'re|'ve|'m|'l l|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(\S)|\s+
/// ```
///
/// ## Contractions
pub fn contractions(slice: &[u8]) -> Result<Vec<Vec<u8>>, crate::error::Error> {
    Ok(Regex::new(crate::tokenizer::TOKEN_RE)
        .expect("Contractions regular expression error.")
        .find_iter(slice)
        .map(|m| -> Vec<u8> { m.as_bytes().to_vec() })
        .collect())
}

/// The `grapheme` function takes a sequence of bytes and returns a list of [extended grapheme clusters](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/),
/// which are groups of one or more code points that are perceived as a single character by users (e.g., emoji, accented letters).
/// The function first decodes the bytes into a GPT Unicode string.
///
/// ## Grapheme
pub(crate) fn grapheme(ngram: &[u8]) -> Result<Grapheme<u8>, crate::error::Error> {
    let symbol_to_bytes = |symbol: &str| -> Grapheme<u8> {
        symbol
            .chars()
            .flat_map(|c| -> Vec<u8> { String::from(c).into_bytes() })
            .map(|c| -> Vec<u8> {
                match BYTES_TO_UNICODE.get(&(c as u16)) {
                    Some(ch) => ch.to_vec(),
                    None => panic!("[ERROR]: Encoding value for '{:?}' not found!", c),
                }
            })
            .collect()
    };

    // let text = String::from_utf8(ngram.to_vec()).expect("[ERROR]: Invalid UTF-8 character found");
    let text = String::from_utf8_lossy(ngram);

    Ok(
        UnicodeSegmentation::graphemes(format!("{text}").as_str(), true)
            .flat_map(|symbol| -> Grapheme<u8> { symbol_to_bytes(symbol) })
            .collect(),
    )
}

/// The `to_pairs` function takes a byte sequence and returns a 2 window pairing for the sequence.
///
///
/// Functionality is similar to the below.
///
/// ```
/// let slice = &[[1,2],[3,4],[5,6]];
/// let mut iter = slice.windows(2);
/// assert_eq!(iter.next(), Some(&[[1,2],[3,4]][..]));
/// assert_eq!(iter.next(), Some(&[[3,4],[5,6]][..]));
/// assert_eq!(iter.next(), None);
/// ```
/// ## To pairs
fn to_pairs(parts: &Grapheme<u8>) -> Vec<BytePair<u8>> {
    parts
        .windows(2)
        .map(|pair| -> BytePair<u8> { [pair[0].to_owned(), pair[1].to_owned()] })
        .collect()
}

/// The `validate_merge` function takes two byte pairs and checks if they can be merged together.
/// The criteria for valid merging is for text processing use cases.
///
/// ## Validate byte merge
fn validate_byte_merge(this: &BytePair<u8>, other: &BytePair<u8>) -> bool {
    let this_left = format!("{}", String::from_utf8_lossy(&this[0]));
    let this_right = format!("{}", String::from_utf8_lossy(&this[1]));
    let other_left = format!("{}", String::from_utf8_lossy(&other[0]));
    let other_right = format!("{}", String::from_utf8_lossy(&other[1]));
    this_left.chars().last() == other_left.chars().last()
        && this_right.chars().next() == other_right.chars().next()
}

/// The `from_pairs` function takes a byte pair sequence and merges it together.
/// The criteria for the function is for it to be reversible with the `to_pairs` function.
///
/// ```
/// let to_pairs = &[[[1,2],[3,4]],[[3,4],[5,6]]];
/// let mut from_pairs = vec![];
/// let mut cursor = to_pairs.iter().peekable();
/// while let Some([left, right]) = cursor.next() {
///     from_pairs.push(left.to_vec());
///     if cursor.peek().is_none() {
///         from_pairs.push(right.to_vec());
///     };
/// }
/// assert_eq!(from_pairs , vec![[1,2],[3,4],[5,6]]);
/// ```
/// ## From pairs
fn from_pairs(bigrams: &[BytePair<u8>]) -> Grapheme<u8> {
    let mut grapheme = vec![];

    let mut cursor = bigrams.iter().peekable();

    while let Some([left, right]) = cursor.next() {
        grapheme.push(left.to_vec());
        if cursor.peek().is_none() {
            grapheme.push(right.to_vec());
        };
    }

    grapheme
}

/// The `merge` function uses the `from_pairs` function to invert a byte pair sequence into a grapheme byte sequece
/// and the GPT token equivalent.
///
/// ## Tokens
fn tokens(bytepairing: Vec<BytePair<u8>>) -> Option<(Grapheme<u8>, Vec<u16>)> {
    let grapheme = from_pairs(&bytepairing);
    let mut tokens = vec![];
    let is_tokenized = {
        for key in &grapheme {
            if let Some(value) = BYTES_TO_TOKEN.get(key) {
                tokens.push(*value)
            };
        }
        tokens.len() == grapheme.len()
    };

    if is_tokenized {
        Some((grapheme, tokens))
    } else {
        None
    }
}
/// The BytePairEncoder struct is responsible for encoding and decoding text using the Byte Pair Encoding (BPE) method,
/// commonly used in GPT models for tokenization.
pub(crate) struct BytePairEncoder<T> {
    /// GPT UNICODE Representation of the text [extended grapheme clusters](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/).
    ///
    /// ## Grapheme
    grapheme: Grapheme<T>,
    /// GPT Token Representation of the text from byte pairing.
    ///
    /// Note:
    ///
    /// ``
    /// BytePairEncoder::grapheme.len() == BytePairEncoder::tokens.len();
    /// ``
    ///
    /// ## Tokens
    tokens: Vec<u16>,
    /// List of recognizable byte pairing from encoder training.
    ///
    /// A byte pair is popped out of this list on every encoder iteration.
    ///
    /// ## Byte Pairs
    bytepairs: Vec<TokenPairing<T>>,
    /// List of byte pairs that have been popped out of the `bytepairs` list on every iteration.
    ///
    /// This is to ensure that the value is not used again.
    ///
    /// ## Byte Pair cache.
    cache: HashSet<BytePair<T>>,
}

/// For ergonomic reasons.
/// Opting to implement the byte pair merge function as AddAssign
impl std::ops::AddAssign<&BytePair<u8>> for BytePairEncoder<u8> {
    fn add_assign(&mut self, pair: &BytePair<u8>) {
        let bigrams = to_pairs(&self.grapheme);
        let mut binding = bigrams.to_vec();
        let mut cursor = bigrams.iter().enumerate().peekable();

        while let Some((index, current)) = cursor.next() {
            if let Some((_, next)) = cursor.peek() {
                if validate_byte_merge(next, pair) && (index + 1) == binding.len() {
                    swap(
                        &mut binding[index],
                        &mut [
                            current[0].to_owned(),
                            [next[0].to_owned(), next[1].to_owned()].concat(),
                        ],
                    );
                    binding.remove(index + 1);
                    if let Some((grapheme, tokens)) = tokens(binding) {
                        self.grapheme = grapheme;
                        self.tokens = tokens;
                    };
                    break;
                };
                if validate_byte_merge(current, pair) {
                    swap(
                        &mut binding[index],
                        &mut [
                            [current[0].to_owned(), current[1].to_owned()].concat(),
                            next[1].to_owned(),
                        ],
                    );
                    binding.remove(index + 1);
                    if let Some((grapheme, tokens)) = tokens(binding) {
                        self.grapheme = grapheme;
                        self.tokens = tokens;
                    };
                    break;
                };
            }
        }
    }
}

impl BytePairEncoder<u8> {
    /// The tick part of a tick-tokenizer.
    /// The function completes the following steps of the byte pair encoder:
    ///
    /// 1. Splits grapheme to byte pairs.
    /// 2. Checks for new byte pairs.
    /// 3. Adds the new byte pairs into iterator list.
    ///
    /// ## Tick
    fn tick(&mut self) {
        for [left, right] in to_pairs(&self.grapheme) {
            let pair: BytePair<u8> = [left, right];
            if !self.cache.contains(&pair) {
                if let Some(rank) = BYTES_TO_TOKEN.get(&pair.concat()) {
                    self.bytepairs
                        .push((*rank, [pair[0].to_owned(), pair[1].to_owned()]));
                };
                self.cache.insert([pair[0].to_owned(), pair[1].to_owned()]);
            };
        }
        self.bytepairs.sort_by(|a, b| b.0.cmp(&a.0));
    }
}

impl From<&Grapheme<u8>> for BytePairEncoder<u8> {
    fn from(value: &Grapheme<u8>) -> Self {
        let mut encoder = BytePairEncoder {
            grapheme: value.to_vec(),
            tokens: vec![],
            bytepairs: vec![],
            cache: HashSet::new(),
        };
        encoder.tick();
        encoder
    }
}

impl Iterator for BytePairEncoder<u8> {
    type Item = Vec<u16>;

    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(debug_assertions)]
        println!("[DEBUG][BYTE_PAIRS]: {:?}", self.bytepairs);

        match self.grapheme.len() == 1 || self.bytepairs.is_empty() {
            true => None,
            false => {
                if let Some((_, bytepair)) = self.bytepairs.pop() {
                    *self += &bytepair;
                    self.tick();
                };
                {
                    Some(self.tokens.to_vec())
                }
            }
        }
    }
}
