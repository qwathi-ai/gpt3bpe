use std::env;
use rusqlite::{ffi::sqlite3_auto_extension, Connection};
use sqlite_vec::{sqlite3_vec_init};
use zerocopy::AsBytes;

pub (crate) fn connection(location: Option<&str>) -> Connection {
    unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
    }
    let connection = match location {
        Some(location) => Connection::open(location).expect("Failed to open database"),
        None => Connection::open("./embeddings.db").expect("Failed to open database"),
    };
    let schema = include_str!("./schema.sql");
    connection.execute_batch(&schema).expect("Failed to execute schema");
    connection
}

pub(crate) fn embed(
    label: &str,
    vector: &[f32],
) -> Result<usize, rusqlite::Error>  {
    let location = env::var("EMBEDDING_LOCATION").ok();
    let conn = connection(location.as_deref());
    let mut stmt = conn.prepare_cached(
        "INSERT INTO embeddings (label, vector) VALUES (?, ?)",
    ).expect("Failed to prepare statement");
    let response = stmt.execute(rusqlite::params![
        label,
        vector.as_bytes()
    ])
        .expect("[ERROR]: Failed to insert embedding");
    Ok(response)
}