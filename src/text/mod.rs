mod unit;
use lazy_static::lazy_static;
use std::{collections::HashMap, io::Read};
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    /// I could not possibly add anything thing to this comment, other than bring it to life.
    ///
    /// Forked from https://github.com/qwathi-ai/gpt-2/blob/master/src/encoder.py
    ///
    /// > Returns list of utf-8 byte and a corresponding list of unicode strings.
    /// > The reversible bpe codes work on unicode strings.
    /// > This means you need a large # of unicode characters in your vocab if you want to avoid UNKs.
    /// > When you're at something like a 10B token dataset you end up needing around 5K for decent coverage.
    /// > This is a signficant percentage of your normal, say, 32K bpe vocab.
    /// > To avoid that, we want lookup tables between utf-8 bytes and unicode strings.
    /// > And avoids mapping to whitespace/control characters the bpe code barfs on.
    #[derive(Debug)]
    static ref ENCODING: [u16;188] = {
        [
            33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
            55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76,
            77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98,
            99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115,
            116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 161, 162, 163, 164, 165, 166,
            167, 168, 169, 170, 171, 172, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184,
            185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201,
            202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218,
            219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235,
            236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252,
            253, 254, 255,
        ]
    };
    /// Creates a person with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    #[derive(Debug)]
    static ref ENCODER: HashMap<u16, String> = {
        let mut x = ENCODING.to_vec();
        let mut y: Vec<u16> = x.clone();
        let mut n: u16 = 0;
        for i in 0..=256 {
            if !x.contains(&i) {
                x.push(i);
                y.push(256 + n);
                n += 1;
            };
        };
        let mut unicodes = HashMap::new();
        for (i, c) in x.iter().enumerate() {
            let decoded = String::from_utf16(&[y[i]]).unwrap();
            unicodes.insert(*c, decoded.to_owned());
        };
        unicodes
    };
    /// Creates a person with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    #[derive(Debug)]
    static ref DECODER: HashMap<String, u16> = {
        let mut decoder = HashMap::new();
        for (key, value) in ENCODER.iter() {
            decoder.insert(value.to_owned(), key.to_owned());
        };
        decoder
    };

    #[derive(Debug)]
    static ref WORD_RE: &'static str = r"(?u)'s|'t|'re|'ve|'m|'l l|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(\S)|\s+";
}

pub fn read_bytes(mut bytes: &[u8]) -> Result<String, crate::error::ERROR> {
    let mut buffer = String::new();
    bytes.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn grapheme(ngram: &str) -> Result<Vec<String>, crate::error::ERROR> {
    let symbol_to_chars = |symbol: &str| -> Vec<String> {
        symbol
            .chars()
            .flat_map(|c| -> Vec<u8> { String::from(c).into_bytes() })
            .map(|c| -> String {
                match ENCODER.get(&(c as u16)) {
                    Some(ch) => ch.to_string(),
                    None => panic!("[ERROR]: Encoding value for '{:?}' not found!", c),
                }
            })
            .collect::<Vec<String>>()
    };

    Ok(UnicodeSegmentation::graphemes(ngram, true)
        .flat_map(|symbol| -> Vec<String> { symbol_to_chars(symbol) })
        .collect::<Vec<String>>())
}

pub fn ngram(grapheme: &Vec<&str>) -> Result<String, crate::error::ERROR> {
    let bytes = grapheme
        .iter()
        .map(|token| -> u16 {
            match DECODER.get(*token) {
                Some(ch) => *ch,
                None => panic!("[ERROR]: Encoding value for '{:?}' not found!", token),
            }
        })
        .collect::<Vec<u16>>();

    Ok(String::from_utf16(&bytes)?)
}
