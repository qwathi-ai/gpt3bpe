CREATE TABLE IF NOT EXISTS words (
    rid INTEGER PRIMARY KEY AUTOINCREMENT,
    vocab TEXT NOT NULL CHECK (vocab IN ('P50K','R50K','CL100K','O200K'))
    -- terminals TEXT[]
);

CREATE VIRTUAL TABLE IF NOT EXISTS embeddings using vec0 (
    rid INTEGER PRIMARY KEY,
    vector FLOAT[300]
);

CREATE VIRTUAL TABLE IF NOT EXISTS search USING fts5 (
    rid, 
    label,
    tokenize='trigram'
);

CREATE VIEW IF NOT EXISTS word_embeddings AS
SELECT 
    w.rid, 
    s.label,
    w.vocab,
    e.vector 
FROM words w
INNER JOIN search s 
    ON w.rid = s.rid
JOIN embeddings e 
    ON w.rid = e.rid;

CREATE TRIGGER IF NOT EXISTS trg_insert_word_embeddings
INSTEAD OF INSERT ON word_embeddings
BEGIN
    INSERT OR ROLLBACK INTO words (vocab) 
    VALUES (new.vocab);
    
    INSERT OR ROLLBACK INTO embeddings (rid, vector) 
    VALUES (last_insert_rowid(), new.vector);

    INSERT OR ROLLBACK INTO search (rid, label) 
    VALUES (last_insert_rowid(), new.label);
END;

CREATE TRIGGER IF NOT EXISTS trg_delete_word_embeddings 
INSTEAD OF DELETE ON word_embeddings 
BEGIN
  DELETE FROM words WHERE rid = old.rid;
  DELETE FROM embeddings WHERE rid = old.rid;
  DELETE FROM search WHERE rid = old.rid;
END;
