# GPT Byte-Pair Encoding Module

## Overview
This document describes the implementation of a GPT Byte-Pair Encoder in Rust. The library provides tokenization for various GPT models by implementing their specific byte-pair encoding schemes. It relies on a set of static mappings and vocabulary files for efficient encoding and decoding.

## Core Components

### Static Mappings
The implementation utilizes several `LazyLock` static variables for performance and to ensure that resources are initialized only once.

#### `TOKENS_RE`
A regex pattern for the initial splitting of text into processable chunks. This pattern is based on the GPT-2/3 tokenization strategy, which handles contractions, different character types (letters, numbers), and whitespace.

#### `GPT_UNICODES`, `UNICODE_TO_BYTES`, and `BYTES_TO_UNICODE`
These mappings handle the conversion between raw bytes and a "safe" set of Unicode characters. This is a crucial step to avoid issues with control characters and complex whitespace during the BPE process.
*   `GPT_UNICODES`: A predefined array of Unicode characters.
*   `UNICODE_TO_BYTES`: Maps a byte value (0-255) to the UTF-8 bytes of its corresponding "safe" Unicode character.
*   `BYTES_TO_UNICODE`: Provides the inverse mapping from a "safe" Unicode character's bytes back to the original byte value.

#### `MERGES`
A `HashMap<Vec<u8>, u32>` that provides the BPE merge ranks. This map is constructed at compile time by embedding the `src/bpe/merges.txt` file directly into the binary using `include_str!`. This eliminates the need for a separate `merges.txt` file at runtime. The `BytePairEncoder` uses these ranks to iteratively merge subword units.

### Vocabulary Files
The tokenizer supports different GPT models by loading their respective vocabularies from `.jsonl` files at compile time (via `include_str!` in the `vocabulary` module). These are loaded into two types of maps:

- **`*_TOKENS`**: A `BTreeMap<Vec<u8>, TokenID>` that maps a token string (as bytes) to its unique token ID (e.g., `CL100K_TOKENS`).
- **`*_UNICODES`**: A `BTreeMap<TokenID, Vec<u16>>` that maps a token ID back to a sequence of Unicode codepoints, used for decoding (e.g., `CL100K_UNICODES`).

Supported vocabularies include `r50k_base`, `p50k_base`, `cl100k_base`, and `o200k_base`.

## Graphemes
A **grapheme** is the smallest unit of a writing system. The library uses grapheme segmentation from the `unicode-segmentation` crate to correctly handle multi-byte characters and composite characters (like "é") during the encoding and decoding process, ensuring text integrity.

## Encoding Process
1.  Input text (as bytes) is split into chunks using the `TOKENS_RE` regex.
2.  For each chunk, it is first checked if the entire chunk exists as a single token in the `*_TOKENS` vocabulary.
3.  If not found, the chunk is broken down into graphemes, and each grapheme is converted into a sequence of "safe" Unicode characters using the `BYTES_TO_UNICODE` map.
4.  The `BytePairEncoder` then iteratively merges the most frequent pairs of subword units based on the ranks provided in `MERGES`.
5.  This process continues until no more merges are possible, and the final output is a sequence of token IDs.

## Decoding Process
1.  Each token ID in the input sequence is mapped to its corresponding sequence of "safe" Unicode codepoints using the `*_UNICODES` map.
2.  These codepoints are converted to a string, which is then segmented into graphemes.
3.  Each grapheme is mapped back to its original byte value using the `UNICODE_TO_BYTES` map.
4.  The resulting bytes are collected to reconstruct the original text.

## Performance Considerations
- All static mappings (`MERGES`, vocabularies, and Unicode maps) are initialized once using `LazyLock`, ensuring fast lookups during encoding and decoding.
- The `regex` crate provides an efficient implementation for the initial tokenization splitting.
- The `merges.txt` file and vocabulary files are embedded in the binary, removing runtime file I/O overhead.
