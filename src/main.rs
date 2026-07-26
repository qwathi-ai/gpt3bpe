//! # GPT-BPE Command-Line Utility
//!
//! This binary provides a command-line interface (CLI) for performing Byte-Pair Encoding (BPE)
//! tokenization tasks. It reads text from standard input and can either encode it into tokens,
//! decode tokens back into text, or split it into its base grapheme clusters.
//!
//! The tool is designed to be used in a pipeline, for example:
//! `echo "hello world" | gpt3bpe`
mod bpe;
mod cli;
#[cfg(feature = "embeddings")]
mod embeddings;
// #[cfg(feature = "instruments")]
mod instruments;
#[cfg(feature = "neural")]
mod neural;
use std::io::{BufRead, Write, stdin, stdout};

/// The main entry point of the command-line utility.
///
/// This function parses command-line arguments, reads from stdin, and performs
/// the requested operation (grapheme splitting, encoding, or decoding).
///
/// It processes input line by line, allowing it to be used with piped data streams.
/// The default operation is encoding, but this can be changed with flags like `--decode`
/// or by using a subcommand like `grapheme`.
/// # Panics
///
/// This function will panic if:
/// * It fails to read a line from stdin.
/// * It fails to parse a token from a line during decoding.
/// * It fails to write the decoded bytes to stdout.
fn main() {
    let args: cli::Arguments = argh::from_env();
    let stdin = stdin();

    // Ensure that --encode and --decode flags are not used simultaneously.
    if args.encode && args.decode {
        eprintln!("[ERROR]: --encode and --decode are mutually exclusive.");
        std::process::exit(1);
    };

    // Process each line from standard input.
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from stdin");

        // // Handle the 'embed' subcommand if present.
        // if let Some(cli::Command::Embed(_)) = args.command {
        //     if !cfg!(feature = "embeddings") {
        //         println!("[WARNING]: `embed` command can only be used if the `embeddings` feature is enabled.");
        //         break;
        //     }
        //     let embedding = cli::embed::<{embeddings::DIMENSIONS}, 75, {embeddings::PADDING}>(line, &args);
        //     let mut writer = stdout().lock();
        //     write!(stdout().lock(), "{:?}", embedding).unwrap();
        //     writer.flush().unwrap();
        //     continue;
        // };
        
        // Handle the 'grapheme' subcommand if present.
        if let Some(cli::Command::Grapheme(_)) = args.command {
            let grapheme = cli::grapheme(line);
            stdout().write_all(grapheme.as_bytes()).unwrap();
            continue;
        };


        if args.decode {
            let bytes = cli::decode(line, &args);
            stdout().write_all(&bytes).unwrap();
            continue;
        };

        // The default operation is encoding.
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
        // Format the resulting tokens into a space-separated string.
        let output = tokens
            .iter()
            .flat_map(|t| -> Vec<String> { t.iter().map(|u| u.to_string()).collect() })
            .collect::<Vec<_>>()
            .join(" ");
        println!("{output}");
    }
}
