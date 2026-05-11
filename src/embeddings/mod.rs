pub(crate) mod unit;
use rusqlite::{ffi::sqlite3_auto_extension, Connection};
use sqlite_vec::sqlite3_vec_init;
use std::sync::Once;
use zerocopy::{AsBytes};
use std::simd::f32x4;

const PADDING: usize = 3;
pub(crate) const DIMENSIONS: usize = 300;
pub(crate) const SIMD_WIDTH: usize = 4;
pub(crate) const SIMD_VECTORS: usize = DIMENSIONS / SIMD_WIDTH; // 300 / 4 = 75

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

    unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
    }
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
    for vocab in crate::bpe::vocabulary::Vocabularies::iter() {
        let (v, tokens) = match vocab {
            crate::bpe::vocabulary::Vocabularies::R50K => (
                "R50K",
                crate::bpe::encode(slice, &crate::bpe::vocabulary::R50K_TOKENS).concat(),
            ),
            crate::bpe::vocabulary::Vocabularies::P50K => (
                "P50K",
                crate::bpe::encode(slice, &crate::bpe::vocabulary::P50K_TOKENS).concat(),
            ),
            crate::bpe::vocabulary::Vocabularies::CL100K => (
                "CL100K",
                crate::bpe::encode(slice, &crate::bpe::vocabulary::CL100K_TOKENS).concat(),
            ),
            crate::bpe::vocabulary::Vocabularies::O200K => (
                "O200K",
                crate::bpe::encode(slice, &crate::bpe::vocabulary::O200K_TOKENS).concat(),
            ),
        };
        let label = String::from_utf8(slice.to_vec()).expect("[ERROR]: Not a valid utf-8 string.");
        if let Err(_) = padding::<PADDING>(tokens) {
            #[cfg(debug_assertions)]
            println!(
                "[WARNING]: Could not embed {:?} for vocabulary {:?}.",
                label, vocab
            );
            continue;
        };
        let mut stmt = conn.prepare_cached("INSERT INTO word_embeddings (label, vocab, vector) VALUES (?, ?, ?)")?;
        stmt.execute(rusqlite::params![label, v, vector.as_bytes()])?;
        break;
    }
    Ok(())
}

#[derive(Debug)]
#[repr(align(16))] 
pub (crate)struct Row {
    pub rid: u16,
    pub label: String,
    pub distance: f32,
    // The vector is now an array of SIMD vectors
    pub vector: Option<[std::simd::f32x4; SIMD_VECTORS]>,
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
/// An array of SIMD vectors `[std::simd::f32x4; SIMD_VECTORS]` that represents
/// the positional encoding. The total number of floats is equal to `DIMENSIONS`.
pub(crate) fn position(position: usize) -> [std::simd::f32x4; SIMD_VECTORS] {
    let mut pe_array = [0.0; DIMENSIONS];
    let position = position as f32;

    for i in (0..DIMENSIONS).step_by(2) {
        let div_term = (10000.0_f32.powf(i as f32 / DIMENSIONS as f32)).recip();
        let angle = position * div_term;
        pe_array[i] = angle.sin();
        if i + 1 < DIMENSIONS {
            pe_array[i + 1] = angle.cos();
        }
    }
    // Safely cast the `[f32; 300]` array into an array of SIMD vectors `[f32x4; 75]`.
    let (prefix, simd_slice, suffix) = unsafe { pe_array.align_to::<f32x4>() };
    assert!(prefix.is_empty());
    assert!(suffix.is_empty());
    simd_slice.try_into().unwrap()
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
pub(crate) fn search(
    conn: &Connection,
    slice: &[u8],
    k: u8,
) -> Result<Vec<Row>, rusqlite::Error> {
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
                let (prefix, simd_slice, suffix) = unsafe { bytes.align_to::<f32x4>() };
                assert!(prefix.is_empty());
                assert!(suffix.is_empty());
                simd_slice.try_into().ok()
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
) -> Result<Vec<Row>, rusqlite::Error> {
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
                let (prefix, simd_slice, suffix) = unsafe { bytes.align_to::<f32x4>() };
                assert!(prefix.is_empty());
                assert!(suffix.is_empty());
                simd_slice.try_into().ok()
            },
        })
    })?;

    result.collect()
}
