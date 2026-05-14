//! Manages word embeddings, database storage, and similarity searches.
//!
//! This module provides the core functionality for handling word embeddings. It includes
//! utilities for:
//! - Connecting to and setting up a SQLite database with the `sqlite-vec` extension
//!   for vector similarity searches.
//! - Encoding words using various BPE tokenizers.
//! - Inserting word embeddings into the database.
//! - Performing nearest neighbor searches based on text or vector embeddings.
//! - Generating positional encodings for sequence models.

pub(crate) mod unit;
use rusqlite::{ffi::sqlite3_auto_extension, Connection};
use sqlite_vec::sqlite3_vec_init;
use std::sync::Once;
use zerocopy::{AsBytes};
use crate::bpe;
const PADDING: usize = 3; // The fixed size for token padding.
pub(crate) const DIMENSIONS: usize = 300;

/// Pads or truncates a vector to a fixed-size array of length 3.
/// 
/// If the input vector is shorter than `P`, it is padded at the beginning with
/// zeros. If it is longer than `P` or empty, an error is returned.
///
/// # Arguments
///
/// * `input`: The `Vec<u32>` to pad.
///
/// # Returns
///
/// A `Result` containing an array `[u32; P]` on success, or an error message
/// string if the input vector's length is invalid.
/// 
/// # Generic Parameters
/// 
/// * `P`: The desired size of the output array.
pub (crate) fn padding<const P: usize>(input: Vec<u32>) -> Result<[u32; P], &'static str> {
    let mut result = [0u32; P];
    if input.len() > P || input.is_empty() {
        return Err("Invalid token.");
    }
    // Pad with zeros at the beginning.
    result[P - input.len()..].copy_from_slice(&input);
    Ok(result)
}

/// Encodes a byte slice using the first available BPE vocabulary that works.
///
/// This function iterates through the supported vocabularies (`r50k`, `p50k`, etc.)
/// and attempts to tokenize the input `slice`. The first vocabulary that produces
/// a valid tokenization (i.e., one that can be padded correctly) is chosen.
///
/// # Returns
///
/// An `Option` containing a tuple with the vocabulary used, the string label, and the token vectors. Returns `None` if no vocabulary can encode the slice.
fn encode (slice: &[u8]) -> Option<(&bpe::vocabulary::Vocabularies, String, Vec<Vec<u32>>)> {
    let mut result = None;
    for vocab in bpe::vocabulary::Vocabularies::iter() {
        let tokens = match vocab {
            bpe::vocabulary::Vocabularies::R50K => bpe::encode(slice, &bpe::vocabulary::R50K_TOKENS),
            bpe::vocabulary::Vocabularies::P50K => bpe::encode(slice, &bpe::vocabulary::P50K_TOKENS),
            bpe::vocabulary::Vocabularies::CL100K => bpe::encode(slice, &bpe::vocabulary::CL100K_TOKENS),
            bpe::vocabulary::Vocabularies::O200K => bpe::encode(slice, &bpe::vocabulary::O200K_TOKENS)
        };
        let label = String::from_utf8(slice.to_vec()).expect("[ERROR]: Not a valid utf-8 string.");
        if let Err(_) = padding::<PADDING>(tokens.concat()) {
            #[cfg(debug_assertions)]
            println!(
                "[WARNING]: Could not tokenize {:?} with vocabulary {:?}.",
                label, vocab
            );
            continue;
        };
        result = Some((vocab, label, tokens));
        break;
    };
    result
}

