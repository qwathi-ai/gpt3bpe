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
        let tokens = match vocab {
            crate::bpe::vocabulary::Vocabularies::R50K => crate::bpe::encode(slice, &crate::bpe::vocabulary::R50K_TOKENS).concat(),
            crate::bpe::vocabulary::Vocabularies::P50K => crate::bpe::encode(slice, &crate::bpe::vocabulary::P50K_TOKENS).concat(),
            crate::bpe::vocabulary::Vocabularies::CL100K => crate::bpe::encode(slice, &crate::bpe::vocabulary::CL100K_TOKENS).concat(),
            crate::bpe::vocabulary::Vocabularies::O200K => crate::bpe::encode(slice, &crate::bpe::vocabulary::O200K_TOKENS).concat(),
        };
        let label = String::from_utf8(slice.to_vec()).unwrap();
        if let Err(_) = padding(tokens) {
            #[cfg(debug_assertions)]
            println!("[WARNING]: Could not embed {:?}, token too long for vocabulary {:?}.", label, vocab);
            continue;
        };
        let mut stmt = conn.prepare_cached(
            "INSERT INTO word_embeddings (label, vector) VALUES (?, ?)",
        ).expect("Failed to prepare statement");

        response = stmt.execute(rusqlite::params![
            label, // `label` is defined above
            vector.as_bytes() // `vector` is passed into the function
        ]).expect("[ERROR]: Failed to insert embedding");
        println!("[INFO]: {:?} embedded using vocabulary {:?}. Response: {:?}", label, vocab, response);

        break;
    }
    Ok(response)
}

#[derive(Debug)]
struct Top {
    pub label: String,
    distance: f32,
}

pub(crate) fn top(
    vector: &[f32],
    k: Option<u8>,
) -> Result<Vec<Top>, rusqlite::Error>  {
    let location = env::var("EMBEDDING_LOCATION").ok();
    let conn = connection(location.as_deref());
    // The 'k' parameter in the MATCH query is a sqlite-vec specific feature
    // for specifying the number of nearest neighbors to return.
    let mut stmt = conn.prepare_cached("SELECT label, distance FROM embeddings WHERE vector MATCH ? ORDER BY distance LIMIT ?")?;
    let top_iter = stmt.query_map(
        rusqlite::params![
            vector.as_bytes(),
            k.unwrap_or(10) as u8
        ],
        |row| {
            Ok(Top {
                label: row.get(0)?,
                distance: row.get(1)?,
            })
        },
    )?;

    top_iter.collect()
}