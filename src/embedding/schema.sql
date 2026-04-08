CREATE TABLE IF NOT EXISTS words (
    rowid INTEGER PRIMARY KEY AUTOINCREMENT,
    label TEXT NOT NULL UNIQUE
    -- terminals TEXT[]
);

CREATE VIRTUAL TABLE IF NOT EXISTS embeddings using vec0 (
    rowid INTEGER PRIMARY KEY,
    vector FLOAT[300]
);

CREATE VIEW IF NOT EXISTS word_embeddings AS
SELECT 
    w.rowid, 
    w.label, 
    e.vector 
FROM words w
JOIN embeddings e ON w.rowid = e.rowid;

CREATE TRIGGER IF NOT EXISTS trg_insert_word_embeddings
INSTEAD OF INSERT ON word_embeddings
BEGIN
    INSERT OR ROLLBACK INTO words (label) VALUES (new.label);
    
    INSERT OR ROLLBACK INTO embeddings (rowid, vector) 
    VALUES (last_insert_rowid(), new.vector);
END;