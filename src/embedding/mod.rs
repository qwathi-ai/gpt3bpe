use std::env;
use rusqlite::{ffi::sqlite3_auto_extension, Connection};
use sqlite_vec::{sqlite3_vec_init};
use zerocopy::AsBytes;

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
fn padding(input: Vec<u32>) -> Result<[u32; 3], &'static str> {
    let mut result = [0u32; 3];
    if input.len() > 3 {
        return Err("Token value too large. Expect 3 or less.")
    }
    // Pad with zeros at the beginning.
    result[3 - input.len()..].copy_from_slice(&input);
    Ok(result)
}

pub (crate) fn connection(location: Option<&str>) -> Connection {
    unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
    }
    let connection = match location {
        Some(location) => Connection::open(location).unwrap_or_else(|_| panic!("[ERROR]: Failed to open database in location {}", location)),
        None => Connection::open("./embeddings.db").expect("[ERROR]: Failed to open database"),
    };
    let schema = include_str!("./schema.sql");
    connection.execute_batch(&schema).expect("[ERROR]: Failed to execute schema");
    connection
}

pub(crate) fn embed(
    slice: &[u8],
    vector: &[f32],
) -> Result<usize, rusqlite::Error>  {
    let location = env::var("EMBEDDING_LOCATION").ok();
    let conn = connection(location.as_deref());
    let mut response: usize = 0;
    for vocab in crate::bpe::vocabulary::Vocabularies::iter() {
        let (v, tokens) = match vocab {
            crate::bpe::vocabulary::Vocabularies::R50K => ("R50K", crate::bpe::encode(slice, &crate::bpe::vocabulary::R50K_TOKENS).concat()),
            crate::bpe::vocabulary::Vocabularies::P50K => ("P50K", crate::bpe::encode(slice, &crate::bpe::vocabulary::P50K_TOKENS).concat()),
            crate::bpe::vocabulary::Vocabularies::CL100K => ("CL100K",crate::bpe::encode(slice, &crate::bpe::vocabulary::CL100K_TOKENS).concat()),
            crate::bpe::vocabulary::Vocabularies::O200K => ("O200K", crate::bpe::encode(slice, &crate::bpe::vocabulary::O200K_TOKENS).concat()),
        };
        let label = String::from_utf8(slice.to_vec()).expect("[ERROR]: Not a valid utf-8 string.");
        if let Err(_) = padding(tokens) {
            #[cfg(debug_assertions)]
            println!("[WARNING]: Could not embed {:?}, token too long for vocabulary {:?}.", label, vocab);
            continue;
        };
        let mut stmt = conn.prepare_cached(
            "INSERT INTO word_embeddings (label, vocab, vector) VALUES (?, ?, ?)",
        ).expect("[ERROR]: Failed to prepare statement");

        response = stmt.execute(rusqlite::params![
            label,
            v,
            vector.as_bytes()
        ]).expect("[ERROR]: Failed to insert embedding");
        println!("[INFO]: {:?} embedded using vocabulary {:?}.", label, vocab);

        break;
    }
    Ok(response)
}

pub(crate) struct Top {
    rid: u16,
    label: &str,
    vocab: &str,
    vector: [f32],
    distance: f32
}

pub(crate) fn search(
    slice: &[u8],
    k: u8,
) -> Result<Vec<Top>, rusqlite::Error>  {
    let location = env::var("EMBEDDING_LOCATION").ok();
    let conn = connection(location.as_deref());
    let mut stmt = conn.prepare_cached("SELECT s.rid, s.label, w.vocab, e.vector, s.rank as distance FROM ( SELECT rid, label, rank FROM search WHERE label MATCH ? ORDER BY rank ASC LIMIT ?) AS s INNER JOIN words w ON s.rid = w.rid INNER JOIN embeddings e ON s.rid = e.rid ORDER BY s.rank ASC")?;
    let label = String::from_utf8(slice.to_vec()).expect("[ERROR]: Not a valid utf-8 string.");
    let result = stmt.query_map(
        rusqlite::params![
            label,
            k
        ],
        |row| {
            Ok(Top {
                rid: row.get(0)?,
                label: row.get(1)?,
                vocab: row.get(2)?,
                vector: row.get(3)?,
                distance: row.get(4)?
            })
        },
    )?;

    result.collect()
}

pub(crate) fn top(
    vector: &[f32],
    k: u8,
) -> Result<Vec<Top>, rusqlite::Error>  {
    let location = env::var("EMBEDDING_LOCATION").ok();
    let conn = connection(location.as_deref());
    let mut stmt = conn.prepare_cached("SELECT e.rid, s.label, w.vocab, e.vector, e.distance FROM ( SELECT rid, vector, distance FROM embeddings WHERE vector MATCH ? ORDER BY distance ASC LIMIT ?) AS e INNER JOIN words w ON e.rid = w.rid INNER JOIN search s ON e.rid = s.rid ORDER BY e.distance ASC")?;
    let result = stmt.query_map(
        rusqlite::params![
            vector.as_bytes(),
            k
        ],
        |row| {
            Ok(Top {
                rid: row.get(0)?,
                label: row.get(1)?,
                vocab: row.get(2)?,
                vector: row.get(3)?,
                distance: row.get(4)?
            })
        },
    )?;

    result.collect()
}
