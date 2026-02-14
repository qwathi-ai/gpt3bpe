mod unit;
pub(crate) mod vocabulary;

use regex::bytes::Regex;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::LazyLock;
use unicode_segmentation::UnicodeSegmentation;
use std::io::BufReader;
use std::io::BufRead;
///! Module inspired by [PicoGPT](https://github.com/jaymody/picoGPT) project.
///

/// Data structure for byte pairings of type `[T]`.
///
/// ## Byte Pair
type BytePair<Type> = (usize, Type);

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
    let mut merges = HashMap::new();
    let file = std::fs::File::open("src/bpe/merges.txt").expect("[ERROR]: Could not open merges.txt. file");
    let reader = BufReader::new(file);
    for (idx, _line) in reader.lines().enumerate() {
        let line = _line.unwrap();
        if line.starts_with("#") || line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            merges.insert(line.as_bytes().to_vec(), 50000-idx);
            // merges.push((parts[0].to_string(), parts[1].to_string()));
        }
    }
    merges

});

///  u8 byte vector to [unicode](crate::tokenizer::GPT_UNICODES) characters.
///
/// ## Grapheme
/// ### Arguments
/// * `slice` - byte vector
///
/// ### Returns
/// * Unicode characters.
pub fn grapheme(slice: &[u8]) -> Grapheme<u8> {
    let to_unicode = |char: &str| -> Vec<Vec<u8>> {
        char.chars()
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

/// Responsible for encoding and decoding text using the Byte Pair Encoding method, commonly used for tokenization.
struct BytePairEncoder<'a> {
    ///
    /// ## Slice
    slice: &'a[u8],

    ///
    /// ## Pairs
    pairs: Vec<BytePair<usize>>,

    ///
    /// ## Encoder
    encoder: BTreeMap<Vec<u8>, usize>
}

impl<'a> BytePairEncoder<'a> {
    pub fn new(
        slice: &'a [u8],
        vocabulary: &LazyLock<BTreeMap<Vec<u8>, usize>>,
    ) -> BytePairEncoder<'a> {
        let mut encoder = std::collections::BTreeMap::new();
        // let mut decoder = std::collections::BTreeMap::new();

        for (key, value) in vocabulary.iter() {
            encoder.insert(key.to_vec(), value.clone());
            // decoder.insert(*value, key.to_vec());
        }

        for (key, value) in MERGES.iter() {
            encoder.insert(key.to_vec(), *value);
        }

        let mut pairs: Vec<BytePair<usize>> =
            (0..slice.len() + 1).map(|i| (i, usize::MAX)).collect();

        for i in 0..pairs.len() - 2 {
            if let Some(rank) = encoder
                .get(&slice[pairs[i].0..pairs[i + 2].0]) {
                    pairs[i].1 = *rank;
            }
        }

        println!("Pairs:   {:?}", pairs);

        BytePairEncoder {
            slice,
            pairs,
            encoder
        }
    }

    fn get_rank(
        &self,
        // slice: &[u8],
        // pairs: &Vec<BytePair<usize>>,
        start_idx: usize,
        skip: usize,
    ) -> Option<usize> {
        if (start_idx + skip + 2) < self.pairs.len() {
            self.encoder
                .get(&self.slice[self.pairs[start_idx].0..self.pairs[start_idx + skip + 2].0])
                .map(|r| *r)
        } else {
            None
        }
    }

    fn encode(&self, pairs: Vec<BytePair<usize>>, encoder: &LazyLock<BTreeMap<Vec<u8>, usize>>) -> Vec<usize> {
        let mut grapheme = Vec::with_capacity(pairs.len() - 1);
        println!("slice: {:?}\n", String::from_utf8(self.slice.to_vec().clone()));
        println!("slice: {:?}\n", &self.slice);
        println!("Grapheme before: {:?}\n", grapheme);


        for i in 0..pairs.len() - 1 {
            println!(
                "{:?} From pairs: {:?}",
                i,
                &self.slice[self.pairs[i].0..self.pairs[i + 1].0]
            );
            if let Some(encoding) = encoder.get(&self.slice[self.pairs[i].0..self.pairs[i + 1].0]) {
                grapheme.push(*encoding);
            }
            println!("Grapheme: {:?}\n", grapheme);
        }
        grapheme
    }
}

impl Iterator for BytePairEncoder<'_> {
    type Item = Vec<BytePair<usize>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pairs.len() == 1 {
            return None;
        }

        let mut rank: (usize, usize) = (usize::MAX, 0);
        for (idx, &(_, r)) in self.pairs[..self.pairs.len() - 1].iter().enumerate() {
            if r < rank.0 {
                rank = (r, idx);
            }
        }
        if rank.0 == usize::MAX {
            return None;
        }

        self.pairs[rank.1].1 =
            self.get_rank(rank.1, 1)
                .unwrap_or(usize::MAX);
        if rank.1 > 0 {
            self.pairs[rank.1 - 1].1 =
                self.get_rank(rank.1 - 1, 1)
                    .unwrap_or(usize::MAX);
        };
        self.pairs.remove(rank.1 + 1);
        // Some(self.from_pairs())
        Some(self.pairs.clone())
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
pub (crate) fn encode(slice: &[u8], lookup: &LazyLock<BTreeMap<Vec<u8>, usize>>) -> Vec<usize> {
    let mut result = vec![];

    for piece in tokens(slice) {
       let graph =  grapheme(piece).concat();
       println!("Graph:    {:?}", String::from_utf8(graph.to_vec()).unwrap());
        if let Some(token) = lookup.get(&graph) {
            result.push(*token);
            continue;
        }

        let mut encoder = BytePairEncoder::new( &graph, lookup);
        let mut merge = vec![];
        while let Some(m) = encoder.next() {
            println!("Merge: {:?}", m);
            merge = m;
        };
        result.extend(encoder.encode(merge, lookup))

    };
    result
}
