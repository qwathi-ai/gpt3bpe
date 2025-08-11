mod unit;
pub(crate) mod vocabulary;

use std::mem::swap;
use regex::bytes::Regex;
use std::sync::LazyLock;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;
///! Module inspired by [PicoGPT](https://github.com/jaymody/picoGPT) project.
///

/// Data structure for byte pairings of type `[T]`.
///
/// ## Byte Pair
type BytePair<Type> = [Vec<Type>; 2];

/// Data structure for mapping byte pairings to tokens of type `[T]`.
///
/// ## Token pairing
type TokenPairing<Type> = (usize, BytePair<Type>);
/// Data structure for storing a text grapheme of type `[T]`.
///
/// ## Grapheme
// type Grapheme64<Type> = Vec<[Type; 64]>;
type Grapheme<Type> = Vec<Vec<Type>>;

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

/// Maps u8 byte vector to GPT unicode scheme.
///
/// ## Bytes to unicodes
static BYTES_TO_UNICODES: LazyLock<BTreeMap<u16, Vec<u8>>> = LazyLock::new(|| {
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

/// Maps GPT unicode scheme to u8 byte vector.
///
/// ## Unicodes to bytes
static UNICODES_TO_BYTES: LazyLock<BTreeMap<Vec<u8>, u8>> = LazyLock::new(|| {
    let mut unicodes = std::collections::BTreeMap::new();
    for (unicode, byte) in BYTES_TO_UNICODES.iter() {
        unicodes.insert(byte.to_vec(), *unicode as u8);
    }
    unicodes
});

/// ## Merges
static MERGES: LazyLock<HashMap<Vec<u8>, usize>> = LazyLock::new(|| {
    let mut encoder = HashMap::new();
    let file = std::fs::File::open("src/bpe/merges.txt")
        .expect("[ERROR]: Could not load merges");
    let file = std::io::BufReader::new(file);

    for (idx, line) in std::io::BufRead::lines(file).enumerate() {
        let _line = line.unwrap();
        encoder.insert(_line.as_bytes().to_vec(), 50000 - idx);
    };
    
    encoder
});

///  u8 byte vector to [unicode](crate::tokenizer::GPT_UNICODES) characters.
///
/// ## Grapheme
/// ### Arguments
/// * `slice` - byte vector
///
/// ### Returns
/// * Unicode characters.
pub fn grapheme(slice: &[u8]) -> Vec<Vec<u8>> {
    let to_unicode = |char: &str| -> Vec<Vec<u8>> {
        char
            .chars()
            .flat_map(|c| -> Vec<u8> { String::from(c).into_bytes() })
            .map(|c| -> Vec<u8> {
                match BYTES_TO_UNICODES.get(&(c as u16)) {
                    Some(ch) => ch.to_vec(),
                    None => panic!("[ERROR]: Unicode value for '{:?}' not found!", c),
                }
            })
            .collect()
    };
    let text = String::from_utf8(slice.to_vec()).unwrap();
    UnicodeSegmentation::graphemes(format!("{text}").as_str(), true)
        .flat_map(|char| -> Vec<Vec<u8>> { to_unicode(char) })
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

/// Takes a byte vector and returns a 2 [window](std::slice::Windows) byte pairing of the vector.
/// ## To pairs
/// ### Arguments
/// * `grapheme` - grapheme byte vector
///
/// ### Returns
/// * a 2 [window](std::slice::Windows) byte paring.
fn to_pairs(grapheme: &Grapheme<u8>) -> Vec<BytePair<u8>> {
    grapheme
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
    };

    grapheme
}

/// Responsible for encoding and decoding text using the Byte Pair Encoding method, commonly used for tokenization.
struct BytePairEncoder<'a, D> {
    ///
    /// ## Vocabulary
    vocabulary: &'a LazyLock<BTreeMap<Vec<u8>, D>>,

    /// [GPT Unicode](crate::tokenizer::GPT_UNICODES) Representation of text in [extended grapheme clusters](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/).
    ///
    /// ## Grapheme
    grapheme: Grapheme<u8>,

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
    bytepairs: Vec<TokenPairing<u8>>,

    /// List of byte pairs that have been popped out of the `bytepairs` list on every iteration.
    ///
    /// This is to ensure that the value is not used again.
    ///
    /// ## Byte Pair cache.
    cache: HashSet<BytePair<u8>>,
}

impl<'a, D: std::clone::Clone> BytePairEncoder<'a, D>{
    pub fn new(
        grapheme: Grapheme<u8> ,
        vocabulary: &'a LazyLock<BTreeMap<Vec<u8>, D>>,
    ) -> BytePairEncoder<'a, D> where usize: From<D>{
        let mut encoder = BytePairEncoder {
            grapheme,
            tokens: vec![],
            bytepairs: vec![],
            cache: HashSet::new(),
            vocabulary,
        };

        encoder.tick();
        encoder
    }
    /// The tick part of a tick-tokenizer.
    /// The function completes the following steps for the byte pair encoder:
    ///
    /// 1. Splits grapheme to byte pairs.
    /// 2. Checks for new byte pairs.
    /// 3. Adds the new byte pairs into iterator list.
    /// 4. Sorts byte pairs.
    ///
    /// ## Tick
    fn tick(&mut self) where usize: From<D> {
        for pair in to_pairs(&self.grapheme) {
            if !self.cache.contains(&pair) {
              // check vocabulary.
                if let Some(rank) = self.vocabulary.get(&pair.concat()) {
                    self.bytepairs
                        .push((rank.clone().into(), [pair[0].clone(), pair[1].clone()]));
                    self.cache.insert(pair);
                    continue;
                };
                // check merges.
                if let Some(rank) = MERGES.get(&pair.concat()) {
                    self.bytepairs
                        .push((*rank, [pair[0].clone(), pair[1].clone()]));
                    self.cache.insert(pair);
                    continue;
                };
            };
        }
        self.bytepairs.sort_by(|a, b| b.0.cmp(&a.0));
    }
    /// Maps a byte pair vector into tuple with a byte vector and equivalent token vector
    ///
    /// ## Contraction
    /// ### Arguments
    /// * `byte paring` - byte pair vector
    ///
    /// ### Returns
    ///
    /// * byte vector and equivalent token vector
    fn contraction(&self, bytepairing: &Vec<BytePair<u8>>) -> Option<(Grapheme<u8>, Vec<D>)> {
        let grapheme = from_pairs(&bytepairing);
        let mut tokens = vec![];

        let is_tokenized = {
            for key in &grapheme {
                if let Some(value) = self.vocabulary.get(key) {
                    tokens.push(value.clone())
                };
            }
            tokens.len() == grapheme.len()
        };

        if is_tokenized {
            Some((grapheme, tokens.clone()))
        } else {
            None
        }
    }
}

