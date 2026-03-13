use std::env;
use rusqlite::{ffi::sqlite3_auto_extension, Connection};
use sqlite_vec::{sqlite3_vec_init};
use zerocopy::AsBytes;

pub (crate) fn connection(location: Option<&str>) -> Connection {
    unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
    }
    let connection = match location {
        Some(location) => Connection::open(location).unwrap(),
        None => Connection::open("./embeddings.db").unwrap(),
    };
    let schema = include_str!("./schema.sql");
    connection.execute_batch(&schema).unwrap();
    connection
}

fn padding(input: Vec<u32>) -> Result<[u32; 3], &'static str> {
    let mut result = [0u32; 3];
    let len = input.len();

    if len > 3 {
        return Err("Token value too large. Expect 3 or less.")
    }
    // Pad with zeros at the beginning.
    result[3 - len..].copy_from_slice(&input);
    Ok(result)
}

pub(crate) fn embed(
    token: Vec<u32>,
    vocabulary: &str,
    label: &str,
    embedding: &[f32],
) -> Result<usize, rusqlite::Error>  {
    let location = env::var("EMBEDDING_LOCATION").ok();
    let conn = connection(location.as_deref());
    let qembedding = embedding.iter().map(|i| {i.signum() as u8}).collect::<Vec<u8>>();
    let mut stmt = conn.prepare_cached(
        "INSERT INTO embeddings (token, vocabulary, label, embedding, qembedding) VALUES (?, ?, ?, ?, ?)",
    )?;
    if let Ok(padded) = padding(token) { 
        let response = stmt.execute(rusqlite::params![padded.as_bytes(), vocabulary, label, embedding.as_bytes(), qembedding.as_slice().as_bytes()])?;
        return Ok(response)
    } else {
        #[cfg(debug_assertions)]
        println!( "[WARNING]: {:?} Token for {:?} is too long. Expect 3 or less.",vocabulary, label);
        return Ok(0)
    }
}