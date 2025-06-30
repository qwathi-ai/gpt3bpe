// //! Generative Pre-trained Byte Pair Encoder (GPT3BPE)
// //!
// //! # Overview
// //! This module provides utility functions for encoding and decoding text using a Generative Pre-trained Transformer tokenized Byte Pair Encoder.
// //! These functions are designed to facilitate the pre-processing of text data for natural language processing tasks and the post-processing of tokenized data back into human-readable text.
// //!
// //! # Functions
// //!
mod error;
mod tokenizer;
use tokenizer::GPT_UNICODES_TO_TOKENS;
use unicode_segmentation::UnicodeSegmentation;

enum Encodings {
    P50kBase,
    R50kBase,
    P50kEdit,
    Cl100kBase,
    O200kBase
}
/// Encodes a given byte slice into a vector of GPT-3 tokens.
/// ## Encode
///
/// ### Arguments
/// * `slice` - a byte vector.
///
/// ### Returns
/// * a [GPT-3 token](crate::tokenizer::tokens) equivalent of slice.
pub fn encode(slice: &[u8]) -> Result<Vec<u16>, crate::error::Error> {
    let token_ids = tokenizer::tokens(slice)
        .iter()
        .fold(vec![], |mut encoding, value| {
            let graph = tokenizer::grapheme(&value.concat()).unwrap();
            let tokens = match GPT_UNICODES_TO_TOKENS.get(&graph.concat()) {
                Some(t) => vec![*t],
                None => {
                    let encoder = tokenizer::BytePairEncoder::from(&graph);
                    encoder.into_iter().fold(vec![], |_encoding, value| value)
                }
            };

            encoding.push(tokens);
            encoding
        });

    Ok(token_ids.concat())
}

/// Decodes a given slice of GPT-3 tokens into byte slice.
/// ## Decode
///
/// ### Arguments
/// * `slice` - GPT-2 token slice.
///
/// ### Returns
/// * a byte vector.
pub fn decode(slice: &[u16]) -> Result<Vec<u8>, crate::error::Error> {
    let resolve = slice.iter().fold(vec![], |mut decoding, token| -> Vec<u8> {
        match tokenizer::TOKENS_TO_GPT_UNICODES.get(token) {
            Some(unicode) => {
                let text = String::from_utf8(unicode.concat()).unwrap();

                let unicode: Vec<u8> = UnicodeSegmentation::graphemes(text.as_str(), true)
                    .flat_map(|char| -> Vec<u8> {
                        match crate::tokenizer::GPT_UNICODES_TO_BYTES.get(char.as_bytes()) {
                            Some(bytes) => {
                                vec![*bytes]
                            }
                            None => char.as_bytes().to_vec(),
                        }
                    })
                    .collect();

                decoding.extend(unicode);
            }
            None => todo!(),
        };

        decoding
    });

    Ok(resolve)
}

fn read<T>(pointer: *const T, length: usize) -> &'static [T] {
    assert!(!pointer.is_null(), "[ERROR]: pointer is null.");
    assert!(
        pointer.is_aligned(),
        "[ERROR]: pointer not properly aligned for type [T]."
    );
    assert!(length < usize::MAX / 8, "[ERROR]: buffer overflow.");
    let slice = unsafe { std::slice::from_raw_parts(pointer, length) };
    assert_eq!(
        slice.len(),
        length,
        "[ERROR]: pointer not properly aligned."
    );
    slice
}

#[no_mangle]
pub extern "C" fn encode_ffi(buffer: *const u8, length: usize, callback: extern "C" fn (usize, u16) ) {
    let slice = read(buffer, length);
    let mut encoding = encode(slice).unwrap();
    for (idx, value) in encoding.drain(..).enumerate() {
        callback(idx, value)
    };
}

#[no_mangle]
pub extern "C" fn decode_ffi(buffer: *const u16, length: usize, callback: extern "C" fn (usize, u8)) {
    let slice = read(buffer, length);
    let mut decoding = decode(slice).unwrap();
    for (idx, value) in decoding.drain(..).enumerate() {
        callback(idx, value)
    }
}

mod tests {
    #[test]
    fn encode() {
        assert_eq!(
            super::encode(b"let there be light.").unwrap(),
            vec![1616, 612, 307, 1657, 13]
        );
        assert_eq!(
            super::encode(b"indivisible values").unwrap(),
            vec![521, 452, 12843, 1988, 82]
        );
        assert_eq!(
            super::encode(b"Pneumonoultramicroscopicsilicovolcanoconiosis").unwrap(),
            vec![
                47, 25668, 261, 25955, 859, 291, 4951, 22163, 873, 41896, 709, 349, 5171, 420, 78,
                77, 4267, 72, 82
            ]
        );
        assert_eq!(
            super::encode(b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D").unwrap(),
            vec![31373, 50169, 233, 995, 220, 172, 253, 234, 235]
        );
    }

    #[test]
    fn decode() {
        assert_eq!(
            b"let there be light.",
            String::from_utf8_lossy(&super::decode(&[1616, 612, 307, 1657, 13]).unwrap())
                .as_bytes()
        );
        assert_eq!(
            b"indivisible values",
            String::from_utf8_lossy(&super::decode(&[521, 452, 12843, 1988, 82]).unwrap())
                .as_bytes()
        );
        assert_eq!(
            b"Pneumonoultramicroscopicsilicovolcanoconiosis",
            String::from_utf8_lossy(
                &super::decode(&[
                    47, 25668, 261, 25955, 859, 291, 4951, 22163, 873, 41896, 709, 349, 5171, 420,
                    78, 77, 4267, 72, 82
                ])
                .unwrap()
            )
            .as_bytes()
        );
        assert_eq!(
            b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D",
            String::from_utf8_lossy(
                &super::decode(&[31373, 50169, 233, 995, 220, 172, 253, 234, 235]).unwrap()
            )
            .as_bytes()
        );
    }
}
