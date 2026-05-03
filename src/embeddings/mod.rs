mod unit;
use rusqlite::{ffi::sqlite3_auto_extension, Connection};
use sqlite_vec::sqlite3_vec_init;
use std::{
    env,
    sync::{Arc, LazyLock, Mutex},
};
use zerocopy::AsBytes;
const PADDING: usize = 3;
pub(crate) const DIMENSIONS: usize = 300;

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
pub (crate) fn padding<const P: usize>(input: Vec<u32>) -> Result<[u32; P], &'static str> {
    let mut result = [0u32; P];
    if input.len() > P || input.is_empty() {
        return Err("Invalid token.");
    }
    // Pad with zeros at the beginning.
    result[P - input.len()..].copy_from_slice(&input);
    Ok(result)
}

pub (crate) fn connection(location: Option<&str>) -> Connection {
    unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
    }
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

pub (crate) static CONNECTION: LazyLock<Arc<Mutex<Connection>>> = LazyLock::new(|| {
    let location = match env::var("EMBEDDING_LOCATION") {
        Ok(l) => l,
        Err(_) => {
            #[cfg(debug_assertions)]
            println!(  "[WARNING]: `EMBEDDING_LOCATION` environment variable not set. Defaulting to `./embeddings.db`" );
            "./embeddings.db".to_string()
        }
    };
    Arc::new(Mutex::new(connection(
        Some(location).as_ref().map(|x| x.as_str()),
    )))
});


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
        let mut stmt = conn
            .prepare_cached("INSERT INTO word_embeddings (label, vocab, vector) VALUES (?, ?, ?)")
            .expect("[ERROR]: Failed to prepare statement");

        stmt.execute(rusqlite::params![label, v, vector.as_bytes()])
            .expect("[ERROR]: Failed to insert embedding");
        break;
    }
    Ok(())
}

#[derive(Debug)]
struct Top<const D: usize> {
    rid: u16,
    pub label: String,
    vocab: String,
    pub distance: f32,
    pub vector: [f32; D],
}

pub(crate) fn search<const D: usize>(
    conn: &Connection,
    slice: &[u8],
    k: u8,
) -> Result<Vec<Top<D>>, rusqlite::Error> {
    if k <= 0 || slice.is_empty() {
        panic!(
            "[ERROR]: Expecting non-empty slice and non-zero k value"
        );
    };
    let mut stmt = conn.prepare_cached("SELECT s.rid, s.label, w.vocab, s.rank as distance, e.vector FROM ( SELECT rid, label, rank FROM search WHERE label MATCH ? ORDER BY rank ASC LIMIT ?) AS s INNER JOIN words w ON s.rid = w.rid INNER JOIN embeddings e ON s.rid = e.rid ORDER BY s.rank ASC")?;
    let label = String::from_utf8(slice.to_vec()).expect("[ERROR]: Not a valid utf-8 string.");
    let result = stmt.query_map(rusqlite::params![label, k], |row| {
        Ok(Top {
            rid: row.get(0)?,
            label: row.get(1)?,
            vocab: row.get(2)?,
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

pub(crate) fn top<const D: usize>(
    conn: &Connection,
    vector: &[f32; D],
    k: u8,
) -> Result<Vec<Top<D>>, rusqlite::Error> {
    if vector.len() != D || k <= 0 {
        panic!(
            "[ERROR]: Expecting a vector of length {:?} and non-zero k value",
            D
        );
    };
    let mut stmt = conn.prepare_cached("SELECT e.rid, s.label, w.vocab, e.distance, e.vector FROM ( SELECT rid, vector, distance FROM embeddings WHERE vector MATCH ? ORDER BY distance ASC LIMIT ?) AS e INNER JOIN words w ON e.rid = w.rid INNER JOIN search s ON e.rid = s.rid ORDER BY e.distance ASC")?;
    let result = stmt.query_map(rusqlite::params![vector.as_bytes(), k], |row| {
        Ok(Top {
            rid: row.get(0)?,
            label: row.get(1)?,
            vocab: row.get(2)?,
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
