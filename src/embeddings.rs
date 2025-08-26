//! Basic embedding representation and utilities.

use std::collections::HashMap;

/// An embedding is a sparse feature map.
#[derive(Debug, Clone, PartialEq)]
pub struct Embedding {
    pub features: HashMap<String, f32>,
}

impl Embedding {
    /// Create an embedding from a feature count map.
    pub fn from_counts(counts: HashMap<String, f32>) -> Self {
        Embedding { features: counts }
    }

    /// Compute L2 norm of the embedding vector.
    pub fn l2_norm(&self) -> f32 {
        self.features.values().map(|v| v * v).sum::<f32>().sqrt()
    }
}

impl Default for Embedding {
    fn default() -> Self {
        Embedding {
            features: HashMap::new(),
        }
    }
}
