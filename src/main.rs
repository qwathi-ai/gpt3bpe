mod bpe;
use argh::FromArgs;
// use std::io::stdin;

/// Encoder command line instructions
#[derive(Debug, FromArgs)]
#[argh(description = "Encode a string using the BPE algorithm.")]
struct Encode {
    #[argh(positional, description = "the string to encode")]
    input: String
}



fn main() {
    let _arguments: Encode = argh::from_env();
    // let encode = bpe::encode(b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D.", &crate::bpe::vocabulary::R50K_TOKENS);
    // let encode = bpe::encode(b"indivisible", &crate::bpe::vocabulary::R50K_TOKENS);
    
    println!("Encoding {:?}", _arguments);

    // for line in stdin().lines() {
    //     let data = line.unwrap();
    //     // println!("[INFO][ENCODE]: {:?} -> {:?}", data, e);
    // }
}