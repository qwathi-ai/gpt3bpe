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

pub(crate) fn insert(
    token: [u32;3],
    vocabulary: &str,
    label: &str,
    embedding: &[f32],
) -> Result<usize, rusqlite::Error>  {
    let location = env::var("EMBEDDING_LOCATION").ok();
    let conn = connection(location.as_deref());
    if token.len() > 3 {
        panic!("Token length for {:?} is too large", label);
    }
    let qembedding = embedding.iter().map(|i| {i.signum() as u8}).collect::<Vec<u8>>();
    let mut stmt = conn.prepare_cached(
        "INSERT INTO embeddings (token, vocabulary, label, embedding, qembedding) VALUES (?, ?, ?, ?, ?)",
    )?;

    let response = stmt.execute(rusqlite::params![token.as_bytes(), vocabulary, label, embedding.as_bytes(), qembedding.as_slice().as_bytes()])?;
    Ok(response)
}