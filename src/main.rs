mod error;

use argh::FromArgs;
use std::io::stdin;

/// Encoder command line instructions
#[derive(Debug, FromArgs)]
struct Encode {}

fn main() {
    let _arguments: Encode = argh::from_env();

    for line in stdin().lines() {
        let data = line.unwrap();
        let e = gpt3bpe::encode(data.as_bytes()).unwrap();
        println!("[INFO][ENCODE]: {:?}", e);
    }
}
