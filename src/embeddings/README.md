# Word Embedding Module

## Overview

This module provides functionality for creating and searching word embeddings using a SQLite database. It leverages the `sqlite-vec` extension to enable efficient vector similarity searches directly within SQLite.

The primary purpose is to store text labels and their corresponding vector representations (embeddings) and then perform nearest-neighbor searches to find similar items based on either text labels or vector queries.

## Core Components

### `mod.rs`

This is the main file of the module, containing the core logic for:

*   **Database Connection**: Establishes a connection to a SQLite database (defaulting to `./embeddings.db`) and initializes the `sqlite-vec` extension.
*   **Embedding Storage**: The `embed` function takes a text label and its vector embedding, tokenizes the label using various BPE vocabularies, and stores it in the database.
*   **Vector Search**:
    *   `search`: Finds the top `k` most similar items to a given text label using FTS5 for text matching and then retrieving the associated vectors.
    *   `top`: Finds the top `k` most similar items to a given vector using `sqlite-vec` for vector similarity search.

### `schema.sql`

This file defines the database schema, which is executed when a connection is first established. It sets up the necessary tables and virtual tables for storing text, vocabularies, and vector embeddings, including FTS5 tables for text search and `vec0` virtual tables for vector search.

### `pos.md`

This file contains a reference list of Part-of-Speech (POS) tags and their descriptions, which can be useful for natural language processing tasks related to the embeddings.