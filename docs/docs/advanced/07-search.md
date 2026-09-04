---
sidebar_position: 7
---

# Universal Search & AI Vectors (ferrox-search)

Search is no longer just about matching keywords. In modern enterprise applications, users expect typo-tolerance, and AI integrations expect mathematical representations of meaning (Embeddings).

`Rust-FERROX` solves this by providing the `ferrox-search` module: a universal `SearchEngine` abstraction that supports both classic Lexical Search and next-generation Semantic Vector Search.

## Supported Adapters

### 1. Meilisearch (`feature = "meili"`)
Written in Rust, Meilisearch is the recommended choice for fast, typo-tolerant search bars (e.g., E-commerce catalogs, documentation search). It replaces the heavy JVM overhead of Elasticsearch for 90% of standard use-cases.

### 2. Elasticsearch (`feature = "elastic"`)
For legacy enterprise systems that require complex Lucene aggregations and massive distributed text workloads.

### 3. Qdrant (`feature = "qdrant"`) - *The AI Step Further*
Qdrant is an ultra-fast Vector Database written in Rust. Using this adapter makes your application **AI-ready**. You can feed text to an OpenAI or HuggingFace LLM to extract "Embeddings" (vectors) and search for *meaning* rather than exact keywords (RAG Architecture).

## How to use

```rust
use ferrox_search::{SearchEngine, qdrant::QdrantAdapter, meili::MeilisearchAdapter};

// 1. Setup a Meilisearch engine for standard product search
let meili = MeilisearchAdapter::new("http://localhost:7700", "masterKey");
let results = meili.search("products", "iphoen").await.unwrap(); // Typo tolerant!

// 2. Setup Qdrant for AI Semantic Search
let qdrant = QdrantAdapter::new("http://localhost:6333", "apiKey");
let my_ai_embedding: Vec<f32> = vec![0.12, -0.45, 0.89, ...];
let similar_docs = qdrant.vector_search("knowledge_base", &my_ai_embedding).await.unwrap();
```

By decoupling the engine, your repository code depends only on the `SearchEngine` trait, meaning you can swap out Elasticsearch for Meilisearch without rewriting your business logic.
