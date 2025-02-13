mod error;

use argh::FromArgs;
use std::io::{stdin, Read};

/// Reach new heights.
#[derive(Debug, FromArgs)]
struct Encode {
    // /// chunks
    // #[argh(option, short = 'c')]
    // chunks: u8,
}

fn main() {
    let arguments: Encode = argh::from_env();
    for line in stdin().lines() {
        let data = line.unwrap();
        #[cfg(debug_assertions)]
        println!("[DEBUG][INPUT]: {:?}", data);

        let e = gpt3bpe::encode(data.as_bytes()).unwrap();
        println!("[INFO][ENCODE]: {:?}", e);
    }
}
