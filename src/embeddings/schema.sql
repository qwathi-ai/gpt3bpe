
CREATE TABLE IF NOT EXISTS words (
    rid INTEGER PRIMARY KEY AUTOINCREMENT,
    tokens TEXT NOT NULL CHECK(
        json_valid(tokens) AND 
        json_type(tokens) = 'array' AND 
        json_array_length(tokens) = 3
    ),
    vocab TEXT NOT NULL CHECK (vocab IN ('P50K','R50K','CL100K','O200K')),
    label TEXT NOT NULL UNIQUE
);
CREATE INDEX IF NOT EXISTS idx_tokens ON words(tokens->>1, tokens->>2);

CREATE VIRTUAL TABLE IF NOT EXISTS embeddings using vec0 (
    rid INTEGER PRIMARY KEY FOREIGN KEY REFERENCES words(rid) ON DELETE CASCADE,
    vector FLOAT[300]
);

CREATE VIEW IF NOT EXISTS word_embeddings AS
SELECT
    w.rid, 
    w.tokens,
    w.label,
    w.vocab,
    e.vector 
FROM words AS w
LEFT JOIN embeddings AS e 
    ON w.rid = e.rid;

CREATE TRIGGER IF NOT EXISTS trg_insert_word_embeddings
INSTEAD OF INSERT ON word_embeddings
BEGIN
    INSERT OR ROLLBACK INTO words (vocab, label, tokens) 
    VALUES (new.vocab, new.label, new.tokens);
    
    INSERT OR ROLLBACK INTO embeddings (rid, vector) 
    VALUES (last_insert_rowid(), new.vector);
END;


CREATE TRIGGER IF NOT EXISTS trg_delete_word_embeddings 
INSTEAD OF DELETE ON word_embeddings 
BEGIN
  DELETE FROM words WHERE rid = old.rid;
END;