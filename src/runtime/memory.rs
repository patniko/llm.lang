//! Memory module for the LLM.lang runtime
//!
//! This module provides the semantic memory manager for the LLM.lang runtime,
//! which allows storing and retrieving values based on semantic meaning.

use std::collections::HashMap;
use crate::Value;
use super::error::RuntimeError;

/// A semantic memory manager
pub struct Memory {
    /// The stored values
    values: HashMap<String, Value>,
    
    /// The current memory usage in bytes
    memory_usage: usize,
}

impl Memory {
    /// Create a new memory manager
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            memory_usage: 0,
        }
    }
    
    /// Remember a value
    pub fn remember(&mut self, key: &str, value: Value) {
        // Calculate the memory usage of the value
        let value_size = self.calculate_value_size(&value);
        
        // Update the memory usage
        self.memory_usage += value_size;
        
        // Store the value
        self.values.insert(key.to_string(), value);
    }
    
    /// Recall a value
    pub fn recall(&self, key: &str) -> Result<Value, RuntimeError> {
        // Look up the value
        self.values.get(key).cloned().ok_or_else(|| {
            RuntimeError::new(
                &format!("No value remembered for key: '{}'", key),
                crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
            )
        })
    }
    
    /// Recall the most relevant value
    pub fn recall_most_relevant(&self) -> Result<Value, RuntimeError> {
        // In a real implementation, this would use semantic similarity
        // For now, just return the first value
        if let Some((_, value)) = self.values.iter().next() {
            Ok(value.clone())
        } else {
            Err(RuntimeError::new(
                "No values in memory",
                crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Get the current memory usage in bytes
    pub fn memory_usage(&self) -> usize {
        self.memory_usage
    }
    
    /// Calculate the memory usage of a value in bytes
    fn calculate_value_size(&self, value: &Value) -> usize {
        match value {
            Value::Void => 0,
            Value::Bool(_) => 1,
            Value::Int(_) => 8,
            Value::Float(_) => 8,
            Value::String(s) => s.len(),
            Value::List(items) => {
                let mut size = 0;
                for item in items {
                    size += self.calculate_value_size(item);
                }
                size
            }
            Value::Map(map) => {
                let mut size = 0;
                for (key, value) in map {
                    size += key.len();
                    size += self.calculate_value_size(value);
                }
                size
            }
            Value::Vector(v) => v.len() * 8,
            Value::Function(name) => name.len(),
            Value::Context(name) => name.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_new() {
        let memory = Memory::new();
        
        assert_eq!(memory.values.len(), 0);
        assert_eq!(memory.memory_usage(), 0);
    }
    
    #[test]
    fn test_memory_remember() {
        let mut memory = Memory::new();
        
        memory.remember("key", Value::Int(42));
        
        assert_eq!(memory.values.len(), 1);
        assert_eq!(memory.values.get("key"), Some(&Value::Int(42)));
        assert_eq!(memory.memory_usage(), 8);
    }
    
    #[test]
    fn test_memory_recall() {
        let mut memory = Memory::new();
        
        memory.remember("key", Value::Int(42));
        
        let result = memory.recall("key").unwrap();
        assert_eq!(result, Value::Int(42));
        
        let error = memory.recall("nonexistent").unwrap_err();
        assert_eq!(error.message, "No value remembered for key: 'nonexistent'");
    }
    
    #[test]
    fn test_memory_recall_most_relevant() {
        let mut memory = Memory::new();
        
        memory.remember("key", Value::Int(42));
        
        let result = memory.recall_most_relevant().unwrap();
        assert_eq!(result, Value::Int(42));
        
        let mut empty_memory = Memory::new();
        let error = empty_memory.recall_most_relevant().unwrap_err();
        assert_eq!(error.message, "No values in memory");
    }
    
    #[test]
    fn test_memory_calculate_value_size() {
        let memory = Memory::new();
        
        assert_eq!(memory.calculate_value_size(&Value::Void), 0);
        assert_eq!(memory.calculate_value_size(&Value::Bool(true)), 1);
        assert_eq!(memory.calculate_value_size(&Value::Int(42)), 8);
        assert_eq!(memory.calculate_value_size(&Value::Float(3.14)), 8);
        assert_eq!(memory.calculate_value_size(&Value::String("hello".to_string())), 5);
        assert_eq!(memory.calculate_value_size(&Value::List(vec![Value::Int(42)])), 8);
        assert_eq!(
            memory.calculate_value_size(&Value::Map(HashMap::from([
                ("key".to_string(), Value::Int(42))
            ]))),
            11
        );
        assert_eq!(memory.calculate_value_size(&Value::Vector(vec![1.0, 2.0, 3.0])), 24);
        assert_eq!(memory.calculate_value_size(&Value::Function("main".to_string())), 4);
        assert_eq!(memory.calculate_value_size(&Value::Context("MainProgram".to_string())), 11);
    }
}
