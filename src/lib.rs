//! # GPT Byte-Pair Encoding and Embeddings Library (C FFI)
//!
//! This library provides a C-compatible Foreign Function Interface (FFI) for performing
//! Byte-Pair Encoding (BPE) tokenization and vector embedding similarity searches. It is
//! designed to be compiled as a dynamic library and linked against other programming
//! languages like C, Python, or Node.js.
//!
//! ## Features
//!
//! - **BPE Tokenization**: Functions to encode text into tokens and decode tokens back into
//!   text using standard GPT vocabularies (`r50k`, `p50k`, `cl100k`, `o200k`).
//! - **Grapheme Splitting**: A utility to split text into its base Unicode grapheme clusters.
//! - **Vector Embeddings**: (Optional feature) Functions to interact with a SQLite database
//!   for storing and searching high-dimensional word embeddings.
//!   - Insert embeddings for a given text.
//!   - Search for the most similar texts based on a query text.
//!   - Find the nearest neighbors to a given vector embedding.
//!
//! ## Safety
//!
//! The functions exported by this library operate on raw pointers and are therefore `unsafe`.
//! Callers are responsible for ensuring that all pointers are valid, non-null, and point to
//! memory regions of the specified length. The library includes assertions to catch null
//! pointers and potential overflows, but the ultimate responsibility for memory safety
//! lies with the caller.
mod bpe;

/// Safely reads data from a raw pointer into a static slice.
///
/// This internal helper function wraps an `unsafe` block to create a slice from a
/// pointer and a length provided by the C FFI. It includes several assertions to
/// prevent common memory safety issues.
///
/// # Arguments
///
/// * `pointer` - A raw pointer to the data.
/// * `length` - The number of elements to read.
///
/// # Panics
///
/// This function will panic if:
/// * The pointer is null.
/// * The pointer is not properly aligned for type `T`.
/// * The requested length could lead to a buffer overflow.
/// * The length of the created slice does not match the requested length.
///
/// # Safety
///
/// The caller must guarantee that the `pointer` is valid for reads of `length` elements
/// of type `T`. The lifetime of the returned slice is `'static`, which means the caller
/// is responsible for ensuring the pointed-to data outlives the slice. This is a common
/// pattern in FFI but requires careful memory management on the C side.
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

/// Splits a byte buffer into grapheme clusters.
///
/// # Arguments
///
/// * `buffer` - A raw pointer to the byte buffer.
/// * `length` - The length of the buffer.
/// * `callback` - A C-compatible function that is called for each byte of the resulting
///   grapheme clusters. It receives the index and the byte value.
///
/// # Safety
/// The `buffer` pointer must be valid for `length` bytes.
#[no_mangle]
pub extern "C" fn grapheme(buffer: *const u8, length: usize, callback: extern "C" fn(usize, u8)) {
    let slice = read(buffer, length);

    let grapheme = bpe::grapheme(slice);
    for (idx, value) in grapheme.concat().drain(..).enumerate() {
        callback(idx, value)
    }
}

/// Encodes a byte buffer using the r50k vocabulary.
///
/// # Arguments
///
/// * `buffer` - A raw pointer to the byte buffer.
/// * `length` - The length of the buffer.
/// * `callback` - A C-compatible function that is called for each resulting token. It
///   receives the index and the token value.
///
/// # Safety
/// The `buffer` pointer must be valid for `length` bytes.
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

/// Decodes a buffer of r50k tokens into bytes.
///
/// # Arguments
///
/// * `buffer` - A raw pointer to the token buffer.
/// * `length` - The length of the buffer.
/// * `callback` - A C-compatible function that is called for each resulting byte. It
///   receives the index and the byte value.
///
/// # Safety
/// The `buffer` pointer must be valid for `length` `u16` tokens.
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

/// Encodes a byte buffer using the p50k vocabulary.
///
/// # Arguments
///
/// * `buffer` - A raw pointer to the byte buffer.
/// * `length` - The length of the buffer.
/// * `callback` - A C-compatible function that is called for each resulting token. It
///   receives the index and the token value.
///
/// # Safety
/// The `buffer` pointer must be valid for `length` bytes.
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


/// Decodes a buffer of p50k tokens into bytes.
///
/// # Arguments
///
/// * `buffer` - A raw pointer to the token buffer.
/// * `length` - The length of the buffer.
/// * `callback` - A C-compatible function that is called for each resulting byte. It
///   receives the index and the byte value.
///
/// # Safety
/// The `buffer` pointer must be valid for `length` `u16` tokens.
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

