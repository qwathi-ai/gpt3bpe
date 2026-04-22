#![cfg(test)]
use crate::embedding::*;
use rusqlite::{Connection, Result};
use std::{
    env,
    sync::{Arc, LazyLock, Mutex},
};

#[test]
fn test_padding_smaller() {
    let input = vec![1, 2];
    let expected = [0, 1, 2];
    assert_eq!(padding::<3>(input).unwrap(), expected);
}

#[test]
fn test_padding_equal() {
    let input = vec![1, 2, 3];
    let expected = [1, 2, 3];
    assert_eq!(padding::<3>(input).unwrap(), expected);
}

#[test]
fn test_padding_larger() {
    let input = vec![1, 2, 3, 4];
    assert!(padding::<3>(input).is_err());
}

// static CONNECTION: LazyLock<Arc<Mutex<Connection>>> = LazyLock::new(|| {
//     let location = env::var("EMBEDDING_LOCATION").ok();
//     Arc::new(Mutex::new(connection(location.as_deref())))
// });

// #[test]
// fn test_search_empty() -> Result<()> {
//     let conn = &CONNECTION.lock().unwrap();
//     let result = search::<DIMENSIONS>(&conn, b"", 5)?;
//     assert!(result.is_empty());
//     Ok(())
// }

// #[test]
// fn test_search() -> Result<()> {
//     let conn = &CONNECTION.lock().unwrap();
//     let vector = [0.1; DIMENSIONS];
//     conn.execute(
//         "INSERT INTO word_embeddings (label, vocab, vector) VALUES (?1, ?2, ?3)",
//         rusqlite::params!["say", "R50K", vector.as_bytes()],
//     )?;
//     conn.execute(
//         "INSERT INTO words (rid, label) VALUES (?1, ?2)",
//         rusqlite::params![1, "say"],
//     )?;
//     conn.execute(
//         "INSERT INTO embeddings (rid, vector) VALUES (?1, ?2)",
//         rusqlite::params![1, vector.as_bytes()],
//     )?;
//     conn.execute(
//         "CREATE VIRTUAL TABLE search USING fts5(label, rid)",
//         rusqlite::params![],
//     )?;
//     conn.execute(
//         "INSERT INTO search (label, rid) VALUES (?1, ?2)",
//         rusqlite::params!["say", 1],
//     )?;

//     let result = search::<DIMENSIONS>(&conn, b"say", 1)?;
//     assert_eq!(result.len(), 1);
//     assert_eq!(result[0].label, "say");
//     Ok(())
// }

// #[test]
// fn test_top_empty() -> Result<()> {
//     let conn = &CONNECTION.lock().unwrap();
//     let vector = [0.0f32; DIMENSIONS];
//     let result = top::<DIMENSIONS>(&conn, &vector, 5)?;
//     assert!(result.is_empty());
//     Ok(())
// }

// #[test]
// fn test_top() -> Result<()> {
//     let conn = &CONNECTION.lock().unwrap();
//     let vector1 = [0.1; DIMENSIONS];
//     let vector2 = [0.2; DIMENSIONS];
//     conn.execute(
//         "INSERT INTO word_embeddings (label, vocab, vector) VALUES (?1, ?2, ?3)",
//         rusqlite::params!["vector1", "R50K", vector1.as_bytes()],
//     )?;
//     conn.execute(
//         "INSERT INTO word_embeddings (label, vocab, vector) VALUES (?1, ?2, ?3)",
//         rusqlite::params!["vector2", "R50K", vector2.as_bytes()],
//     )?;
//     conn.execute(
//         "INSERT INTO words (rid, label) VALUES (?1, ?2)",
//         rusqlite::params![1, "vector1"],
//     )?;
//     conn.execute(
//         "INSERT INTO words (rid, label) VALUES (?1, ?2)",
//         rusqlite::params![2, "vector2"],
//     )?;
//     conn.execute(
//         "INSERT INTO embeddings (rid, vector) VALUES (?1, ?2)",
//         rusqlite::params![1, vector1.as_bytes()],
//     )?;
//     conn.execute(
//         "INSERT INTO embeddings (rid, vector) VALUES (?1, ?2)",
//         rusqlite::params![2, vector2.as_bytes()],
//     )?;
//     conn.execute(
//         "CREATE VIRTUAL TABLE search USING fts5(label, rid)",
//         rusqlite::params![],
//     )?;
//     conn.execute(
//         "INSERT INTO search (label, rid) VALUES (?1, ?2)",
//         rusqlite::params!["vector1", 1],
//     )?;
//     conn.execute(
//         "INSERT INTO search (label, rid) VALUES (?1, ?2)",
//         rusqlite::params!["vector2", 2],
//     )?;

//     let result = top::<DIMENSIONS>(&conn, &vector1, 1)?;
//     assert_eq!(result.len(), 1);
//     assert_eq!(result[0].label, "vector1");
//     Ok(())
// }
