# GPT-3 Byte Pair Encoder (BPE)

A command-line utility for encoding text using GPT-3's Byte Pair Encoding (BPE) algorithm.

## Features
- Efficiently encodes text using GPT-3's BPE.
- Simple command-line interface.
- Can process input from files, standard input, or direct text.

## Installation

You can install the encoder using Rust's package manager, Cargo:

```sh
cargo install gpt3bpe
```

## Usage

You can use the `gpt3bpe` command to encode text. 

### Encoding with Piped Input

You can also pipe input directly:

```sh
cat README.md | gpt3bpe >> test.txt
```

This will encode the contents of `README.md` and append the result to `test.txt`.

### Encoding Direct Input

You can also pass text directly:

```sh
echo "Hello, world!" | gpt3bpe
```