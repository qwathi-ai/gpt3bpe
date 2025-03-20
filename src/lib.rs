// //! Generative Pre-trained Byte Pair Encoder (GPT3BPE)
// //!
// //! #Overview
// //! This module provides utility functions for encoding and decoding text using a Generative Pre-trained Transformer tokenized Byte Pair Encoder.
// //! These functions are designed to facilitate the pre-processing of text data for natural language processing tasks and the post-processing of tokenized data back into human-readable text.
// //!
// //! # Functions
// //!
mod error;
mod parser;
mod tokenizer;
use unicode_segmentation::UnicodeSegmentation;

/// Encodes a given byte slice into a vector of GPT-3 tokens.
/// ## Encode
///
/// ### Arguments
/// * `slice` - a byte vector.
///
/// ### Returns
/// * a [GPT-3 token](crate::tokenizer::tokens) equivalent of slice.
pub fn encode(slice: &[u8]) -> Result<Vec<u16>, crate::error::Error> {
    let tokens = tokenizer::tokenize(slice).unwrap().concat();
    assert!(!tokens.is_empty(), "[ERROR]: No tokens found");
    Ok(tokens)
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
        let unicodes = match tokenizer::TOKENS_TO_GPT_UNICODES.get(token) {
            Some(value) => value.concat(),
            None => todo!(),
        };

        let text = String::from_utf8_lossy(&unicodes);

        let grapheme: Vec<u8> = UnicodeSegmentation::graphemes(text.as_ref(), true)
            .flat_map(|char| -> Vec<u8> {
                match crate::tokenizer::GPT_UNICODES_TO_BYTES.get(char.as_bytes()) {
                    Some(bytes) => vec![*bytes],
                    None => {
                        println!("{:?}", char);
                        char.as_bytes().to_vec()
                    }
                    //vec![(*token).try_into().expect("[ERROR]: token could not be decoded.")]
                }
            })
            .collect();

        #[cfg(debug_assertions)]
        println!("[DEBUG][DECODE][GRAPHEME]: {:?}", grapheme);

        decoding.extend(grapheme);
        decoding
    });

    Ok(resolve)
}

fn read<T>(pointer: *const T, length: usize) -> &'static [T] {
    assert!(!pointer.is_null(), "[ERROR]: pointer is null.");
    assert!(
        pointer.is_aligned(),
        "[ERROR]: pointer is not properly aligned."
    );
    assert!(
        length < usize::MAX / 8,
        "[ERROR]: buffer overflow, greater than isize::MAX"
    );
    let slice = unsafe { std::slice::from_raw_parts(pointer, length) };
    assert_eq!(slice.len(), length, "[ERROR]: slice length mismatch.");
    slice
}

#[no_mangle]
pub extern "C" fn encode_ffi(
    buffer: *const u8,
    length: usize,
    callback: extern "C" fn(usize, u16),
) {
    let slice = read(buffer, length);
    let mut encoding = encode(slice).unwrap();
    for (idx, value) in encoding.drain(..).enumerate() {
        callback(idx, value)
    }
}

#[no_mangle]
pub extern "C" fn decode_ffi(
    buffer: *const u16,
    length: usize,
    callback: extern "C" fn(usize, u8),
) {
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
