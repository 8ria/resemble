//! Resemble: A Rust code similarity checker.
//! Provides AST parsing, embeddings, and cosine similarity calculation.

pub mod ast;
pub mod embeddings;
pub mod similarity;

pub use ast::parse_and_count;
pub use embeddings::Embedding;
pub use similarity::cosine_similarity;
