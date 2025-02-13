# GPT Byte-Pair Encoding (BPE) for GPT-3 in Rust

## Overview
Implementation of a GPT Byte-Pair Encoder (BPE) designed for GPT-3 in Rust. The library is structured for research and engineering purposes, and all relevant code is contained within a single file.

The BPE algorithm tokenizes text into subword units using a pre-defined tokenization strategy aligned with GPT-3's encoding scheme. The implementation relies on a set of static mappings to facilitate fast encoding and decoding.

## Static Mappings
The implementation includes the following static mappings:

### `TOKENS_RE`
A regular expression pattern used to match tokens in the input text. This pattern defines the tokenization strategy, capturing subwords, whitespace, and special characters as per GPT-3's encoding scheme.

### `GPT_UNICODES`
A predefined array of Unicode character mappings used in GPT-3’s encoding. These represent the character set supported by the model.

### `BYTES_TO_GPT_UNICODES`
A mapping from byte values (0-255) to their corresponding GPT Unicode values. This allows efficient conversion of raw text bytes into GPT-3’s internal representation.

### `GPT_UNICODES_TO_BYTES`
The inverse mapping of `BYTES_TO_GPT_UNICODES`, enabling conversion from GPT-3 Unicode values back to raw byte values. This is useful for decoding tokenized text.

### `GPT_UNICODES_TO_TOKENS`
A mapping from GPT Unicode values to token strings. This mapping facilitates the encoding process, allowing the library to translate Unicode values into their respective token representations.

### `TOKENS_TO_GPT_UNICODES`
The reverse mapping of `GPT_UNICODES_TO_TOKENS`, allowing token strings to be mapped back to GPT Unicode values for decoding purposes.

## Encoding Process
1. Normalize input text to ensure consistent representation.
2. Apply `TOKENS_RE` to segment text into tokens.
3. Convert matched tokens into GPT Unicode values using `TOKENS_TO_GPT_UNICODES`.
4. Apply Byte-Pair Encoding (BPE) merges to iteratively reduce token sequences based on GPT-3’s trained merge rules.
5. Output the final tokenized sequence as a list of token indices.

## Decoding Process
1. Convert token indices back to GPT Unicode values using `GPT_UNICODES_TO_TOKENS`.
2. Map GPT Unicode values back to bytes using `GPT_UNICODES_TO_BYTES`.
3. Reconstruct the original text from byte values, ensuring proper handling of special characters and whitespace.

## Use Cases
- Research on GPT tokenization and encoding strategies.
- Efficient text preprocessing for GPT-3 applications in Rust.
- Analysis and experimentation with different BPE merge strategies.

## Performance Considerations
- The static mappings allow for constant-time lookups during encoding and decoding.
- Efficient regular expressions ensure minimal overhead in tokenization.
- Optimized BPE merge rules improve processing speed for large text inputs.

## Special Thanks to:
* Chatgpt for the documentation :)