/// For ergonomic reasons.
/// Opting to implement the byte pair merge function as AddAssign
impl <D: std::clone::Clone> std::ops::AddAssign<&BytePair<u8>> for BytePairEncoder<'_, D> {
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
                    if let Some((grapheme, tokens)) = self.contraction(&binding) {
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
                    if let Some((grapheme, tokens)) = self.contraction(&binding) {
                        self.grapheme = grapheme;
                        self.tokens = tokens;
                    };
                    break;
                };
            }
        }
    }
}

impl<D : std::clone::Clone> Iterator for BytePairEncoder<'_, D>  where usize: From<D>{
    type Item = Vec<D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.grapheme.len() == 1 || self.bytepairs.is_empty() {
            true => None,
            false => {
                if let Some((_, bytepair)) = self.bytepairs.pop() {
                    // See, ergonomic.
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

/// Encodes a given byte slice into a token vector.
/// ## Encode
///
/// ### Arguments
/// * `slice` - a byte vector.
/// * `lookup` - a lookup table with vocabulary scheme (slice to tokens).
///
/// ### Returns
/// * a [token](crate::tokenizer::tokens) vector equivalent of slice.
pub(crate) fn encode<D: std::clone::Clone>( slice: &[u8], lookup: &LazyLock<BTreeMap<Vec<u8>, D>>) -> Vec<D>  where usize: From<D>{
    tokens(slice)
    .iter()
    .map(|t| -> Grapheme<u8> {grapheme(*t)})
    .fold(vec![], |mut tokens: Vec<D>, grapheme| -> Vec<D> {
        let lexeme: Vec<D> = match lookup.get(&grapheme.concat()) {
            Some(t) => vec![t.clone()],
            None => {
                let encoder = BytePairEncoder::new(grapheme, lookup);
                encoder.into_iter().fold(vec![],|_enc, value| value)
            }
        };
        tokens.extend(lexeme);
        tokens
    })
}

/// Decodes a given token vector into a byte slice.
/// ## Decode
///
/// ### Arguments
/// * `tokens` - token vector.
/// * `lookup` - a lookup table with vocabulary scheme (tokens to slice).
///
/// ### Returns
/// * a byte slice.
pub(crate) fn decode<T : std::cmp::Ord + std::fmt::Debug>( tokens: &[T], vocabulary: &LazyLock<BTreeMap<T, Vec<u8>>>) -> Vec<u8> {
    tokens
    .iter()
    .fold(vec![],|mut slice: Vec<u8>, lexeme: &T| -> Vec<u8> {
        match vocabulary.get(lexeme) {
            Some(unicodes) => {
                let text = String::from_utf8(unicodes.to_vec()).unwrap();

                let bytes: Vec<u8> = UnicodeSegmentation::graphemes(text.as_str(), true)
                    .flat_map(|char| -> Vec<u8> {
                        match UNICODES_TO_BYTES.get(char.as_bytes()) {
                            Some(b) => vec![*b],
                            None => char.as_bytes().to_vec()
                        }
                    })
                    .collect();

                slice.extend(bytes);
            }
            None => {
                todo!();
                // Here is the thing. This is technically impossible.
                // So there should not be any code here.
                // Just look away.
                // print!("Me: What on earth are you doing lexeme!?\nLexeme: {:?}", lexeme);
            },
        };
        slice
    })
}
