//! Example module for the LLM.lang runtime
//!
//! This module provides the example executor for the LLM.lang runtime,
//! which learns from examples and generalizes to new inputs.

use std::collections::HashMap;
use crate::Value;
use super::error::RuntimeError;
use super::vector::Vector;

/// An example data
#[derive(Debug, Clone)]
struct ExampleData {
    /// The input
    input: Value,
    
    /// The output
    output: Value,
}

/// An example executor
pub struct Example {
    /// The examples
    examples: Vec<ExampleData>,
    
    /// The vector engine
    vector: Vector,
}

impl Example {
    /// Create a new example executor
    pub fn new() -> Self {
        Self {
            examples: Vec::new(),
            vector: Vector::new(),
        }
    }
    
    /// Add an example
    pub fn add_example(&mut self, input: Value, output: Value) {
        self.examples.push(ExampleData { input, output });
    }
    
    /// Get the examples
    pub fn get_examples(&self) -> Vec<(Value, Value)> {
        self.examples.iter().map(|e| (e.input.clone(), e.output.clone())).collect()
    }
    
    /// Clear the examples
    pub fn clear_examples(&mut self) {
        self.examples.clear();
    }
    
    /// Execute an example
    pub fn execute(&self, input: Value) -> Result<Value, RuntimeError> {
        // If there are no examples, return an error
        if self.examples.is_empty() {
            return Err(RuntimeError::new(
                "No examples to learn from",
                crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        // Find the most similar example
        let mut best_example = &self.examples[0];
        let mut best_similarity = self.similarity(&input, &best_example.input);
        
        for example in &self.examples[1..] {
            let similarity = self.similarity(&input, &example.input);
            
            if similarity > best_similarity {
                best_example = example;
                best_similarity = similarity;
            }
        }
        
        // Return the output of the most similar example
        Ok(best_example.output.clone())
    }
    
    /// Calculate the similarity between two values
    fn similarity(&self, a: &Value, b: &Value) -> f64 {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => {
                if a == b {
                    1.0
                } else {
                    0.0
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if (a - b).abs() < 1e-10 {
                    1.0
                } else {
                    0.0
                }
            }
            (Value::String(a), Value::String(b)) => {
                // Use the vector engine to calculate the similarity
                if let (Ok(a_vec), Ok(b_vec)) = (self.vector.embed(a), self.vector.embed(b)) {
                    self.vector.similarity(&a_vec, &b_vec)
                } else {
                    0.0
                }
            }
            (Value::List(a), Value::List(b)) => {
                if a.len() != b.len() {
                    return 0.0;
                }
                
                let mut similarity = 0.0;
                
                for (a_item, b_item) in a.iter().zip(b.iter()) {
                    similarity += self.similarity(a_item, b_item);
                }
                
                similarity / a.len() as f64
            }
            (Value::Map(a), Value::Map(b)) => {
                if a.len() != b.len() {
                    return 0.0;
                }
                
                let mut similarity = 0.0;
                
                for (key, a_value) in a {
                    if let Some(b_value) = b.get(key) {
                        similarity += self.similarity(a_value, b_value);
                    } else {
                        return 0.0;
                    }
                }
                
                similarity / a.len() as f64
            }
            _ => 0.0,
        }
    }
    
    /// Learn from examples
    pub fn learn(&mut self, examples: Vec<(Value, Value)>) {
        for (input, output) in examples {
            self.add_example(input, output);
        }
    }
    
    /// Generalize from examples
    pub fn generalize(&self, inputs: Vec<Value>) -> Result<Vec<Value>, RuntimeError> {
        let mut outputs = Vec::new();
        
        for input in inputs {
            outputs.push(self.execute(input)?);
        }
        
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_new() {
        let example = Example::new();
        
        assert_eq!(example.get_examples().len(), 0);
    }
    
    #[test]
    fn test_example_add_example() {
        let mut example = Example::new();
        
        example.add_example(Value::Int(1), Value::Int(2));
        
        assert_eq!(example.get_examples().len(), 1);
        assert_eq!(example.get_examples()[0].0, Value::Int(1));
        assert_eq!(example.get_examples()[0].1, Value::Int(2));
    }
    
    #[test]
    fn test_example_clear_examples() {
        let mut example = Example::new();
        
        example.add_example(Value::Int(1), Value::Int(2));
        example.clear_examples();
        
        assert_eq!(example.get_examples().len(), 0);
    }
    
    #[test]
    fn test_example_execute() {
        let mut example = Example::new();
        
        example.add_example(Value::Int(1), Value::Int(2));
        example.add_example(Value::Int(3), Value::Int(4));
        
        let result = example.execute(Value::Int(1)).unwrap();
        
        assert_eq!(result, Value::Int(2));
        
        let result = example.execute(Value::Int(3)).unwrap();
        
        assert_eq!(result, Value::Int(4));
    }
    
    #[test]
    fn test_example_execute_no_examples() {
        let example = Example::new();
        
        let result = example.execute(Value::Int(1));
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "No examples to learn from");
    }
    
    #[test]
    fn test_example_similarity() {
        let example = Example::new();
        
        assert_eq!(example.similarity(&Value::Int(1), &Value::Int(1)), 1.0);
        assert_eq!(example.similarity(&Value::Int(1), &Value::Int(2)), 0.0);
        
        assert_eq!(example.similarity(&Value::Float(1.0), &Value::Float(1.0)), 1.0);
        assert_eq!(example.similarity(&Value::Float(1.0), &Value::Float(2.0)), 0.0);
        
        assert!(example.similarity(&Value::String("hello".to_string()), &Value::String("hello".to_string())) > 0.9);
        assert!(example.similarity(&Value::String("hello".to_string()), &Value::String("world".to_string())) < 0.9);
        
        assert_eq!(
            example.similarity(
                &Value::List(vec![Value::Int(1), Value::Int(2)]),
                &Value::List(vec![Value::Int(1), Value::Int(2)])
            ),
            1.0
        );
        
        assert_eq!(
            example.similarity(
                &Value::List(vec![Value::Int(1), Value::Int(2)]),
                &Value::List(vec![Value::Int(1), Value::Int(3)])
            ),
            0.5
        );
        
        assert_eq!(
            example.similarity(
                &Value::Map(HashMap::from([
                    ("a".to_string(), Value::Int(1)),
                    ("b".to_string(), Value::Int(2)),
                ])),
                &Value::Map(HashMap::from([
                    ("a".to_string(), Value::Int(1)),
                    ("b".to_string(), Value::Int(2)),
                ]))
            ),
            1.0
        );
        
        assert_eq!(
            example.similarity(
                &Value::Map(HashMap::from([
                    ("a".to_string(), Value::Int(1)),
                    ("b".to_string(), Value::Int(2)),
                ])),
                &Value::Map(HashMap::from([
                    ("a".to_string(), Value::Int(1)),
                    ("b".to_string(), Value::Int(3)),
                ]))
            ),
            0.5
        );
    }
    
    #[test]
    fn test_example_learn() {
        let mut example = Example::new();
        
        example.learn(vec![
            (Value::Int(1), Value::Int(2)),
            (Value::Int(3), Value::Int(4)),
        ]);
        
        assert_eq!(example.get_examples().len(), 2);
        assert_eq!(example.get_examples()[0].0, Value::Int(1));
        assert_eq!(example.get_examples()[0].1, Value::Int(2));
        assert_eq!(example.get_examples()[1].0, Value::Int(3));
        assert_eq!(example.get_examples()[1].1, Value::Int(4));
    }
    
    #[test]
    fn test_example_generalize() {
        let mut example = Example::new();
        
        example.add_example(Value::Int(1), Value::Int(2));
        example.add_example(Value::Int(3), Value::Int(4));
        
        let result = example.generalize(vec![Value::Int(1), Value::Int(3)]).unwrap();
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Value::Int(2));
        assert_eq!(result[1], Value::Int(4));
    }
}
