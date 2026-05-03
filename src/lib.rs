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
    match embeddings::insert(&embeddings::CONNECTION.lock().unwrap(), slice, embeddings) {
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
    callback: extern "C" fn(usize, usize, *mut f32),
) -> *mut c_void {
    let slice = read::<u8>(buffer, buffer_length);
    let mut top =
        match embeddings::search::<{ embeddings::DIMENSIONS }>(&CONNECTION.lock().unwrap(), slice, k)
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
    for (idx, value) in top.iter_mut().enumerate() {
        let results = &mut value.vector;
        let len = results.len();
        let ptr = results.as_mut_ptr();
        callback(idx, len, ptr);
    }
    let boxed_results = Box::new(top);
    Box::into_raw(boxed_results) as *mut c_void
}

// #[cfg(feature = "embeddings")]
// #[no_mangle]
// pub extern "C" fn top(
//     vector: *const f32,
//     vector_length: usize,
//     k: u8,
//     callback: extern "C" fn(usize, usize, *mut u8),
// ) -> *mut c_void {
//     let slice: &[f32; embeddings::DIMENSIONS] =
//         read::<f32>(vector, vector_length).try_into().unwrap();
//     let mut top =
//         match embeddings::top::<{ embeddings::DIMENSIONS }>(&CONNECTION.lock().unwrap(), slice, k) {
//             Ok(t) => t,
//             Err(e) => {
//                 #[cfg(debug_assertions)]
//                 println!("[ERROR]: Top not found.\n{:?}", e);
//                 vec![]
//             }
//         };
//     for (idx, value) in top.iter_mut().enumerate() {
//         let results = unsafe { value.label.as_bytes_mut() };
//         let len = results.len();
//         let ptr = results.as_mut_ptr();
//         callback(idx, len, ptr);
//     }
//     let boxed_results = Box::new(top);
//     Box::into_raw(boxed_results) as *mut c_void
// }

// #[cfg(feature = "embeddings")]
// #[no_mangle]
// pub extern "C" fn free(ptr: *mut c_void) {
//     if !ptr.is_null() {
//         unsafe {
//             drop(Box::from_raw(
//                 ptr as *mut Vec<embeddings::Top<{ embeddings::DIMENSIONS }>>,
//             ));
//         }
//     }
// }
