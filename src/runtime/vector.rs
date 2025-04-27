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
        // This is a more sophisticated embedding algorithm that simulates
        // how a real embedding model might work, but still deterministic
        
        // Use a larger embedding size for more realistic embeddings
        let target_size = 32;
        let mut embedding = vec![0.0; target_size];
        
        // Process the text in a more sophisticated way
        // 1. Convert to lowercase for consistency
        let text = text.to_lowercase();
        
        // 2. Split into words
        let words: Vec<&str> = text.split_whitespace().collect();
        
        // 3. For each word, compute a "hash" and distribute it across the embedding
        for (i, word) in words.iter().enumerate() {
            // Compute a simple hash of the word
            let mut hash: u64 = 0;
            for c in word.chars() {
                hash = hash.wrapping_mul(31).wrapping_add(c as u64);
            }
            
            // Distribute the hash across the embedding
            for j in 0..target_size {
                let position = (hash.wrapping_add(j as u64)) % target_size as u64;
                let value = ((hash >> j) & 0xFF) as f64 / 255.0;
                
                // Add the value to the embedding, with position-dependent weighting
                embedding[position as usize] += value * (1.0 / (1.0 + i as f64));
            }
        }
        
        // 4. Add positional encoding to capture word order
        for (i, word) in words.iter().enumerate() {
            let position_factor = 1.0 / (1.0 + i as f64);
            let word_len = word.len() as f64;
            
            for j in 0..std::cmp::min(word.len(), target_size) {
                let char_value = word.chars().nth(j).unwrap() as u32 as f64 / 1000.0;
                embedding[j] += char_value * position_factor * (word_len / 10.0);
            }
        }
        
        // 5. Add semantic features based on common patterns
        // Check for question words
        if text.contains("what") || text.contains("who") || text.contains("when") || 
           text.contains("where") || text.contains("why") || text.contains("how") {
            for i in 0..8 {
                embedding[i] += 0.2;
            }
        }
        
        // Check for programming-related terms
        if text.contains("code") || text.contains("program") || text.contains("function") || 
           text.contains("class") || text.contains("variable") || text.contains("algorithm") {
            for i in 8..16 {
                embedding[i] += 0.2;
            }
        }
        
        // Check for sentiment words
        if text.contains("good") || text.contains("great") || text.contains("excellent") || 
           text.contains("amazing") || text.contains("wonderful") || text.contains("positive") {
            for i in 16..24 {
                embedding[i] += 0.2;
            }
        } else if text.contains("bad") || text.contains("terrible") || text.contains("awful") || 
                  text.contains("horrible") || text.contains("negative") || text.contains("poor") {
            for i in 16..24 {
                embedding[i] -= 0.2;
            }
        }
        
        // 6. Normalize the vector
        let magnitude = embedding.iter().map(|x| x * x).sum::<f64>().sqrt();
        
        if magnitude > 0.0 {
            for x in &mut embedding {
                *x /= magnitude;
            }
        } else {
            // If the embedding is all zeros, create a random embedding
            for i in 0..target_size {
                embedding[i] = ((i * 17) % 255) as f64 / 255.0;
            }
            
            // Normalize again
            let magnitude = embedding.iter().map(|x| x * x).sum::<f64>().sqrt();
            for x in &mut embedding {
                *x /= magnitude;
            }
        }
        
        Ok(embedding)
    }
    
    /// Calculate the similarity between two vectors
    pub fn similarity(&self, a: &VectorEmbedding, b: &VectorEmbedding) -> f64 {
        // Calculate the cosine similarity
        let min_len = std::cmp::min(a.len(), b.len());
        
        if min_len == 0 {
            return 0.0;
        }
        
        let mut dot_product = 0.0;
        let mut magnitude_a = 0.0;
        let mut magnitude_b = 0.0;
        
        for i in 0..min_len {
            dot_product += a[i] * b[i];
            magnitude_a += a[i] * a[i];
            magnitude_b += b[i] * b[i];
        }
        
        magnitude_a = magnitude_a.sqrt();
        magnitude_b = magnitude_b.sqrt();
        
        if magnitude_a > 0.0 && magnitude_b > 0.0 {
            // Apply a non-linear transformation to make the similarity more intuitive
            // This makes similar vectors have higher similarity and dissimilar vectors have lower similarity
            let raw_similarity = dot_product / (magnitude_a * magnitude_b);
            
            // Apply sigmoid-like transformation to emphasize differences
            (raw_similarity + 1.0) / 2.0
        } else {
            0.0
        }
    }
    
    /// Find the nearest vectors to a given vector
    pub fn nearest(&self, vector: &VectorEmbedding, vectors: &[VectorEmbedding], k: usize) -> Vec<usize> {
        if vectors.is_empty() {
            return Vec::new();
        }
        
        // Calculate the similarity between the vector and each vector in the list
        let mut similarities: Vec<(usize, f64)> = vectors
            .iter()
            .enumerate()
            .map(|(i, v)| (i, self.similarity(vector, v)))
            .collect();
        
        // Sort by similarity (descending)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Return the indices of the k nearest vectors (or all if k > vectors.len())
        let k = std::cmp::min(k, vectors.len());
        similarities.iter().take(k).map(|(i, _)| *i).collect()
    }
    
    /// Create a vector embedding from a list of words
    pub fn embed_words(&self, words: &[&str]) -> Result<VectorEmbedding, RuntimeError> {
        // Combine the words into a single string
        let text = words.join(" ");
        
        // Use the regular embed function
        self.embed(&text)
    }
    
    /// Create a vector embedding from a list of vectors
    pub fn combine_vectors(&self, vectors: &[VectorEmbedding], weights: Option<&[f64]>) -> Result<VectorEmbedding, RuntimeError> {
        if vectors.is_empty() {
            return Err(RuntimeError::new(
                "Cannot combine empty list of vectors",
                crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        // Get the size of the first vector
        let size = vectors[0].len();
        
        // Check that all vectors have the same size
        for v in vectors.iter().skip(1) {
            if v.len() != size {
                return Err(RuntimeError::new(
                    "All vectors must have the same size",
                    crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
                ));
            }
        }
        
        // Create a new vector
        let mut result = vec![0.0; size];
        
        // Combine the vectors
        if let Some(weights) = weights {
            // Check that the weights have the same length as the vectors
            if weights.len() != vectors.len() {
                return Err(RuntimeError::new(
                    "Weights must have the same length as vectors",
                    crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
                ));
            }
            
            // Combine with weights
            for i in 0..size {
                for (j, v) in vectors.iter().enumerate() {
                    result[i] += v[i] * weights[j];
                }
            }
        } else {
            // Combine with equal weights
            let weight = 1.0 / vectors.len() as f64;
            
            for i in 0..size {
                for v in vectors {
                    result[i] += v[i] * weight;
                }
            }
        }
        
        // Normalize the result
        let magnitude = result.iter().map(|x| x * x).sum::<f64>().sqrt();
        
        if magnitude > 0.0 {
            for x in &mut result {
                *x /= magnitude;
            }
        }
        
        Ok(result)
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
