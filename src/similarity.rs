//! Functions for calculating similarity between embeddings.

use crate::embeddings::Embedding;
use std::collections::HashMap;

/// Computes the cosine similarity between two embeddings.
/// Returns a value between 0.0 and 1.0.
/// If either embedding has zero magnitude, returns 0.0.
pub fn cosine_similarity(a: &Embedding, b: &Embedding) -> f32 {
    let (smaller, larger) = if a.features.len() <= b.features.len() {
        (&a.features, &b.features)
    } else {
        (&b.features, &a.features)
    };

    // Compute dot product
    let mut dot = 0.0_f32;
    for (k, v) in smaller.iter() {
        if let Some(w) = larger.get(k) {
            dot += v * w;
        }
    }

    // Normalize by L2 norm
    let na = a.l2_norm();
    let nb = b.l2_norm();
    if na == 0.0 || nb == 0.0 {
        return 0.0;
    }

    (dot / (na * nb)).clamp(0.0, 1.0)
}

/// Convenience function: compute similarity from raw counts.
pub fn similarity_from_counts(
    a: HashMap<String, f32>,
    b: HashMap<String, f32>,
) -> f32 {
    let ea = Embedding::from_counts(a);
    let eb = Embedding::from_counts(b);
    cosine_similarity(&ea, &eb)
}
