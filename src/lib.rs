// #![feature(portable_simd)]
// //! Generative Pre-trained Transformer Byte Pair Encoder (GPTBPE)
// //!
// //! # Overview
// //! This module provides utility functions for encoding and decoding text using a Generative Pre-trained Transformer tokenized Byte Pair Encoder.
// //! These functions are designed to facilitate the pre-processing of text data for natural language processing tasks and the post-processing of tokenized data back into human-readable text.
// //!
// //! # Functions
// //!
mod bpe;

fn read<T>(pointer: *const T, length: usize) -> &'static [T] {
    assert!(!pointer.is_null(), "[ERROR]: pointer is null.");
    assert!(pointer.is_aligned(), "[ERROR]: pointer not properly aligned for type [T].");
    assert!(length < usize::MAX / 8, "[ERROR]: buffer overflow.");
    let slice = unsafe { std::slice::from_raw_parts(pointer, length) };
    assert_eq!(slice.len(),length,"[ERROR]: pointer not properly aligned.");
    slice
}

#[no_mangle]
pub extern "C" fn grapheme(buffer: *const u8, length: usize, callback: extern "C" fn(usize, u8)) {
    let slice = read(buffer, length);

    let grapheme = bpe::grapheme(slice);
    for (idx, value) in grapheme.concat().drain(..).enumerate() {
        callback(idx, value)
    }
}

#[no_mangle]
pub extern "C" fn encode_r50k(
    buffer: *const u8,
    length: usize,
    callback: extern "C" fn(usize, u16),
) {
    let slice = read(buffer, length);

    let mut encoding = bpe::encode(slice, &crate::bpe::vocabulary::R50K_TOKENS).concat();
    for (idx, value) in encoding.drain(..).enumerate() {
        callback(idx, value.try_into().unwrap())
    }
}

#[no_mangle]
pub extern "C" fn decode_r50k(
    buffer: *const u16,
    length: usize,
    callback: extern "C" fn(usize, u8),
) {
    let slice = read(buffer, length);

    let mut decoding = bpe::decode(slice, &crate::bpe::vocabulary::R50K_UNICODES);
    for (idx, value) in decoding.drain(..).enumerate() {
        callback(idx, value)
    }
}

#[no_mangle]
pub extern "C" fn encode_p50k(
    buffer: *const u8,
    length: usize,
    callback: extern "C" fn(usize, u16),
) {
    let slice = read(buffer, length);

    let mut encoding = bpe::encode(slice, &crate::bpe::vocabulary::P50K_TOKENS).concat();
    for (idx, value) in encoding.drain(..).enumerate() {
        callback(idx, value.try_into().unwrap())
    }
}

#[no_mangle]
pub extern "C" fn decode_p50k(
    buffer: *const u16,
    length: usize,
    callback: extern "C" fn(usize, u8),
) {
    let slice = read(buffer, length);

    let mut decoding = bpe::decode(slice, &crate::bpe::vocabulary::P50K_UNICODES);
    for (idx, value) in decoding.drain(..).enumerate() {
        callback(idx, value)
    }
}

#[no_mangle]
pub extern "C" fn encode_cl100k(
    buffer: *const u8,
    length: usize,
    callback: extern "C" fn(usize, u32),
) {
    let slice = read(buffer, length);

    let mut encoding = bpe::encode(slice, &crate::bpe::vocabulary::CL100K_TOKENS).concat();
    for (idx, value) in encoding.drain(..).enumerate() {
        callback(idx, value)
    }
}

#[no_mangle]
pub extern "C" fn decode_cl100k(
    buffer: *const u32,
    length: usize,
    callback: extern "C" fn(usize, u8),
) {
    let slice = read(buffer, length);

    let mut decoding = bpe::decode(slice, &crate::bpe::vocabulary::CL100K_UNICODES);
    for (idx, value) in decoding.drain(..).enumerate() {
        callback(idx, value)
    }
}

#[no_mangle]
pub extern "C" fn encode_o200k(
    buffer: *const u8,
    length: usize,
    callback: extern "C" fn(usize, u32),
) {
    let slice = read(buffer, length);

    let mut encoding = bpe::encode(slice, &crate::bpe::vocabulary::O200K_TOKENS).concat();
    for (idx, value) in encoding.drain(..).enumerate() {
        callback(idx, value)
    }
}

