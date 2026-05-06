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
        "[ERROR]: pointer not properly aligned for type T."
    );
    assert!(
        length < (usize::MAX / std::mem::size_of::<T>()) / 16,
        "[ERROR]: buffer overflow."
    );
    let slice = unsafe { std::slice::from_raw_parts(pointer, length) };
    assert_eq!(
        slice.len(),
        length,
        "[ERROR]: pointer not properly aligned."
    );
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
    let slice = read::<u8>(buffer, length);
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
    let slice = read::<u16>(buffer, length);

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
    let slice = read::<u8>(buffer, length);

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
    let slice = read::<u16>(buffer, length);

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
    let slice = read::<u8>(buffer, length);

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
    let slice = read::<u32>(buffer, length);

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
    let slice = read::<u8>(buffer, length);

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
    let slice = read::<u32>(buffer, length);

    let mut decoding = bpe::decode(slice, &crate::bpe::vocabulary::O200K_UNICODES);
    for (idx, value) in decoding.drain(..).enumerate() {
        callback(idx, value)
    }
}

#[cfg(feature = "embeddings")]
mod embeddings;

#[cfg(feature = "embeddings")]
#[no_mangle]
pub extern "C" fn insert(
    buffer: *const u8,
    buffer_length: usize,
    vector: *const f32,
    vector_length: usize,
) -> bool {
    let slice = read::<u8>(buffer, buffer_length);
    let embeddings: &[f32; embeddings::DIMENSIONS] =
        read::<f32>(vector, vector_length).try_into().unwrap();
    let location = match std::env::var("EMBEDDING_LOCATION") {
        Ok(l) => Some(l),
        Err(_) => None
    };
    match embeddings::insert(&embeddings::connection(location.as_deref()), slice, embeddings) {
        Ok(_) => true,
        Err(_) => {
            #[cfg(debug_assertions)]
            println!(
                "[WARNING]: Could not embed {:?}.",
                String::from_utf8(slice.to_vec()).unwrap_or_default()
            );
            false
        }
    }
}

#[cfg(feature = "embeddings")]
#[no_mangle]
pub extern "C" fn search(
    buffer: *const u8,
    buffer_length: usize,
    k: u8,
    callback: extern "C" fn(usize, usize, usize, f32),
) {
    let slice = read::<u8>(buffer, buffer_length);
    let location = match std::env::var("EMBEDDING_LOCATION") {
        Ok(l) => Some(l),
        Err(_) => None
    };
    let mut top =
        match embeddings::search::<{ embeddings::DIMENSIONS }>(&embeddings::connection(location.as_deref()), slice, k)
        {
            Ok(t) => t,
            Err(e) => {
                #[cfg(debug_assertions)]
                println!(
                    "[ERROR]: Search not found {:?}.\n{:?}",
                    String::from_utf8(slice.to_vec()).unwrap_or_default(),
                    e
                );
                vec![]
            }
        };
    for (idx, results) in top.drain(..).enumerate() {
        let len = results.vector.len();
        for (position, value) in results.vector.iter().enumerate() {
            println!("idx: {:?} len: {:?} position: {:?} value: {:?}", idx, len, position, value);
            callback( idx, len, position, *value);
        };
    };
}

#[cfg(feature = "embeddings")]
#[no_mangle]
pub extern "C" fn euclid(
    vector: *const f32,
    vector_length: usize,
    k: u8,
    callback: extern "C" fn(usize, usize, usize, u8),
) {
    let slice: &[f32; embeddings::DIMENSIONS] =
        read::<f32>(vector, vector_length).try_into().unwrap();
    let location = match std::env::var("EMBEDDING_LOCATION") {
        Ok(l) => Some(l),
        Err(_) => None
    };
    let mut top =
        match embeddings::euclid::<{ embeddings::DIMENSIONS }>(&embeddings::connection(location.as_deref()), slice, k) {
            Ok(t) => t,
            Err(e) => {
                #[cfg(debug_assertions)]
                println!("[ERROR]: Top not found.\n{:?}", e);
                vec![]
            }
        };
    for (idx, results) in top.drain(..).enumerate() {
        let bytes = results.label.as_bytes();
        let len = bytes.len();
        for (position, value) in bytes.iter().enumerate() {
            callback( idx, len,position, *value);
        };
    };
}
