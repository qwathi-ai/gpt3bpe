-- This table stores the vocabulary words and their associated vocabulary set.
CREATE TABLE IF NOT EXISTS words (
    rid INTEGER PRIMARY KEY AUTOINCREMENT, -- Unique row identifier for each word.
    vocab TEXT NOT NULL CHECK (vocab IN ('P50K','R50K','CL100K','O200K')), -- The vocabulary set the word belongs to.
    label TEXT NOT NULL UNIQUE -- The word/token itself, must be unique.
);

-- This virtual table stores the vector embeddings for each word using the 'vec0' extension.
-- It allows for efficient similarity searches on high-dimensional vectors.
CREATE VIRTUAL TABLE IF NOT EXISTS embeddings using vec0 (
    rid INTEGER PRIMARY KEY FOREIGN KEY REFERENCES words(rid) ON DELETE CASCADE, -- Foreign key linking to the 'words' table. 'ON DELETE CASCADE' ensures that if a word is deleted, its embedding is also deleted.
    vector FLOAT[300] -- The 300-dimensional floating-point vector representing the word embedding.
);

-- This virtual table provides full-text search capabilities using the FTS5 extension.
-- It's configured to use trigram tokenization for substring matching.
CREATE VIRTUAL TABLE IF NOT EXISTS search USING fts5 (
    rid, -- The row identifier, linking back to the 'words' table.
    label, -- The text content (the word/token) to be indexed for searching.
    tokenize='trigram' -- Uses trigram tokenization, indexing overlapping 3-character sequences. This is useful for finding partial matches and handling typos.
);

-- This view provides a unified and convenient way to query for a word, its vocabulary, and its embedding vector.
-- It joins the 'words', 'search', and 'embeddings' tables.
CREATE VIEW IF NOT EXISTS word_embeddings AS
SELECT 
    w.rid, 
    s.label,
    w.vocab,
    e.vector 
FROM words AS w
INNER JOIN search AS s ON w.rid = s.rid
JOIN embeddings AS e ON w.rid = e.rid;

-- This trigger allows direct insertion into the 'word_embeddings' view.
-- Since views are not directly insertable, this trigger intercepts the INSERT operation
-- and correctly distributes the data into the underlying base tables ('words', 'embeddings', 'search').
CREATE TRIGGER IF NOT EXISTS trg_insert_word_embeddings
INSTEAD OF INSERT ON word_embeddings
BEGIN
    -- Insert the new word and its vocabulary into the 'words' table.
    -- 'INSERT OR ROLLBACK' ensures that if this fails (e.g., due to a UNIQUE constraint violation), the transaction is aborted.
    INSERT OR ROLLBACK INTO words (vocab, label) 
    VALUES (new.vocab, new.label);
    
    INSERT OR ROLLBACK INTO embeddings (rid, vector) 
    VALUES (last_insert_rowid(), new.vector);

    INSERT OR ROLLBACK INTO search (rid, label) 
    VALUES (last_insert_rowid(), new.label);
END;

-- This trigger allows direct deletion from the 'word_embeddings' view.
-- It intercepts the DELETE operation and removes the corresponding rows from the underlying tables.
CREATE TRIGGER IF NOT EXISTS trg_delete_word_embeddings 
INSTEAD OF DELETE ON word_embeddings 
BEGIN
  -- Delete the word from the 'words' table using its row id.
  DELETE FROM words WHERE rid = old.rid;

  -- The corresponding row in the 'embeddings' table is deleted automatically
  -- because of the 'ON DELETE CASCADE' constraint on its foreign key.
  -- The line below is therefore redundant:
  -- DELETE FROM embeddings WHERE rid = old.rid;

  -- Delete the word from the FTS5 search index.
  DELETE FROM search WHERE rid = old.rid;
END;
