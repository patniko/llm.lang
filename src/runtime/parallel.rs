//! Parallel module for the LLM.lang runtime
//!
//! This module provides the parallel executor for the LLM.lang runtime,
//! which manages parallel execution of code paths.

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::Value;
use super::error::RuntimeError;

/// A parallel executor
pub struct Parallel {
    /// The maximum number of threads to use
    max_threads: usize,
}

impl Parallel {
    /// Create a new parallel executor
    pub fn new() -> Self {
        Self {
            max_threads: 4,
        }
    }
    
    /// Set the maximum number of threads
    pub fn set_max_threads(&mut self, max_threads: usize) {
        self.max_threads = max_threads;
    }
    
    /// Get the maximum number of threads
    pub fn get_max_threads(&self) -> usize {
        self.max_threads
    }
    
    /// Execute a function in parallel
    pub fn execute<F>(&self, functions: Vec<(String, F)>) -> Vec<(String, Result<Value, RuntimeError>)>
    where
        F: FnOnce() -> Result<Value, RuntimeError> + Send + 'static,
    {
        // Limit the number of threads
        let num_threads = std::cmp::min(functions.len(), self.max_threads);
        
        // Create a thread pool
        let results = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::new();
        
        // Execute each function in a separate thread
        for (name, function) in functions {
            let results = Arc::clone(&results);
            
            let handle = thread::spawn(move || {
                let result = function();
                
                // Store the result
                let mut results = results.lock().unwrap();
                results.push((name, result));
            });
            
            handles.push(handle);
            
            // Limit the number of concurrent threads
            if handles.len() >= num_threads {
                // Wait for a thread to finish
                if let Some(handle) = handles.pop() {
                    handle.join().unwrap();
                }
            }
        }
        
        // Wait for all threads to finish
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Return the results
        Arc::try_unwrap(results).unwrap().into_inner().unwrap()
    }
    
    /// Execute a function with a timeout
    pub fn execute_with_timeout<F>(&self, function: F, timeout: Duration) -> Result<Value, RuntimeError>
    where
        F: FnOnce() -> Result<Value, RuntimeError> + Send + 'static,
    {
        // Create a channel to receive the result
        let (sender, receiver) = std::sync::mpsc::channel();
        
        // Execute the function in a separate thread
        let handle = thread::spawn(move || {
            let result = function();
            
            // Send the result
            let _ = sender.send(result);
        });
        
        // Wait for the result or timeout
        match receiver.recv_timeout(timeout) {
            Ok(result) => {
                // The function completed within the timeout
                result
            }
            Err(_) => {
                // The function timed out
                Err(RuntimeError::new(
                    &format!("Function timed out after {:?}", timeout),
                    crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parallel_new() {
        let parallel = Parallel::new();
        
        assert_eq!(parallel.get_max_threads(), 4);
    }
    
    #[test]
    fn test_parallel_set_max_threads() {
        let mut parallel = Parallel::new();
        
        parallel.set_max_threads(8);
        
        assert_eq!(parallel.get_max_threads(), 8);
    }
    
    #[test]
    fn test_parallel_execute() {
        let parallel = Parallel::new();
        
        let functions = vec![
            ("a".to_string(), || Ok(Value::Int(1))),
            ("b".to_string(), || Ok(Value::Int(2))),
            ("c".to_string(), || Ok(Value::Int(3))),
        ];
        
        let results = parallel.execute(functions);
        
        assert_eq!(results.len(), 3);
        
        // Sort the results by name
        let mut results = results;
        results.sort_by(|a, b| a.0.cmp(&b.0));
        
        assert_eq!(results[0].0, "a");
        assert_eq!(results[1].0, "b");
        assert_eq!(results[2].0, "c");
        
        assert_eq!(results[0].1.as_ref().unwrap(), &Value::Int(1));
        assert_eq!(results[1].1.as_ref().unwrap(), &Value::Int(2));
        assert_eq!(results[2].1.as_ref().unwrap(), &Value::Int(3));
    }
    
    #[test]
    fn test_parallel_execute_with_timeout() {
        let parallel = Parallel::new();
        
        // Test a function that completes within the timeout
        let result = parallel.execute_with_timeout(
            || Ok(Value::Int(42)),
            Duration::from_millis(100),
        );
        
        assert_eq!(result.unwrap(), Value::Int(42));
        
        // Test a function that times out
        let result = parallel.execute_with_timeout(
            || {
                thread::sleep(Duration::from_millis(200));
                Ok(Value::Int(42))
            },
            Duration::from_millis(100),
        );
        
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message,
            "Function timed out after 100ms"
        );
    }
}
