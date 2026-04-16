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
    assert!(pointer.is_aligned(), "[ERROR]: pointer not properly aligned for type T.");
    assert!(length < (usize::MAX / std::mem::size_of::<T>()) / 16, "[ERROR]: buffer overflow.");
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
    };
}

#[cfg(feature = "embedding")]
mod embedding;
#[cfg(feature = "embedding")]
const DIMENSIONS : usize = 300;

#[cfg(feature = "embedding")]
#[no_mangle]
pub extern "C" fn embed(
    buffer: *const u8,
    buffer_length: usize,
    vector: *const f32,
    vector_length: usize
) -> bool {
    let slice = read::<u8>(buffer, buffer_length);
    let embedding: &[f32; DIMENSIONS] = read::<f32>(vector, vector_length).try_into().unwrap();
    match embedding::embed(slice, embedding) {
        Ok(_) => true,
        Err (_) => {
            #[cfg(debug_assertions)]
            println!( "[WARNING]: Could not embed {:?}.", String::from_utf8(slice.to_vec()).unwrap_or_default());
            false       
        }
    }
}

#[cfg(feature = "embedding")]
use std::os::raw::c_char;
#[cfg(feature = "embedding")]
use std::ffi::CString;

/// A C-compatible version of the `embedding::Top` struct for FFI.
///
/// Strings are represented as raw pointers to null-terminated C strings.
/// The caller of the FFI function is responsible for freeing the memory
/// for these strings.
#[cfg(feature = "embedding")]
#[repr(C)]
struct TopFFI<const D: usize> {
    pub rid: u16,
    pub label: *const c_char,
    pub vocab: *const c_char,
    pub vector: [f32; D],
    pub distance: f32,
}

#[cfg(feature = "embedding")]
impl Into<TopFFI<{ DIMENSIONS }>> for embedding::Top<{ DIMENSIONS }> {
    fn into(self) -> TopFFI<{ DIMENSIONS }> {
        TopFFI {
            rid: self.rid,
            label: CString::new(self.label).unwrap().into_raw(),
            vocab: CString::new(self.vocab).unwrap().into_raw(),
            vector: self.vector as [f32; DIMENSIONS],
            distance: self.distance,
        }
    }
}

#[cfg(feature = "embedding")]
#[no_mangle]
pub extern "C" fn search (
    buffer: *const u8,
    buffer_length: usize,
    k: u8,
    callback: extern "C" fn(usize, *mut TopFFI<DIMENSIONS>)
) {
    let slice = read::<u8>(buffer, buffer_length);
    let mut top = match embedding::search::<DIMENSIONS>(slice, k) {
        Ok(t) => t,
        Err (e) => {
            #[cfg(debug_assertions)]
            println!( "[ERROR]: Search not found {:?}.\n{:?}", String::from_utf8(slice.to_vec()).unwrap_or_default(), e);
            vec![]       
        }
    };
    for (idx, t) in top.drain(..).enumerate() {
        println!("{:?}", t);
        ;
        callback(idx, Box::into_raw(Box::new(t.into())));
    };
}