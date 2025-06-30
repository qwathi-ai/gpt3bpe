mod unit;
use regex::bytes::Regex;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::mem::swap;
use std::sync::LazyLock;
use unicode_segmentation::UnicodeSegmentation;

/// Data structure for byte pairings of type `[T]`.
///
/// ## Byte Pair
type BytePair<T> = [Vec<T>; 2];

/// Data structure for mapping byte pairings to tokens of type `[T]`.
///
/// ## Token pairing
type TokenPairing<T> = (u16, BytePair<T>);

/// Data structure for storing a text grapheme of type `[T]`.
///
/// ## Grapheme
type Grapheme<T> = Vec<Vec<T>>;

/// Regular expression pattern for finding token contractions.
///
/// ## Tokens regular expression
const TOKENS_RE: &str =
    r"(u)'s|'t|'re|'ve|'m|'l l|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(\S)|\s+";

/// I like the original comment on this. So I'm keeping it.
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
/// ## GPT UNICODES
pub(crate) const GPT_UNICODES: [u16; 188] = [
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

/// Maps u8 byte vector to unicodes.
///
/// ## Bytes to unicodes
static BYTES_TO_GPT_UNICODES: LazyLock<BTreeMap<u16, Vec<u8>>> = LazyLock::new(|| {
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

    let mut unicodes = BTreeMap::new();
    for (i, c) in x.iter().enumerate() {
        let decoded = String::from_utf16_lossy(&[y[i]]);
        unicodes.insert(*c, decoded.into_bytes());
    }
    unicodes
});

/// Maps unicodes to vocabulary tokens.
///
/// ## Unicodes to tokens
pub(crate) static GPT_UNICODES_TO_TOKENS: LazyLock<BTreeMap<Vec<u8>, u16>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/tokenizer/bytepairs.jsonl")
        .expect("[ERROR]: Could not load GPT_UNICODES_TO_TOKENS");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        let mut data: BTreeMap<String, u16> = serde_json::from_str(_line.as_str())
            .expect("[ERROR]: Could not load GPT_UNICODES_TO_TOKENS");
        while let Some((key, value)) = data.pop_first() {
            encoder.insert(key.into_bytes(), value);
        }
    }
    encoder
});

/// Maps tokens back into unicodes.
///
/// ## Tokens to unicodes
pub(crate) static TOKENS_TO_GPT_UNICODES: LazyLock<BTreeMap<u16, Vec<Vec<u8>>>> =
    LazyLock::new(|| {
        let mut decode = std::collections::BTreeMap::new();

        for (key, value) in GPT_UNICODES_TO_TOKENS.iter() {
            let string: String = String::from_utf8(key.to_vec())
                .expect("[ERROR]: Could not load TOKENS_TO_GPT_UNICODES");

            if string.split_whitespace().count() == 1 {
                decode.insert(*value, vec![key.to_vec()]);
            };
        }
        decode
    });

/// Maps unicodes to u8 byte vector.
///
/// ## Unicodes to bytes
pub(crate) static GPT_UNICODES_TO_BYTES: LazyLock<BTreeMap<Vec<u8>, u8>> = LazyLock::new(|| {
    let mut unicodes = std::collections::BTreeMap::new();
    for (unicode, byte) in BYTES_TO_GPT_UNICODES.iter() {
        unicodes.insert(byte.to_vec(), *unicode as u8);
    }
    unicodes
});

/// Find token contractions in a byte vector.
/// Refer to [TOKENS_RE].
///
/// ## Tokenizer
/// ### Arguments
/// * `slice` - byte vector
///
/// ### Returns
/// * token contractions.
pub(crate) fn tokens(slice: &[u8]) -> Result<Vec<Vec<u8>>, crate::error::Error> {
    Ok(Regex::new(TOKENS_RE)?
        .find_iter(slice)
        .map(|m| -> Vec<u8> { m.as_bytes().to_vec() })
        .collect())
}

/// Function takes a u8 byte vector and returns a list of [unicode](crate::tokenizer::GPT_UNICODES) characters.
///
/// ## Grapheme
/// ### Arguments
/// * `slice` - byte vector
///
/// ### Returns
/// * gpt unicode characters.
pub(crate) fn grapheme(slice: &[u8]) -> Result<Grapheme<u8>, crate::error::Error> {
    let symbol_to_bytes = |symbol: &str| -> Grapheme<u8> {
        symbol
            .chars()
            .flat_map(|c| -> Vec<u8> { String::from(c).into_bytes() })
            .map(|c| -> Vec<u8> {
                match BYTES_TO_GPT_UNICODES.get(&(c as u16)) {
                    Some(ch) => ch.to_vec(),
                    None => panic!("[ERROR]: Encoding value for '{:?}' not found!", c),
                }
            })
            .collect()
    };

    let text = String::from_utf8_lossy(slice);

    Ok(
        UnicodeSegmentation::graphemes(format!("{text}").as_str(), true)
            .flat_map(|symbol| -> Grapheme<u8> { symbol_to_bytes(symbol) })
            .collect(),
    )
}

