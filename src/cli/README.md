# CLI Module

## Overview

This module contains all the logic for the command-line interface (CLI) of the `gpt3bpe` utility. It uses the `argh` crate to parse command-line arguments, flags, and subcommands. Its primary role is to interpret user input and dispatch the appropriate actions, such as encoding text to tokens, decoding tokens to text, or splitting text into graphemes.

All operations expect input to be piped via `stdin`.

## Usage Examples

### Encoding (Default Operation)

To encode a string into tokens, pipe it to the command. By default, it uses the `p50k` vocabulary.

```bash
# Encode using the default p50k vocabulary
echo "hello world" | gpt3bpe
```

You can specify a different vocabulary using the `-v` flag.

```bash
# Encode using the cl100k vocabulary
echo "hello world" | gpt3bpe -v cl100k
```

### Decoding

To decode a space-separated sequence of tokens, use the `-d` or `--decode` flag.

```bash
# Decode using the cl100k vocabulary
echo "9906 11 1917" | gpt3bpe -d -v cl100k
```

### Grapheme Splitting

To split a string into its underlying GPT-style Unicode graphemes, use the `grapheme` subcommand.

```bash
echo "hello 👋" | gpt3bpe grapheme
```

## Core Components

### `mod.rs`

This is the main file for the CLI module. It defines the entire command-line structure, including:
*   **`Arguments` struct**: Defines all possible arguments and flags (`--encode`, `--decode`, `--vocabulary`) using `argh`.
*   **`Command` enum**: Defines available subcommands (e.g., `grapheme`).
*   **`grapheme()` function**: Implements the logic for the `grapheme` subcommand.
*   **`decode()` function**: Implements the logic for the decoding operation.

The functions in this file are designed to be testable by accepting a generic `Write` trait, allowing output to be captured in tests instead of being printed directly to `stdout`.

### `unit.rs`

This file contains a comprehensive suite of unit tests for the CLI module. The tests are organized into sub-modules for clarity (`grapheme`, `decoder`, `arguments`) and verify the following:
*   Correctness of the `grapheme` and `decode` functions with various inputs.
*   Robustness against invalid or malformed input (e.g., non-numeric tokens).
*   Proper parsing of command-line arguments by `argh`.