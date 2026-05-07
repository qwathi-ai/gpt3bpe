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
    let location = match std::env::var("EMBEDDINGS") {
        Ok(l) => Some(l),
        Err(_) => {
            panic!(
                "[ERROR]: `EMBEDDINGS` environment variable not set."
            );           
        }
    };
    match embeddings::insert(&embeddings::connection(location.as_deref()), slice, embeddings) {
        Ok(_) => true,
        Err(e) => {
            if e.sqlite_error_code() == Some(rusqlite::ErrorCode::ConstraintViolation) {
                #[cfg(debug_assertions)]
                println!(
                    "[WARNING]: {:?} aleardy exists",
                    String::from_utf8(slice.to_vec()).unwrap_or_default()
                );
                true         
            } else {
                #[cfg(debug_assertions)]
                println!(
                    "[ERROR]: Could not insert {:?}.\n{:?}",
                    String::from_utf8(slice.to_vec()).unwrap_or_default(),
                    e
                );
                false
            }
        }
    }
}

#[cfg(feature = "embeddings")]
#[no_mangle]
pub extern "C" fn search(
    buffer: *const u8,
    buffer_length: usize,
    k: u8,
    callback: extern "C" fn(u16, f32, usize, f32),
) {
    let slice = read::<u8>(buffer, buffer_length);
    let location = match std::env::var("EMBEDDINGS") {
        Ok(l) => Some(l),
        Err(_) => {
            panic!(
                "[ERROR]: `EMBEDDINGS` environment variable not set."
            );           
        }
    };
    let mut top = embeddings::search::<{ embeddings::DIMENSIONS }>(&embeddings::connection(location.as_deref()), slice, k).unwrap();
    for row in top.drain(..) {
        for (position, value) in row.vector.iter().enumerate() {
            callback( row.rid, row.distance, position, *value);
        };
    };
}

#[cfg(feature = "embeddings")]
#[no_mangle]
pub extern "C" fn nearest(
    vector: *const f32,
    vector_length: usize,
    k: u8,
    callback: extern "C" fn(u16, f32, usize, usize, u8),
) {
    let slice: &[f32; embeddings::DIMENSIONS] =
        read::<f32>(vector, vector_length).try_into().unwrap();
    let location = match std::env::var("EMBEDDINGS") {
        Ok(l) => Some(l),
        Err(_) => {
            panic!(
                "[ERROR]: `EMBEDDINGS` environment variable not set."
            );           
        }
    };
    let mut top = embeddings::nearest::<{ embeddings::DIMENSIONS }>(&embeddings::connection(location.as_deref()), slice, k).unwrap();
    for row in top.drain(..) {
        let bytes = row.label.as_bytes();
        let len = bytes.len();
        for (position, value) in bytes.iter().enumerate() {
            callback( row.rid, row.distance, len, position, *value);
        };
    };
}
