# GPT Byte-Pair Encoder

A Rust implementation of the Byte-Pair Encoding (BPE) algorithm used by OpenAI's Generative Pre-trained Transformers (GPT). This project provides both a command-line tool and a C-compatible library for encoding text into tokens and decoding tokens back into text.

It is inspired by Andrej Karpathy's [picoGPT](https://github.com/jaymody/picoGPT) project.

## Features

*   **High Performance**: Written in Rust for speed and memory safety.
*   **Multiple Vocabularies**: Supports `p50k_base`, `r50k_base`, `cl100k_base`, and `o200k_base` vocabularies.
*   **Streaming CLI**: The command-line tool processes input from `stdin`, making it easy to use in shell pipelines.
*   **Flexible Interface**: Can be used as a standalone CLI tool or as a shared library via a C-compatible Foreign Function Interface (FFI).

## Installation

### From source

To build the CLI tool, you'll need the Rust toolchain installed.

1.  Clone the repository:
    ```sh
    git clone https://github.com/qwathi-ai/gpt3bpe.git
    cd gpt3bpe
    ```

2.  Build the project:
    ```sh
    cargo build --release
    ```

3.  The executable will be located at `target/release/gpt3bpe`. You can copy it to a directory in your `PATH`, for example:
    ```sh
    cp target/release/gpt3bpe /usr/local/bin/
    ```

## Usage

### Command-Line Interface (CLI)

The CLI tool reads from standard input and writes to standard output.

#### **Encoding (Default)**

By default, the tool encodes the input string into token IDs.

```sh
# Encode with the default p50k vocabulary
echo "Hello, world!" | gpt3bpe
```
Output:
```
15496 11 290 0
```

You can specify a different vocabulary with the `-v` or `--vocabulary` flag.

```sh
# Encode with the cl100k vocabulary
echo "Hello, world!" | gpt3bpe -v cl100k
```
Output:
```
9906 11 1917 0
```

#### **Decoding**

Use the `-d` or `--decode` flag to decode a space-separated list of token IDs back into a string.

```sh
# Decode with the default p50k vocabulary
echo "15496 11 290 0" | gpt3bpe -d
```
Output:
```
Hello, world!
```

```sh
# Decode with the cl100k vocabulary
echo "9906 11 1917 0" | gpt3bpe -d -v cl100k
```
Output:
```
Hello, world!
```

#### **Grapheme Splitting**

The `grapheme` subcommand splits the input string into its base GPT Unicode graphemes.

```sh
echo "hello 👋" | gpt3bpe grapheme
```

#### **Help**

For a full list of commands and options, use the `--help` flag.

```sh
gpt3bpe --help
```

### Vocabularies

The tool supports several vocabularies used by different GPT models. The `-v` flag allows you to choose one.

*   **`p50k`** (default): `p50k_base`. For models like `text-davinci-002`. Encodes to `u16` values.
*   **`r50k`**: `r50k_base` (or `gpt2`). For older models like `text-davinci-001`. Encodes to `u16` values.
*   **`cl100k`**: `cl100k_base`. For models like `gpt-3.5-turbo` and `gpt-4`. Encodes to `u32` values.
*   **`o200k`**: `o200k_base`. For models like `gpt-4o`. Encodes to `u32` values.

### Foreign Function Interface (FFI)

The project can be built as a dynamic library to be used in other languages that support C ABIs.

#### **Building the library**

Update your `Cargo.toml` to build a dynamic library:
```toml
[lib]
crate-type = ["cdylib"]
```

Then build it:
```sh
cargo build --release
```
This will produce a shared library (e.g., `libgpt3bpe.so` on Linux, `libgpt3bpe.dylib` on macOS, `gpt3bpe.dll` on Windows) in the `target/release` directory.

#### **Available Functions**

The library exposes the following functions:

```c
// Splits a string into GPT unicode grapheme characters.
void grapheme(const uint8_t* buffer, size_t length, void (*callback)(size_t, uint8_t));

// r50k vocabulary
void encode_r50k(const uint8_t* buffer, size_t length, void (*callback)(size_t, uint32_t));
void decode_r50k(const uint16_t* buffer, size_t length, void (*callback)(size_t, uint8_t));

// p50k vocabulary
void encode_p50k(const uint8_t* buffer, size_t length, void (*callback)(size_t, uint32_t));
void decode_p50k(const uint16_t* buffer, size_t length, void (*callback)(size_t, uint8_t));

// cl100k vocabulary
void encode_cl100k(const uint8_t* buffer, size_t length, void (*callback)(size_t, uint32_t));
void decode_cl100k(const uint32_t* buffer, size_t length, void (*callback)(size_t, uint8_t));

// o200k vocabulary
void encode_o200k(const uint8_t* buffer, size_t length, void (*callback)(size_t, uint32_t));
void decode_o200k(const uint32_t* buffer, size_t length, void (*callback)(size_t, uint8_t));
```

#### **C Usage Example**

To compile and run C code that uses the library, you would link against the `.so`/`.dylib`/`.dll` file. For example, on Linux:

```sh
# Compile the C code
gcc -o example example.c -L./target/release -lgpt3bpe

# Set library path and run
LD_LIBRARY_PATH=./target/release ./example
```

### Note on `merges.txt`

The current implementation loads BPE merges from `src/bpe/merges.txt` relative to the current working directory at runtime. Please ensure this file is present at the expected location when running the compiled binary.

For a more robust deployment, consider embedding the file directly into the binary using `include_str!` or `include_bytes!` in `src/bpe/mod.rs`.