static SQLITE_VEC_INIT: Once = Once::new();
/// Establishes a connection to a SQLite database and initializes the `sqlite-vec` extension.
///
/// This function opens a database connection, either to a file specified by `location` or
/// in-memory if `location` is `None`. It ensures that the `sqlite-vec` extension is
/// loaded exactly once per process and then executes the database schema defined in `schema.sql`.
///
/// # Arguments
///
/// * `location`: An optional string slice representing the path to the database file.
///   If `None`, an in-memory database is created.
///
/// # Returns
///
/// A `rusqlite::Connection` object.
///
/// # Panics
///
/// This function will panic if it fails to open the database or execute the schema.
pub (crate) fn connection(location: Option<&str>) -> Connection {
    SQLITE_VEC_INIT.call_once(|| {
        // This should only be called once per process.
        // SAFETY: `sqlite3_vec_init` is a valid extension entry point.
        unsafe { sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ()))) };
    });
    let connection = match location {
        Some(location) => Connection::open(location).unwrap_or_else(|_| {
            panic!("[ERROR]: Failed to open database in location {}", location)
        }),
        None => Connection::open_in_memory().expect("[ERROR]: Failed to open database in memory"),
    };
    let schema = include_str!("./schema.sql");
    connection
        .execute_batch(&schema)
        .expect("[ERROR]: Failed to execute schema");
    connection
}

/// Inserts a word, its vocabulary, and its embedding vector into the database.
///
/// The function attempts to encode the input `slice` (word) using all available
/// vocabularies. It inserts the first successful encoding that can be padded to the
/// required length (`PADDING`). If no vocabulary produces a valid tokenization,
/// no insertion is performed for that word.
///
/// # Arguments
///
/// * `conn`: A reference to the `rusqlite::Connection`.
/// * `slice`: The byte slice of the word to insert.
/// * `vector`: The embedding vector corresponding to the word.
///
/// # Returns
///
/// A `Result<(), rusqlite::Error>` indicating success or failure of the database operation.
///
/// # Panics
///
/// Panics if `slice` is empty or the `vector` length does not match `D`.
/// Also panics if the `slice` is not a valid UTF-8 string.
pub(crate) fn insert<const D: usize>(
    conn: &Connection,
    slice: &[u8],
    vector: &[f32; D],
) -> Result<(), rusqlite::Error> {
    if vector.len() != D || slice.is_empty() {
        panic!(
            "[ERROR]: Expecting non-empty slice and vector of length {:?}",
            D
        );
    };
    if let Some((vocab, label, _)) = encode(slice) {
        let mut stmt = conn.prepare_cached("INSERT INTO word_embeddings (label, vocab, vector) VALUES (?, ?, ?)")?;
        stmt.execute(rusqlite::params![label, vocab.to_string(), vector.as_bytes()])?;
    };
    Ok(())
}

#[derive(Debug)]
#[repr(align(16))] 
pub (crate)struct Row<const D: usize>{
    pub rid: u16,
    pub label: String,
    pub distance: f32,
    pub vector: [f32; D]
}

/// Calculates the positional encoding for a given position in a sequence.
///
/// Positional encodings are added to the input embeddings to provide the model
/// with information about the relative or absolute position of tokens.
///
/// # Arguments
///
/// * `position` - The position of the token in the sequence (e.g., 0, 1, 2, ...).
///
/// # Returns
///
/// An array `[f32; D]` representing the positional encoding for the given position.
pub(crate) fn position<const D: usize>(position: usize) -> [f32; D] {
    let mut pe = [0.0; D];
    let position = position as f32;
    let inv_base = 1.0 / 10000.0_f32;

    pe.chunks_mut(2).enumerate().for_each(|(i, chunk)| {
        // Calculate the division term for the current pair of dimensions.
        let div_term = inv_base.powf((i * 2) as f32 / D as f32);
        let angle = position * div_term;

        // The first element of the chunk gets the sine encoding.
        chunk[0] = angle.sin();

        // The second element gets the cosine encoding, if it exists.
        if let Some(elem) = chunk.get_mut(1) {
            *elem = angle.cos();
        }
    });
    pe
}

