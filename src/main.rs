#![feature(portable_simd)]
mod bpe;
mod cli;
#[cfg(feature = "embeddings")]
mod embeddings;
mod instruments;
use std::{io::{self, BufRead}, str::FromStr};

impl FromStr for bpe::vocabulary::Vocabularies {
    type Err = String;

    /// Parses a string into a `Vocab` enum.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse. Can be "r50k", "p50k", or "cl100k".
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid vocabulary identifier.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r50k" => Ok(bpe::vocabulary::Vocabularies::R50K),
            "p50k" => Ok(bpe::vocabulary::Vocabularies::P50K),
            "cl100k" => Ok(bpe::vocabulary::Vocabularies::CL100K),
            "o200k" => Ok(bpe::vocabulary::Vocabularies::O200K),
            _ => Err(format!(
                "unknown vocabulary: {s}. Please use one of: r50k, p50k, cl100k, o200k"
            )),
        }
    }
}

/// The main entry point of the command-line utility.
///
/// This function parses command-line arguments, reads from stdin, and performs
/// the requested operation (grapheme splitting, encoding, or decoding).
///
/// # Panics
///
/// This function will panic if:
/// * It fails to read a line from stdin.
/// * It fails to parse a token from a line during decoding.
/// * It fails to write the decoded bytes to stdout.
fn main() {
    let args: cli::Arguments = argh::from_env();
    let stdin = io::stdin();

    if args.encode && args.decode {
        eprintln!("[ERROR]: --encode and --decode are mutually exclusive.");
        std::process::exit(1);
    };

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from stdin");

        if let Some(cli::Command::Grapheme(_)) = args.command {
            let _ = cli::grapheme(line, std::io::stdout());
            continue;
        };

        if args.decode {
            let _ = cli::decode(line, &args, std::io::stdout());
            continue;
        };

        let tokens = match args.vocabulary {
            bpe::vocabulary::Vocabularies::R50K => {
                bpe::encode(line.as_bytes(), &bpe::vocabulary::R50K_TOKENS)
            },
            bpe::vocabulary::Vocabularies::P50K => {
                bpe::encode(line.as_bytes(), &bpe::vocabulary::P50K_TOKENS)
            },
            bpe::vocabulary::Vocabularies::CL100K => {
                bpe::encode(line.as_bytes(), &bpe::vocabulary::CL100K_TOKENS)
            },
            bpe::vocabulary::Vocabularies::O200K => {
                bpe::encode(line.as_bytes(), &bpe::vocabulary::O200K_TOKENS)
            }
        };
        let output = tokens
            .iter()
            .flat_map(|t| -> Vec<String> { t.iter().map(|u| u.to_string()).collect() })
            .collect::<Vec<_>>()
            .join(" ");
        println!("{output}");
    }
}
