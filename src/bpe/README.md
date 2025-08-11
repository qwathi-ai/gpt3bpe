# GPT Byte-Pair Encoding

## Overview
This document describes the implementation of a GPT Byte-Pair Encoder written in Rust. The library is structured for research and engineering purposes.

The BPE algorithm tokenizes text into subword units using a pre-defined tokenization strategy aligned with GPT encoding scheme. The implementation relies on a set of static mappings to facilitate fast encoding and decoding.

## Static Mappings
The implementation includes the following static mappings:

### `TOKENS_RE`
A regular expression pattern used to match tokens in the input text. This pattern defines the tokenization strategy, capturing subwords, whitespace, and special characters as per GPT encoding scheme.

### `GPT_UNICODES`
A predefined array of the GPT Unicode scheme. This represents the character set supported by the model.

### `BYTES_TO_UNICODES`
A mapping of byte values (0-255) to their corresponding GPT Unicodes. This allows efficient conversion of bytes into the GPT internal representation.

### `UNICODES_TO_BYTES`
The inverse mapping of `BYTES_TO_UNICODES`, enabling conversion from GPT-3 Unicode scheme to byte values. This is useful for decoding a tokenized array.

### `UNICODES_TO_TOKENS`
A mapping from the GPT Unicode values to tokens. This mapping facilitates the encoding process, allowing the library to translate Unicode values into their respective token representations.

### `TOKENS_TO_UNICODES`
The reverse mapping of `UNICODES_TO_TOKENS`, allowing token strings to be mapped back to the GPT-3 unicode scheme Unicode values for decoding purposes.

## Graphemes
A **grapheme** is the smallest unit of a writing system that represents a single, meaningful character. In some cases, a grapheme may consist of multiple Unicode code points that together form a single visual character. For instance, "é" can be represented as a single precomposed character (U+00E9) or as a combination of "e" (U+0065) and an acute accent (U+0301). Understanding graphemes is crucial for accurate tokenization, as naive character splitting may incorrectly segment meaningful text units.

## Encoding Process
1. Normalize input text to ensure consistent representation.
2. Apply `TOKENS_RE` to segment text into tokens.
3. Convert matched tokens into the GPT Unicode values using `TOKENS_TO_UNICODES`.
4. Apply Byte-Pair Encoding (BPE) merges to iteratively reduce token sequences based on trained merge rules.
5. Output the final tokenized sequence.

## Decoding Process
1. Convert token indices back to GPT Unicode values using `UNICODES_TO_TOKENS`.
2. Map GPT Unicode values back to bytes using `UNICODES_TO_BYTES`.
3. Reconstruct the original text from byte values, ensuring proper handling of special characters and whitespace.

## Use Cases
- [x] Research on GPT tokenization and encoding strategies.
- [x[ Efficient text preprocessing for GPT applications in Rust.
- [ ] Analysis and experimentation with different BPE merge strategies. 

## Performance Considerations
- The static mappings allow for constant-time lookups during encoding and decoding.
- Efficient regular expressions ensure minimal overhead in tokenization.
- Optimized BPE merge rules improve processing speed for large text inputs.
