CREATE VIRTUAL TABLE IF NOT EXISTS embeddings using vec0 (
    label TEXT,
    vector FLOAT[300]
);
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