/// Searches for the `k` most similar words to a given text `slice` in the database.
///
/// This function uses the `sqlite-vec` extension's `MATCH` operator to perform a
/// similarity search. It queries a virtual table (`search`) that is designed for
/// efficient text-based lookups.
///
/// # Arguments
///
/// * `conn`: A reference to the `rusqlite::Connection`.
/// * `slice`: The byte slice of the text to search for.
/// * `k`: The number of nearest neighbors to retrieve.
///
/// # Returns
///
/// A `Result` containing a `Vec<Row<D>>` of the top `k` most similar items,
/// or a `rusqlite::Error` on failure.
///
/// # Panics
///
/// Panics if `k` is zero, `slice` is empty, or `slice` is not a valid UTF-8 string.
pub(crate) fn search<const D: usize>(
    conn: &Connection,
    slice: &[u8],
    k: u8,
) -> Result<Vec<Row<D>>, rusqlite::Error> {
    if k <= 0 || slice.is_empty() {
        panic!("[ERROR]: Expecting non-empty slice and non-zero k value");
    };
    let mut stmt = conn.prepare_cached("SELECT s.rid, s.label, w.vocab, s.rank as distance, e.vector FROM ( SELECT rid, label, rank FROM search WHERE label MATCH ? ORDER BY rank ASC LIMIT ?) AS s INNER JOIN words w ON s.rid = w.rid INNER JOIN embeddings e ON s.rid = e.rid ORDER BY s.rank ASC")?;
    let label = String::from_utf8(slice.to_vec()).expect("[ERROR]: Not a valid utf-8 string.");
    let result = stmt.query_map(rusqlite::params![label, k], |row| {
        Ok(Row {
            rid: row.get(0)?,
            label: row.get(1)?,
            distance: row.get(3)?,
            vector: {
                let bytes: Vec<u8> = row.get(4)?;
                let mut arr = [0.0; D];
                bytes
                    .chunks_exact(4)
                    .map(|a| f32::from_le_bytes(a.try_into().unwrap()))
                    .enumerate()
                    .for_each(|(i, f)| arr[i] = f);
                arr
            },
        })
    })?;

    result.collect()
}

/// Finds the `k` nearest neighbors to a given embedding vector.
///
/// This function performs a vector similarity search against the `embeddings` table
/// using the `sqlite-vec` extension. It identifies the database entries whose
/// vectors are closest to the provided `vector`.
///
/// # Arguments
///
/// * `conn`: A reference to the `rusqlite::Connection`.
/// * `vector`: The embedding vector to find neighbors for.
/// * `k`: The number of nearest neighbors to retrieve.
///
/// # Returns
///
/// A `Result` containing a `Vec<Row<D>>` of the top `k` nearest neighbors,
/// or a `rusqlite::Error` on failure.
///
/// # Panics
///
/// Panics if `k` is zero or the `vector` length does not match `D`.
pub(crate) fn nearest<const D: usize>(
    conn: &Connection,
    vector: &[f32; D],
    k: u8,
) -> Result<Vec<Row<D>>, rusqlite::Error> {
    if vector.len() != D || k <= 0 {
        panic!(
            "[ERROR]: Expecting a vector of length {:?} and non-zero k value",
            D
        );
    };
    let mut stmt = conn.prepare_cached("SELECT e.rid, s.label, w.vocab, e.distance, e.vector FROM ( SELECT rid, vector, distance FROM embeddings WHERE vector MATCH ? ORDER BY distance ASC LIMIT ?) AS e INNER JOIN words w ON e.rid = w.rid INNER JOIN search s ON e.rid = s.rid ORDER BY e.distance ASC")?;
    let result = stmt.query_map(rusqlite::params![vector.as_bytes(), k], |row| {
        Ok(Row {
            rid: row.get(0)?,
            label: row.get(1)?,
            distance: row.get(3)?,
            vector: {
                let bytes: Vec<u8> = row.get(4)?;
                let mut arr = [0.0; D];
                bytes
                    .chunks_exact(4)
                    .map(|a| f32::from_le_bytes(a.try_into().unwrap()))
                    .enumerate()
                    .for_each(|(i, f)| arr[i] = f);
                arr
            },
        })
    })?;

    result.collect()
}