/// Takes a byte vector and returns a 2 [window](std::slice::Windows) byte pairing of the vector.
/// ## To pairs
/// ### Arguments
/// * `parts` - byte vector
///
/// ### Returns
/// * a 2 [window](std::slice::Windows) byte paring.
fn to_pairs(parts: &Grapheme<u8>) -> Vec<BytePair<u8>> {
    parts
        .windows(2)
        .map(|pair| -> BytePair<u8> { [pair[0].to_owned(), pair[1].to_owned()] })
        .collect()
}

/// Takes two byte parings and checks if they can be merged together.
///
/// ## Validate byte merge
/// ### Arguments
/// * `this` - byte pairing
/// * `other` - byte pairing
///
/// ### Returns
/// * a boolean
fn validate_byte_merge(this: &BytePair<u8>, other: &BytePair<u8>) -> bool {
    let this_left = String::from_utf8(this[0].to_vec()).unwrap().to_string();
    let this_right = String::from_utf8(this[1].to_vec()).unwrap().to_string();
    let other_left = String::from_utf8(other[0].to_vec()).unwrap().to_string();
    let other_right = String::from_utf8(other[1].to_vec()).unwrap().to_string();
    this_left.chars().last() == other_left.chars().last()
        && this_right.chars().next() == other_right.chars().next()
}

/// Takes a 2 [window](std::slice::Windows) byte pair slice and returns a byte vector.
/// ## From pairs
/// ### Arguments
/// * `bigrams` - 2 window byte pairing slice
///
/// ### Returns
/// * a byte vector
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

/// Maps a byte pair vector into tuple with a byte vector and equivalent token vector
///
/// ## Contraction
/// ### Arguments
/// * `byte paring` - byte pair vector
///
/// ### Returns
///
/// TODO:
/// * byte vector and equivalent token vector
fn contraction(bytepairing: Vec<BytePair<u8>>) -> Option<(Grapheme<u8>, Vec<u16>)> {
    let grapheme = from_pairs(&bytepairing);
    let mut tokens = vec![];

    let is_tokenized = {
        for key in &grapheme {
            if let Some(value) = GPT_UNICODES_TO_TOKENS.get(key) {
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

/// Responsible for encoding and decoding text using the Byte Pair Encoding method, commonly used for tokenization.
pub(crate) struct BytePairEncoder<E, D> {
    /// [GPT Unicode](crate::tokenizer::GPT_UNICODES) Representation of text in [extended grapheme clusters](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/).
    ///
    /// ## Grapheme
    grapheme: Grapheme<E>,
    /// Token Representation of the text from byte pairing.
    ///
    /// *Note*:
    /// ``
    /// Encoder::grapheme.len() == Encoder::tokens.len();
    /// ``
    ///
    /// ## Tokens
    tokens: Vec<D>,
    /// List of recognizable byte pairs from encoder training.
    ///
    /// A byte pair is popped out of this list on every encoder iteration.
    ///
    /// ## Byte Pairs
    bytepairs: Vec<TokenPairing<E>>,
    /// List of byte pairs that have been popped out of the `bytepairs` list on every iteration.
    ///
    /// This is to ensure that the value is not used again.
    ///
    /// ## Byte Pair cache.
    cache: HashSet<BytePair<E>>,
}

impl BytePairEncoder<u8, u16> {
    /// The tick part of a tick-tokenizer.
    /// The function completes the following steps for the byte pair encoder:
    ///
    /// 1. Splits grapheme to byte pairs.
    /// 2. Checks for new byte pairs.
    /// 3. Adds the new byte pairs into iterator list.
    /// 4. Sorts byte pairs.
    ///
    /// ## Tick
    fn tick(&mut self) {
        for [left, right] in to_pairs(&self.grapheme) {
            let pair: BytePair<u8> = [left, right];

            if !self.cache.contains(&pair) {
                if let Some(rank) = GPT_UNICODES_TO_TOKENS.get(&pair.concat()) {
                    self.bytepairs
                        .push((*rank, [pair[0].to_owned(), pair[1].to_owned()]));
                };
                self.cache.insert([pair[0].to_owned(), pair[1].to_owned()]);
            };
        }
        self.bytepairs.sort_by(|a, b| b.0.cmp(&a.0));
    }
}

/// For ergonomic reasons.
/// Opting to implement the byte pair merge function as AddAssign
impl std::ops::AddAssign<&BytePair<u8>> for BytePairEncoder<u8, u16> {
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
                    if let Some((grapheme, tokens)) = contraction(binding) {
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
                    if let Some((grapheme, tokens)) = contraction(binding) {
                        self.grapheme = grapheme;
                        self.tokens = tokens;
                    };
                    break;
                };
            }
        }
    }
}

impl From<&Grapheme<u8>> for BytePairEncoder<u8, u16> {
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

impl Iterator for BytePairEncoder<u8, u16> {
    type Item = Vec<u16>;

    fn next(&mut self) -> Option<Self::Item> {
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
