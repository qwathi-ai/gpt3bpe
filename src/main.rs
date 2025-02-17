mod error;

use argh::FromArgs;
use std::fs::File;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::path::Path;

/// Reach new heights.
#[derive(Debug, FromArgs)]
struct Encode {
    /// chunks
    #[argh(option, short = 'f')]
    file: Option<String>,
}

fn main() {
    let arguments: Encode = argh::from_env();
    for line in stdin().lines() {
        let data = line.unwrap();
        let e = gpt3bpe::encode(data.as_bytes()).unwrap();
        println!("[INFO][ENCODE]: {:?}", e);
    }
}
