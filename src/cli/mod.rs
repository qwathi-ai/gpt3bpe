//! Handles command-line interface (CLI) logic for the application.
//!
//! This module uses the `argh` crate to parse command-line arguments and
//! defines the available commands, subcommands, and flags. It contains the
//! primary logic for dispatching to the correct encoding, decoding, or
//! grapheme-splitting functions based on user input.
pub(crate) mod unit;
use std::io;
use std::io::{ Write };
use argh::FromArgs;
use crate::bpe;

/// Subcommand for splitting a string into GPT Unicode graphemes.
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "grapheme")]
pub (crate) struct GraphemeCommand {}

/// An enumeration of all available subcommands.
#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub (crate) enum Command {
    Grapheme(GraphemeCommand),
}

/// Defines the command-line arguments for the GPT Byte-Pair-Encoder utility.
#[derive(FromArgs, Debug)]
/// A command-line utility for the GPT Byte-Pair-Encoder.
///
/// This tool provides three main functions:
///
///   - encode: Encodes a string into tokens using a specified vocabulary (default).
///   - decode: Decodes a sequence of tokens back into a string.
///   - grapheme: Splits a string into GPT unicode grapheme characters.
///
/// Input should be piped to the command via stdin.
/// For example: 
/// * `echo "hello world" | gpt3bpe` (encodes with p50k)
/// * `echo "31373 995" | gpt3bpe -d -v r50k` (decodes with r50k)
pub struct Arguments {
    /// Use the encode operation (this is the default behavior).
    #[argh(
        switch,
        short = 'e',
        long = "encode",
        description = "use encode operation (default)."
    )]
    pub encode: bool,
    
    /// Use the decode operation instead of the default encode.
    #[argh(
        switch,
        short = 'd',
        long = "decode",
        description = "use decode operation."
    )]
    pub decode: bool,

    /// The vocabulary to use for encoding or decoding.
    #[argh(
        option,
        short = 'v',
        description = "vocabulary to use (r50k, p50k, cl100k, o200k) [default: p50k]",
        default = "bpe::vocabulary::Vocabularies::default()"
    )]
    pub vocabulary: bpe::vocabulary::Vocabularies,

    /// an optional subcommand to execute (e.g., `grapheme`).
    #[argh(subcommand)]
    pub (crate) command: Option<Command>,
}

/// Splits an input string into its constituent GPT-style graphemes and writes them to a writer.
///
/// Graphemes are space-separated in the output.
///
/// # Arguments
///
/// * `line` - The input `String` to be split.
/// * `writer` - An object implementing `io::Write` to which the output is written.
///
/// # Returns
///
/// An `io::Result<()>` indicating the success or failure of the write operation.
pub fn grapheme(line: String, mut writer: impl Write) -> io::Result<()> {
    let graphemes = bpe::grapheme(line.trim().as_bytes());
    let output = graphemes
    .iter()
    .map(|g| String::from_utf8_lossy(g))
    .collect::<Vec<_>>()
    .join(" ");
    writeln!(writer, "{output}")
}

/// Decodes a space-separated string of token IDs into text and writes it to a writer.
///
/// The function selects the appropriate decoding map based on the vocabulary
/// specified in the `Arguments`. It gracefully handles empty input and ignores
/// any non-numeric parts of the input string.
///
/// # Arguments
///
/// * `line` - The input `String` of space-separated token IDs.
/// * `args` - A reference to the parsed `Arguments`, used to determine the vocabulary.
/// * `writer` - An object implementing `io::Write` to which the decoded output is written.
///
/// # Returns
///
/// An `io::Result<()>` indicating the success or failure of the write operation.
pub fn decode(line: String, args: &Arguments, mut writer: impl Write) -> io::Result<()> {
    if line.trim().is_empty() {
        return writeln!(writer);
    };

    let decoded_bytes = match args.vocabulary {
        bpe::vocabulary::Vocabularies::R50K => {
            let tokens: Vec<u16> = line.trim()
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            bpe::decode(&tokens, &bpe::vocabulary::R50K_UNICODES)
        }
        bpe::vocabulary::Vocabularies::P50K => {
            let tokens: Vec<u16> = line.trim()
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            bpe::decode(&tokens, &bpe::vocabulary::P50K_UNICODES)
        }
        bpe::vocabulary::Vocabularies::CL100K => {
            let tokens: Vec<u32> = line.trim()
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            bpe::decode(&tokens, &bpe::vocabulary::CL100K_UNICODES)
        }
        bpe::vocabulary::Vocabularies::O200K => {
            let tokens: Vec<u32> = line.trim()
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            bpe::decode(&tokens, &bpe::vocabulary::O200K_UNICODES)
        }
    };

    writer.write_all(&decoded_bytes)?;
    writeln!(writer)
}
