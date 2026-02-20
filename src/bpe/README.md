# GPT Byte-Pair Encoding

## Overview
This document describes the implementation of a GPT Byte-Pair Encoder in Rust. The library provides tokenization for various GPT models by implementing their specific byte-pair encoding schemes. It relies on a set of static mappings and vocabulary files for efficient encoding and decoding.

## Core Components

## Static Mappings
The implementation utilizes several `LazyLock` static variables for performance:

### `TOKENS_RE`
A regex pattern for the initial splitting of text into processable chunks, based on the GPT-2/3 tokenization strategy.

### `GPT_UNICODES`
A predefined array of unicode characters used to create a mapping from bytes (0-255) to a set of "safe" unicode characters. This avoids issues with control characters and whitespace during BPE.

### `UNICODE_TO_BYTES`
A `BTreeMap<u16, Vec<u8>>` mapping a byte value (0-255) to the UTF-8 bytes of its corresponding "safe" unicode character.

### `BYTES_TO_UNICODE`
A `BTreeMap<Vec<u8>, u16>` providing the inverse mapping from a "safe" unicode character's bytes back to the original byte value.

### `MERGES`
A `HashMap<Vec<u8>, u32>` loaded from `src/bpe/merges.txt`. It provides BPE merge ranks which are used by the `BytePairEncoder` to merge subword units.

### Vocabulary Files
The tokenizer supports different GPT models by loading their respective vocabularies from `.jsonl` files. These are loaded into two types of maps:

- **`*_TOKENS`**: A `BTreeMap<Vec<u8>, TokenID>` that maps a token string (as bytes) to its unique token ID (e.g., `CL100K_TOKENS`).
- **`*_UNICODES`**: A `BTreeMap<TokenID, Vec<u16>>` that maps a token ID back to a sequence of unicode codepoints, used for decoding (e.g., `CL100K_UNICODES`).

Supported vocabularies include `r50k_base`, `p50k_base`, `cl100k_base`, and `o200k_base`.

## Graphemes
A **grapheme** is the smallest unit of a writing system. The library uses grapheme segmentation to correctly handle multi-byte characters and composite characters (like "é") during the encoding and decoding process, ensuring text integrity.

## Encoding Process
1.  Input text (as bytes) is split into chunks using the `TOKENS_RE` regex.
2.  Each chunk is converted into a sequence of "safe" unicode characters using the `UNICODE_TO_BYTES` map. This process is handled by the `grapheme` function.
3.  The encoder first attempts to find the entire chunk as a single token in the `*_TOKENS` vocabulary.
4.  If not found, the `BytePairEncoder` iteratively merges the most frequent pairs of subword units based on the ranks provided in `MERGES` and the vocabulary, until no more merges are possible.
5.  The final output is a sequence of token IDs.

## Decoding Process
1.  Each token ID is mapped to its sequence of "safe" unicode codepoints using the `*_UNICODES` map.
2.  These codepoints are converted to a string, which is then segmented into graphemes.
3.  Each grapheme is mapped back to its original byte value using the `BYTES_TO_UNICODE` map.
4.  The resulting bytes are collected to reconstruct the original text.

## Use Cases
- [x] Research on GPT tokenization and encoding strategies.
- [x] Efficient text preprocessing for GPT applications in Rust.
- [ ] Analysis and experimentation with different BPE merge strategies.

## Performance Considerations
- Static mappings in `LazyLock` allow for fast lookups during encoding and decoding.
- The `regex` crate provides efficient initial tokenization.
- The BPE implementation iteratively merges tokens to produce the final encoding.
