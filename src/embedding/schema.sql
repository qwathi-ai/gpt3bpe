CREATE TABLE IF NOT EXISTS words (
    rid INTEGER PRIMARY KEY AUTOINCREMENT,
    label TEXT NOT NULL UNIQUE,
    vocab TEXT NOT NULL CHECK (vocab IN ('P50K','R50K','CL100K','O200K'))
    -- terminals TEXT[]
);

CREATE VIRTUAL TABLE IF NOT EXISTS embeddings using vec0 (
    rid INTEGER PRIMARY KEY,
    vector FLOAT[300]
);

CREATE VIRTUAL TABLE IF NOT EXISTS search USING fts5 (
    label,
    content='', 
    content_rowid='rid',
    tokenize='trigram'
);

CREATE VIEW IF NOT EXISTS word_embeddings AS
SELECT 
    w.rid, 
    w.label,
    w.vocab,
    e.vector 
FROM words w
JOIN embeddings e ON w.rid = e.rid;

CREATE TRIGGER IF NOT EXISTS trg_insert_word_embeddings
INSTEAD OF INSERT ON word_embeddings
BEGIN
    INSERT OR ROLLBACK INTO words (label, vocab) 
    VALUES (new.label, new.vocab);
    
    INSERT OR ROLLBACK INTO embeddings (rid, vector) 
    VALUES (last_insert_rowid(), new.vector);

    INSERT OR ROLLBACK INTO search (rowid, label) 
    VALUES (last_insert_rowid(), new.label);
END;

-- CREATE TRIGGER IF NOT EXISTS trg_delete_word_embeddings 
-- AFTER DELETE ON words 
-- BEGIN
--   INSERT INTO words_fts(words_fts, rowid, label) VALUES('delete', old.id, old.label);
-- END;