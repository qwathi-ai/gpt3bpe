# Embeddings Module

## Overview

This module provides functionality for creating and searching word embeddings using a SQLite database. It is available when the `embeddings` feature is enabled. It leverages the `sqlite-vec` extension to enable efficient vector similarity searches directly within SQLite.

The primary purpose is to store text labels and their corresponding vector representations (embeddings) and then perform nearest-neighbor searches to find similar items based on either text labels or vector queries.

## Default Embeddings

The module can be used with any set of pre-trained word embeddings, but it is tested with a specific set of GloVe vectors. The metadata for these embeddings is detailed in `metadata.json`.

Key details:
*   **Algorithm**: GloVe (Global Vectors), version 1.2
*   **Dimensions**: 300
*   **Vocabulary Size**: 262,269
*   **Corpus**: Gigaword 5th Edition (lemmatized, case-preserved)
*   **Training Parameters**:
    *   Window size: 5
    *   Iterations: 100
*   **Source**: NLPL Vector Repository
*   **Creator**: Andrey Kutuzov

## Core Components

### `mod.rs`

This is the main file of the module, containing the core logic for:

*   **Database Connection**: Establishes a connection to a SQLite database. The path to the database is expected to be provided via the `EMBEDDINGS` environment variable. The module initializes the `sqlite-vec` extension on the connection.
*   **`insert`**: Takes a text label and its vector embedding and stores it in the database.
*   **`search`**: Finds the top `k` most similar items to a given text label by first finding the corresponding vector and then performing a vector similarity search.
*   **`nearest`**: Finds the top `k` most similar items to a given vector using `sqlite-vec` for vector similarity search.

### `schema.sql`

This file defines the database schema, which is executed when a connection is first established. It sets up the necessary tables for storing vector embeddings, including a `vec0` virtual table for efficient vector search.

### `unit.rs`

This file contains unit tests for the embeddings module, ensuring that the `insert`, `search`, and `nearest` functions work as expected.

### `pos.md`

This file contains a reference list of Part-of-Speech (POS) tags and their descriptions, which can be useful for natural language processing tasks related to the embeddings.