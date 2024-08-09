mod unit;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

const WORD_RE: &'static str =
    r"(?u)'s|'t|'re|'ve|'m|'l l|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(\S)|\s+";
const ENCODING: [u16; 188] = [
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

lazy_static! {
    #[derive(Debug)]
    static ref ENCODER: HashMap<u16, Vec<u8>> = {
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
            unicodes.insert(*c, decoded.into_bytes());
        };
        unicodes
    };

    #[derive(Debug)]
    static ref DECODER: HashMap<Vec<u8>, u16> = {
        let mut decoder = HashMap::new();
        for (key, value) in ENCODER.iter() {
            decoder.insert(value.to_owned(), key.to_owned());
        };
        decoder
    };
}

pub fn grapheme(slice: &[u8]) -> Result<Vec<Vec<u8>>, crate::error::Error> {
    let symbol_to_bytes = |symbol: &str| -> Vec<Vec<u8>> {
        symbol
            .chars()
            .flat_map(|c| -> Vec<u8> { 
                println!("{:?} -> {:?}", c, String::from(c).into_bytes());
                String::from(c).into_bytes() 
            })
            .map(|c| -> Vec<u8> {
                match crate::text::ENCODER.get(&(c as u16)) {
                    Some(ch) => { 
                        println!("{:?}", ch);
                        ch.to_vec()
                    },
                    None => panic!("[ERROR]: Encoding value for '{:?}' not found!", c),
                }
            })
            .collect()
    };

    let ngram = String::from_utf8(slice.to_vec()).unwrap();

    Ok(UnicodeSegmentation::graphemes(ngram.as_str(), true)
        .flat_map(|symbol| -> Vec<Vec<u8>> { symbol_to_bytes(symbol) })
        .collect())
}

pub fn ngram(grapheme: &Vec<Vec<u8>>) -> Result<Vec<u16>, crate::error::Error> {
    Ok(grapheme
        .iter()
        .map(|graph| -> u16 {
            match crate::text::DECODER.get(graph) {
                Some(ch) => *ch,
                None => panic!("[ERROR]: Decoding value for '{:?}' not found!", graph),
            }
        })
        .collect())
}

pub fn words(slice: &[u8]) -> Result<Vec<Vec<u8>>, crate::error::Error> {
    Ok(Regex::new(&crate::text::WORD_RE)
        .unwrap()
        .find_iter(slice)
        .map(|m| -> Vec<u8> { m.as_bytes().to_vec()})
        .collect())
}
