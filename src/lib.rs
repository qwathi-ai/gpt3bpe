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
pub extern "C" fn grapheme(buffer: *const u8, length: usize, callback: extern "C" fn (usize, u8) ) {
    let slice = read(buffer, length);

    let grapheme = bpe::grapheme(slice);
    for (idx, value) in grapheme.concat().drain(..).enumerate() {
        callback(idx, value)
    };
}

#[no_mangle]
pub extern "C" fn encode_r50k(buffer: *const u8, length: usize, callback: extern "C" fn (usize, u16) ) {
    let slice = read(buffer, length);

    let mut encoding = bpe::encode(slice, &crate::bpe::vocabulary::R50K_TOKENS); 
    for (idx, value) in encoding.drain(..).enumerate() {
        callback(idx, value)
    };
}

#[no_mangle]
pub extern "C" fn decode_r50k(buffer: *const u16, length: usize, callback: extern "C" fn (usize, u8)) {
    let slice = read(buffer, length);

    let mut decoding = bpe::decode(slice, &crate::bpe::vocabulary::R50K_UNICODES);
    for (idx, value) in decoding.drain(..).enumerate() {
        callback(idx, value)
    }
}

#[no_mangle]
pub extern "C" fn encode_p50k(buffer: *const u8, length: usize, callback: extern "C" fn (usize, u16) ) {
    let slice = read(buffer, length);

    let mut encoding = bpe::encode(slice, &crate::bpe::vocabulary::R50K_TOKENS);
    for (idx, value) in encoding.drain(..).enumerate() {
        callback(idx, value)
    };
}

#[no_mangle]
pub extern "C" fn decode_p50k(buffer: *const u16, length: usize, callback: extern "C" fn (usize, u8)) {
    let slice = read(buffer, length);

    let mut decoding = bpe::decode(slice, &crate::bpe::vocabulary::R50K_UNICODES); //.unwrap();
    for (idx, value) in decoding.drain(..).enumerate() {
        callback(idx, value)
    }
}

