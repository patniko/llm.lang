//! Error definitions for the LLM.lang semantic analyzer
//!
//! This module defines the error types and structures used by the semantic analyzer.

use crate::utils::SourceLocation;
use std::fmt;

/// An error that can occur during semantic analysis
#[derive(Debug, Clone)]
pub struct SemanticError {
    /// The error message
    pub message: String,
    
    /// The error location
    pub location: SourceLocation,
}

impl SemanticError {
    /// Create a new semantic error
    pub fn new(message: &str, location: SourceLocation) -> Self {
        Self {
            message: message.to_string(),
            location,
        }
    }
    
    /// Create a new semantic error with a formatted message
    pub fn with_format(format: fmt::Arguments<'_>, location: SourceLocation) -> Self {
        Self {
            message: format!("{}", format),
            location,
        }
    }
    
    /// Create a new "undefined variable" error
    pub fn undefined_variable(name: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Undefined variable: '{}'", name), location)
    }
    
    /// Create a new "undefined function" error
    pub fn undefined_function(name: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Undefined function: '{}'", name), location)
    }
    
    /// Create a new "undefined context" error
    pub fn undefined_context(name: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Undefined context: '{}'", name), location)
    }
    
    /// Create a new "undefined type" error
    pub fn undefined_type(name: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Undefined type: '{}'", name), location)
    }
    
    /// Create a new "invalid type" error
    pub fn invalid_type(expected: &str, actual: &str, location: SourceLocation) -> Self {
        Self::new(
            &format!("Invalid type: expected {}, got {}", expected, actual),
            location,
        )
    }
    
    /// Create a new "invalid assignment target" error
    pub fn invalid_assignment_target(location: SourceLocation) -> Self {
        Self::new("Invalid assignment target", location)
    }
    
    /// Create a new "invalid argument count" error
    pub fn invalid_argument_count(
        function: &str,
        expected: usize,
        actual: usize,
        location: SourceLocation,
    ) -> Self {
        Self::new(
            &format!(
                "Invalid argument count for function '{}': expected {}, got {}",
                function, expected, actual
            ),
            location,
        )
    }
    
    /// Create a new "invalid operation" error
    pub fn invalid_operation(
        operator: &str,
        left: &str,
        right: &str,
        location: SourceLocation,
    ) -> Self {
        Self::new(
            &format!(
                "Invalid operation: {} {} {}",
                left, operator, right
            ),
            location,
        )
    }
    
    /// Create a new "invalid unary operation" error
    pub fn invalid_unary_operation(
        operator: &str,
        operand: &str,
        location: SourceLocation,
    ) -> Self {
        Self::new(
            &format!(
                "Invalid unary operation: {} {}",
                operator, operand
            ),
            location,
        )
    }
    
    /// Create a new "redefined variable" error
    pub fn redefined_variable(name: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Redefined variable: '{}'", name), location)
    }
    
    /// Create a new "redefined function" error
    pub fn redefined_function(name: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Redefined function: '{}'", name), location)
    }
    
    /// Create a new "redefined context" error
    pub fn redefined_context(name: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Redefined context: '{}'", name), location)
    }
    
    /// Create a new "return outside function" error
    pub fn return_outside_function(location: SourceLocation) -> Self {
        Self::new("Return statement outside function", location)
    }
    
    /// Create a new "break outside loop" error
    pub fn break_outside_loop(location: SourceLocation) -> Self {
        Self::new("Break statement outside loop", location)
    }
    
    /// Create a new "continue outside loop" error
    pub fn continue_outside_loop(location: SourceLocation) -> Self {
        Self::new("Continue statement outside loop", location)
    }
    
    /// Create a new "invalid strategy" error
    pub fn invalid_strategy(strategy: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Invalid strategy: '{}'", strategy), location)
    }
    
    /// Create a new "no paths" error
    pub fn no_paths(location: SourceLocation) -> Self {
        Self::new("No paths in parallel statement", location)
    }
    
    /// Create a new "missing return" error
    pub fn missing_return(function: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Missing return in function '{}'", function), location)
    }
    
    /// Create a new "invalid semantic token" error
    pub fn invalid_semantic_token(token: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Invalid semantic token: '{}'", token), location)
    }
}

impl std::error::Error for SemanticError {}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Semantic error: {} at {}:{}:{}",
            self.message,
            self.location.file,
            self.location.start_line,
            self.location.start_column
        )
    }
}

/// A result type for semantic analysis
pub type SemanticResult<T> = Result<T, SemanticError>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_semantic_error_new() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::new("Test error", location.clone());
        
        assert_eq!(error.message, "Test error");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_with_format() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::with_format(format_args!("Error: {}", 42), location.clone());
        
        assert_eq!(error.message, "Error: 42");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_undefined_variable() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::undefined_variable("x", location.clone());
        
        assert_eq!(error.message, "Undefined variable: 'x'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_undefined_function() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::undefined_function("foo", location.clone());
        
        assert_eq!(error.message, "Undefined function: 'foo'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_undefined_context() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::undefined_context("MyContext", location.clone());
        
        assert_eq!(error.message, "Undefined context: 'MyContext'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_undefined_type() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::undefined_type("Foo", location.clone());
        
        assert_eq!(error.message, "Undefined type: 'Foo'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_invalid_type() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::invalid_type("Int", "String", location.clone());
        
        assert_eq!(error.message, "Invalid type: expected Int, got String");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_invalid_assignment_target() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::invalid_assignment_target(location.clone());
        
        assert_eq!(error.message, "Invalid assignment target");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_invalid_argument_count() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::invalid_argument_count("foo", 2, 3, location.clone());
        
        assert_eq!(
            error.message,
            "Invalid argument count for function 'foo': expected 2, got 3"
        );
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_invalid_operation() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::invalid_operation("+", "Int", "String", location.clone());
        
        assert_eq!(error.message, "Invalid operation: Int + String");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_invalid_unary_operation() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::invalid_unary_operation("-", "String", location.clone());
        
        assert_eq!(error.message, "Invalid unary operation: - String");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_redefined_variable() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::redefined_variable("x", location.clone());
        
        assert_eq!(error.message, "Redefined variable: 'x'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_redefined_function() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::redefined_function("foo", location.clone());
        
        assert_eq!(error.message, "Redefined function: 'foo'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_redefined_context() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::redefined_context("MyContext", location.clone());
        
        assert_eq!(error.message, "Redefined context: 'MyContext'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_return_outside_function() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::return_outside_function(location.clone());
        
        assert_eq!(error.message, "Return statement outside function");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_break_outside_loop() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::break_outside_loop(location.clone());
        
        assert_eq!(error.message, "Break statement outside loop");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_continue_outside_loop() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::continue_outside_loop(location.clone());
        
        assert_eq!(error.message, "Continue statement outside loop");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_invalid_strategy() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::invalid_strategy("foo", location.clone());
        
        assert_eq!(error.message, "Invalid strategy: 'foo'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_no_paths() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::no_paths(location.clone());
        
        assert_eq!(error.message, "No paths in parallel statement");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_missing_return() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::missing_return("foo", location.clone());
        
        assert_eq!(error.message, "Missing return in function 'foo'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_invalid_semantic_token() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::invalid_semantic_token("@foo", location.clone());
        
        assert_eq!(error.message, "Invalid semantic token: '@foo'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_semantic_error_display() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = SemanticError::new("Test error", location);
        
        assert_eq!(
            format!("{}", error),
            "Semantic error: Test error at test.llm:1:1"
        );
    }
}
