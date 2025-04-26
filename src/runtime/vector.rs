//! Vector module for the LLM.lang runtime
//!
//! This module provides the vector engine for the LLM.lang runtime,
//! which manages semantic vector embeddings.

use super::error::RuntimeError;

/// A semantic vector embedding
pub type VectorEmbedding = Vec<f64>;

/// A vector engine
pub struct Vector {
    /// The current vector
    current_vector: Option<VectorEmbedding>,
}

impl Vector {
    /// Create a new vector engine
    pub fn new() -> Self {
        Self {
            current_vector: None,
        }
    }
    
    /// Set the current vector
    pub fn set_current_vector(&mut self, vector: VectorEmbedding) {
        self.current_vector = Some(vector);
    }
    
    /// Clear the current vector
    pub fn clear_current_vector(&mut self) {
        self.current_vector = None;
    }
    
    /// Get the current vector
    pub fn get_current_vector(&self) -> Option<&VectorEmbedding> {
        self.current_vector.as_ref()
    }
    
    /// Create a vector embedding from text
    pub fn embed(&self, text: &str) -> Result<VectorEmbedding, RuntimeError> {
        // In a real implementation, this would use a language model to create a vector embedding
        // For now, just create a simple embedding based on character codes
        let mut embedding = Vec::new();
        
        for c in text.chars() {
            embedding.push(c as u32 as f64 / 1000.0);
        }
        
        // Pad or truncate to a fixed size
        let target_size = 10;
        
        if embedding.len() < target_size {
            // Pad with zeros
            embedding.resize(target_size, 0.0);
        } else if embedding.len() > target_size {
            // Truncate
            embedding.truncate(target_size);
        }
        
        // Normalize the vector
        let magnitude = embedding.iter().map(|x| x * x).sum::<f64>().sqrt();
        
        if magnitude > 0.0 {
            for x in &mut embedding {
                *x /= magnitude;
            }
        }
        
        Ok(embedding)
    }
    
    /// Calculate the similarity between two vectors
    pub fn similarity(&self, a: &VectorEmbedding, b: &VectorEmbedding) -> f64 {
        // Calculate the cosine similarity
        let dot_product = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f64>();
        let magnitude_a = a.iter().map(|x| x * x).sum::<f64>().sqrt();
        let magnitude_b = b.iter().map(|x| x * x).sum::<f64>().sqrt();
        
        if magnitude_a > 0.0 && magnitude_b > 0.0 {
            dot_product / (magnitude_a * magnitude_b)
        } else {
            0.0
        }
    }
    
    /// Find the nearest vectors to a given vector
    pub fn nearest(&self, vector: &VectorEmbedding, vectors: &[VectorEmbedding], k: usize) -> Vec<usize> {
        // Calculate the similarity between the vector and each vector in the list
        let mut similarities: Vec<(usize, f64)> = vectors
            .iter()
            .enumerate()
            .map(|(i, v)| (i, self.similarity(vector, v)))
            .collect();
        
        // Sort by similarity (descending)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Return the indices of the k nearest vectors
        similarities.iter().take(k).map(|(i, _)| *i).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vector_new() {
        let vector = Vector::new();
        
        assert!(vector.get_current_vector().is_none());
    }
    
    #[test]
    fn test_vector_set_current_vector() {
        let mut vector = Vector::new();
        
        vector.set_current_vector(vec![1.0, 2.0, 3.0]);
        
        assert_eq!(vector.get_current_vector(), Some(&vec![1.0, 2.0, 3.0]));
    }
    
    #[test]
    fn test_vector_clear_current_vector() {
        let mut vector = Vector::new();
        
        vector.set_current_vector(vec![1.0, 2.0, 3.0]);
        vector.clear_current_vector();
        
        assert!(vector.get_current_vector().is_none());
    }
    
    #[test]
    fn test_vector_embed() {
        let vector = Vector::new();
        
        let embedding = vector.embed("hello").unwrap();
        
        assert_eq!(embedding.len(), 10);
        
        // Check that the embedding is normalized
        let magnitude = embedding.iter().map(|x| x * x).sum::<f64>().sqrt();
        assert!((magnitude - 1.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_vector_similarity() {
        let vector = Vector::new();
        
        let a = vector.embed("hello").unwrap();
        let b = vector.embed("hello").unwrap();
        let c = vector.embed("world").unwrap();
        
        assert!((vector.similarity(&a, &b) - 1.0).abs() < 1e-10);
        assert!(vector.similarity(&a, &c) < 1.0);
    }
    
    #[test]
    fn test_vector_nearest() {
        let vector = Vector::new();
        
        let a = vector.embed("hello").unwrap();
        let b = vector.embed("hello world").unwrap();
        let c = vector.embed("world").unwrap();
        let d = vector.embed("foo").unwrap();
        
        let vectors = vec![b.clone(), c.clone(), d.clone()];
        
        let nearest = vector.nearest(&a, &vectors, 2);
        
        assert_eq!(nearest.len(), 2);
        assert_eq!(nearest[0], 0); // b is most similar to a
    }
}
