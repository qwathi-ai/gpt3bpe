CREATE TABLE IF NOT EXISTS embeddings (
    token BLOB NOT NULL,
    vocabulary TEXT NOT NULL CHECK (vocabulary IN ('r50k', 'p50k', 'cl100k', 'o200k')),
    label TEXT NOT NULL,
    -- typeof TEXT NOT NULL CHECK (typeof IN ('number', 'string', 'boolean')),
    embedding f32[300] NOT NULL,
    qembedding int8[300] NOT NULL,
    PRIMARY KEY(token, vocabulary, label)
);

CREATE INDEX IF NOT EXISTS idx_embedding_vocabulary ON embeddings(vocabulary);

-- CREATE TABLE terminals (
--     token BLOB FOREIGN KEY REFERENCES embeddings.glove(token),
--     partos BLOB FOREIGN KEY REFERENCES partos(_id) ON DELETE CASCADE,
--     PRIMARY KEY (token, partos)
-- );

-- -- Grammar edges
-- CREATE TABLE grammar (
--     _from BLOB,
--     _to BLOB,
--     properties JSON, -- { from_label, to_label, // context }
--     PRIMARY KEY (_from, _to),
--     FOREIGN KEY (_from) REFERENCES partos(_id),
--     FOREIGN KEY (_to) REFERENCES partos(_id)
-- ) WITHOUT ROWID;