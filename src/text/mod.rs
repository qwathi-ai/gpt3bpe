mod unit;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    #[derive(Debug)]
    static ref ENCODER: HashMap<u16, String> = {
        let mut x = vec![
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
        ];

        let mut y: Vec<u16> = x.clone();
        let mut n: u8 = 0;

        for i in 0..=256 {
            if !x.contains(&i) {
                x.push(i);
                y.push(256 + Into::<u16>::into(n));
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
    #[derive(Debug)]
    static ref DECODER: HashMap<String, u16> = {
        let mut decoder = HashMap::new();
        for (key, value) in ENCODER.iter() {
            decoder.insert(value.to_owned(), key.to_owned());
        }
        decoder
    };
}

const WORD_RE: &str =
    r"(?u)'s|'t|'re|'ve|'m|'l l|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(\S)|\s+";

fn bytes_to_unicode(c: u8) -> String {
    match ENCODER.get(&(c as u16)) {
        Some(ch) => ch.to_string(),
        None => panic!("ERROR: Encoding value for {:?} not found!", c),
    }
}

fn unicode_to_bytes(c: &str) -> u16 {
    match DECODER.get(c) {
        Some(ch) => ch.to_owned(),
        None => panic!("ERROR: Decoding value for {:?} not found!", c),
    }
}

fn encode(token: &str) -> Vec<String> {
    UnicodeSegmentation::graphemes(token, true)
        .flat_map(|symbol| {
            symbol
                .chars()
                .flat_map(|c| String::from(c).into_bytes())
                .map(|c| bytes_to_unicode(c).to_owned())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>()
}

pub fn grapheme(text: &str) -> Vec<String> {
    Regex::new(WORD_RE)
        .unwrap()
        .find_iter(text)
        .flat_map(|m| encode(m.as_str()))
        .collect()
}

pub fn write(graphemes: &Vec<String>) -> String {
    graphemes
        .iter()
        .map(|grapheme| {
            UnicodeSegmentation::graphemes(grapheme.as_str(), false)
                .map(|graph| unicode_to_bytes(graph))
                .collect::<Vec<u16>>()
        })
        .flat_map(|bytes| String::from_utf16(&bytes))
        .collect::<String>()
}

pub fn words(text: &str) -> Vec<&str> {
    Regex::new(WORD_RE)
        .unwrap()
        .find_iter(text)
        .map(|m| m.as_str())
        .collect()
}
