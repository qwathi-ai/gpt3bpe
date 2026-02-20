mod bpe;
use argh::FromArgs;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

#[derive(FromArgs, Debug)]
/// A command-line utility for the GPT Byte-Pair-Encoder.
///
/// This tool provides three main functions:
///
///   - grapheme: Splits a string into GPT unicode grapheme characters.
///   - encode: Encodes a string into tokens using a specified vocabulary (default).
///   - decode: Decodes a sequence of tokens back into a string.
///
/// Input should be piped to the command via stdin.
/// For example: `echo "hello world" | gpt3bpe` (encodes with p50k)
///              `echo "31373 995" | gpt3bpe -d -v r50k` (decodes with r50k)
struct GptBpeArgs {
    #[argh(subcommand)]
    command: Option<Command>,

    #[argh(
        switch,
        short = 'd',
        long = "decode",
        description = "use decode operation."
    )]
    decode: bool,

    #[argh(
        switch,
        short = 'e',
        long = "encode",
        description = "use encode operation (default)."
    )]
    encode: bool,

    #[argh(
        option,
        short = 'v',
        description = "vocabulary to use (r50k, p50k, cl100k, o200k) [default: p50k]",
        default = "Vocab::default()"
    )]
    vocabulary: Vocab,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum Command {
    Grapheme(GraphemeCommand),
}

#[derive(FromArgs, Debug)]
/// Splits a string into GPT unicode grapheme characters.
#[argh(subcommand, name = "grapheme")]
struct GraphemeCommand {}

#[derive(Debug, PartialEq, Eq, Default)]
enum Vocab {
    R50k,
    #[default]
    P50k,
    Cl100k,
    O200k,
}

impl FromStr for Vocab {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r50k" => Ok(Vocab::R50k),
            "p50k" => Ok(Vocab::P50k),
            "cl100k" => Ok(Vocab::Cl100k),
            "o200k" => Ok(Vocab::O200k),
            _ => Err(format!(
                "unknown vocabulary: {s}. Please use one of: r50k, p50k, cl100k, o200k"
            )),
        }
    }
}

fn main() {
    let args: GptBpeArgs = argh::from_env();
    let stdin = io::stdin();

    if args.encode && args.decode {
        eprintln!("Error: --encode and --decode are mutually exclusive.");
        std::process::exit(1);
    }

    match args.command {
        Some(Command::Grapheme(_)) => {
            for line in stdin.lock().lines() {
                let line = line.expect("Could not read line from stdin");
                let graphemes = bpe::grapheme(line.as_bytes());
                let output = graphemes
                    .iter()
                    .map(|g| String::from_utf8_lossy(g))
                    .collect::<Vec<_>>()
                    .join(" ");
                println!("{output}");
            }
        }
        None => {
            if args.decode {
                // Decode
                for line in stdin.lock().lines() {
                    let line = line.expect("Could not read line from stdin");
                    if line.trim().is_empty() {
                        println!();
                        continue;
                    }

                    let decoded_bytes = match args.vocabulary {
                        Vocab::R50k => {
                            let tokens: Vec<u16> = line
                                .split_whitespace()
                                .map(|s| s.parse().expect("Invalid u16 token"))
                                .collect();
                            bpe::decode(&tokens, &bpe::vocabulary::R50K_UNICODES)
                        }
                        Vocab::P50k => {
                            let tokens: Vec<u16> = line
                                .split_whitespace()
                                .map(|s| s.parse().expect("Invalid u16 token"))
                                .collect();
                            bpe::decode(&tokens, &bpe::vocabulary::P50K_UNICODES)
                        }
                        Vocab::Cl100k => {
                            let tokens: Vec<u32> = line
                                .split_whitespace()
                                .map(|s| s.parse().expect("Invalid u32 token"))
                                .collect();
                            bpe::decode(&tokens, &bpe::vocabulary::CL100K_UNICODES)
                        }
                        Vocab::O200k => {
                            let tokens: Vec<u32> = line
                                .split_whitespace()
                                .map(|s| s.parse().expect("Invalid u32 token"))
                                .collect();
                            bpe::decode(&tokens, &bpe::vocabulary::O200K_UNICODES)
                        }
                    };
                    io::stdout()
                        .write_all(&decoded_bytes)
                        .expect("Failed to write to stdout");
                    println!();
                }
            } else {
                // Encode
                for line in stdin.lock().lines() {
                    let line = line.expect("Could not read line from stdin");
                    let tokens = match args.vocabulary {
                        Vocab::R50k => bpe::encode(line.as_bytes(), &bpe::vocabulary::R50K_TOKENS),
                        Vocab::P50k => bpe::encode(line.as_bytes(), &bpe::vocabulary::P50K_TOKENS),
                        Vocab::Cl100k => {
                            bpe::encode(line.as_bytes(), &bpe::vocabulary::CL100K_TOKENS)
                        }
                        Vocab::O200k => {
                            bpe::encode(line.as_bytes(), &bpe::vocabulary::O200K_TOKENS)
                        }
                    };
                    let output = tokens
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join(" ");
                    println!("{output}");
                }
            }
        }
    }
}
