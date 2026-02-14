mod bpe;
// use argh::FromArgs;
// use std::io::stdin;

/// Encoder command line instructions
// #[derive(Debug, FromArgs)]

fn main() {
    // let _arguments: Encode = argh::from_env();
    let encode = bpe::encode(b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D.", &crate::bpe::vocabulary::R50K_TOKENS);
    // let encode = bpe::encode(b"indivisible", &crate::bpe::vocabulary::R50K_TOKENS);
    
    println!("Encoding {:?}", encode);

    // for line in stdin().lines() {
    //     let data = line.unwrap();
    //     // println!("[INFO][ENCODE]: {:?} -> {:?}", data, e);
    // }
}
// rust_tokenizer_bytes_fix.rs
// Demonstrates the correct byte-level mapping used by GPT-style tokenizers
// so that Unicode characters (including emoji) survive round-trip encoding.
//
// Key idea: map each raw UTF-8 byte (0..=255) to a unique Unicode string
// symbol (often printable). BPE merges operate over these symbols.

// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use std::collections::HashMap;
// use serde_json::Value;

// pub fn load_vocab(path: &str) -> HashMap<String, usize> {
//     let file = File::open(path).expect("Could not open vocab.json");
//     let json: Value = serde_json::from_reader(file).expect("Invalid vocab.json");
//     json.as_object()
//         .unwrap()
//         .iter()
//         .map(|(k, v)| (k.clone(), v.as_u64().unwrap() as usize))
//         .collect()
// }

// pub fn load_merges(path: &str) -> Vec<(String, String)> {
//     let file = File::open(path).expect("Could not open merges.txt");
//     let reader = BufReader::new(file);
//     let mut merges = Vec::new();

//     for line in reader.lines() {
//         let line = line.expect("Error reading merges.txt");
//         if line.starts_with("#") || line.trim().is_empty() {
//             continue;
//         }
//         let parts: Vec<&str> = line.split_whitespace().collect();
//         if parts.len() == 2 {
//             merges.push((parts[0].to_string(), parts[1].to_string()));
//         }
//     }
//     merges
// }