/// Encodes a byte buffer using the cl100k vocabulary.
///
/// # Arguments
///
/// * `buffer` - A raw pointer to the byte buffer.
/// * `length` - The length of the buffer.
/// * `callback` - A C-compatible function that is called for each resulting token. It
///   receives the index and the token value.
///
/// # Safety
/// The `buffer` pointer must be valid for `length` bytes.
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


/// Decodes a buffer of cl100k tokens into bytes.
///
/// # Arguments
///
/// * `buffer` - A raw pointer to the token buffer.
/// * `length` - The length of the buffer.
/// * `callback` - A C-compatible function that is called for each resulting byte. It
///   receives the index and the byte value.
///
/// # Safety
/// The `buffer` pointer must be valid for `length` `u32` tokens.
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

/// Encodes a byte buffer using the o200k vocabulary.
///
/// # Arguments
///
/// * `buffer` - A raw pointer to the byte buffer.
/// * `length` - The length of the buffer.
/// * `callback` - A C-compatible function that is called for each resulting token. It
///   receives the index and the token value.
///
/// # Safety
/// The `buffer` pointer must be valid for `length` bytes.
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


/// Decodes a buffer of o200k tokens into bytes.
///
/// # Arguments
///
/// * `buffer` - A raw pointer to the token buffer.
/// * `length` - The length of the buffer.
/// * `callback` - A C-compatible function that is called for each resulting byte. It
///   receives the index and the byte value.
///
/// # Safety
/// The `buffer` pointer must be valid for `length` `u32` tokens.
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

/// Inserts a text and its corresponding embedding vector into the database.
///
/// This function is only available when the `embeddings` feature is enabled.
///
/// # Arguments
///
/// * `buffer` - A raw pointer to the byte buffer representing the text.
/// * `buffer_length` - The length of the text buffer.
/// * `vector` - A raw pointer to the embedding vector.
/// * `vector_length` - The length of the vector.
///
/// # Returns
///
/// Returns `true` if the insertion was successful or if the text already exists in the database, `false` otherwise.
///
/// # Panics
///
/// This function will panic if the `EMBEDDINGS` environment variable is not set to the
/// path of the SQLite database file.
///
/// # Safety
///
/// This function will panic if the `EMBEDDINGS` environment variable is not set.
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

/// Searches for the most similar embeddings to a given text.
///
/// This function is only available when the `embeddings` feature is enabled.
///
/// # Arguments
///
/// * `buffer` - A raw pointer to the byte buffer representing the text.
/// * `buffer_length` - The length of the text buffer.
/// * `k` - The number of nearest neighbors to retrieve.
/// * `callback` - A C-compatible function that is called for each element of the resulting
///   vectors. It receives the row ID, distance, vector position, and vector value.
///
/// # Panics
///
/// This function will panic if the `EMBEDDINGS` environment variable is not set to the
/// path of the SQLite database file.
///
/// # Safety
/// This function will panic if the `EMBEDDINGS` environment variable is not set.
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
    let mut top =
        embeddings::search::<{embeddings::DIMENSIONS}>(&embeddings::connection(location.as_deref()), slice, k).unwrap();
    for (rid, row) in top.drain(..).enumerate() {
        for (position, value) in row.vector.iter().enumerate() {
            callback(rid, row.distance, position, *value);
        };
    };
}

/// Finds the nearest neighbors to a given embedding vector.
///
/// This function is only available when the `embeddings` feature is enabled.
///
/// # Arguments
///
/// * `vector` - A raw pointer to the embedding vector.
/// * `vector_length` - The length of the vector.
/// * `k` - The number of nearest neighbors to retrieve.
/// * `callback` - A C-compatible function that is called for each byte of the resulting
///   labels. It receives the row ID, distance, label length, byte position, and byte value.
///
/// # Panics
///
/// This function will panic if the `EMBEDDINGS` environment variable is not set to the
/// path of the SQLite database file.
///
/// # Safety
/// This function will panic if the `EMBEDDINGS` environment variable is not set.
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
    for (rid, row) in top.drain(..).enumerate() {
        let bytes = row.label.as_bytes();
        let len = bytes.len();
        for (position, value) in bytes.iter().enumerate() {
            callback( rid, row.distance, len, position, *value);
        };
    };
}