#[no_mangle]
pub extern "C" fn decode_o200k(
    buffer: *const u32,
    length: usize,
    callback: extern "C" fn(usize, u8),
) {
    let slice = read(buffer, length);

    let mut decoding = bpe::decode(slice, &crate::bpe::vocabulary::O200K_UNICODES);
    for (idx, value) in decoding.drain(..).enumerate() {
        callback(idx, value)
    }
}

#[cfg(feature = "embedding")]
mod embedding;
/// Pads or truncates a vector to a fixed-size array of length 3.
///
/// If the input vector is shorter than 3, it is padded at the beginning with
/// the default value for the type `T`. If it is longer than 3, only the last
/// 3 elements are taken.
///
/// # Type Parameters
///
/// * `T`: The type of the elements, which must implement `Default` and `Copy`.
///
/// # Arguments
///
/// * `input`: The `Vec<T>` to pad or truncate.
///
/// # Returns
///
/// An array `[T; 3]`.
#[cfg(feature = "embedding")]
#[no_mangle]
pub extern "C" fn embed_p50k(
    buffer: *const u8,
    buffer_length: usize,
    vector: *const f32,
    vector_length: usize
) -> usize {
    let slice = read(buffer, buffer_length);
    let tokens = bpe::encode(slice, &crate::bpe::vocabulary::P50K_TOKENS).concat();
    let embedding = read(vector, vector_length);
    let label = String::from_utf8(slice.to_vec()).unwrap();
    match embedding::embed(tokens, "p50k", &label, embedding) {
        Ok(response) => response,
        Err (_) => {
            #[cfg(debug_assertions)]
            println!( "[WARNING]: P50K Could not embed {:?}.",label);
            0        
        }

    }
}

#[cfg(feature = "embedding")]
#[no_mangle]
pub extern "C" fn embed_r50k(
    buffer: *const u8,
    buffer_length: usize,
    vector: *const f32,
    vector_length: usize
) -> usize {
    let slice = read(buffer, buffer_length);
    let tokens = bpe::encode(slice, &crate::bpe::vocabulary::R50K_TOKENS).concat();
    let embedding = read(vector, vector_length);
    let label = String::from_utf8(slice.to_vec()).unwrap();

    match embedding::embed(tokens, "r50k", &label, embedding) {
        Ok(response) => response,
        Err(_) => {
            #[cfg(debug_assertions)]
            println!( "[WARNING]: R50K Could not embed {:?}.",label);
            0
        }
    }
}

#[cfg(feature = "embedding")]
#[no_mangle]
pub extern "C" fn embed_cl100k(
    buffer: *const u8,
    buffer_length: usize,
    vector: *const f32,
    vector_length: usize
) -> usize {
    let slice = read(buffer, buffer_length);
    let tokens = bpe::encode(slice, &crate::bpe::vocabulary::CL100K_TOKENS).concat();
    let embedding = read(vector, vector_length);
    let label = String::from_utf8(slice.to_vec()).unwrap();

    match embedding::embed(tokens, "cl100k", &label, embedding) {
        Ok(response) => response,
        Err(_) => {
            #[cfg(debug_assertions)]
            println!( "[WARNING]: CL100K Could not embed {:?}.",label);
            0
        }
    }
}

#[cfg(feature = "embedding")]
#[no_mangle]
pub extern "C" fn embed_o200k(
    buffer: *const u8,
    buffer_length: usize,
    vector: *const f32,
    vector_length: usize
) -> usize {
    let slice = read(buffer, buffer_length);
    let tokens = bpe::encode(slice, &crate::bpe::vocabulary::O200K_TOKENS).concat();
    let embedding = read(vector, vector_length);
    let label = String::from_utf8(slice.to_vec()).unwrap();

    match embedding::embed(tokens, "o200k", &label, embedding) {
        Ok(response) => response,
        Err(_) => {
            #[cfg(debug_assertions)]
            println!( "[WARNING]: O200K Could not embed {:?}.",label);
            0
        }
    }
}