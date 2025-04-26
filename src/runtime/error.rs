//! Error definitions for the LLM.lang runtime
//!
//! This module defines the error types and structures used by the runtime.

use crate::utils::SourceLocation;
use std::fmt;

/// An error that can occur during runtime
#[derive(Debug, Clone)]
pub struct RuntimeError {
    /// The error message
    pub message: String,
    
    /// The error location
    pub location: SourceLocation,
}

impl RuntimeError {
    /// Create a new runtime error
    pub fn new(message: &str, location: SourceLocation) -> Self {
        Self {
            message: message.to_string(),
            location,
        }
    }
    
    /// Create a new runtime error with a formatted message
    pub fn with_format(format: fmt::Arguments<'_>, location: SourceLocation) -> Self {
        Self {
            message: format!("{}", format),
            location,
        }
    }
    
    /// Create a new "missing attribute" error
    pub fn missing_attribute(attribute: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Missing attribute: '{}'", attribute), location)
    }
    
    /// Create a new "missing child" error
    pub fn missing_child(index: usize, location: SourceLocation) -> Self {
        Self::new(&format!("Missing child at index {}", index), location)
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
    
    /// Create a new "undefined property" error
    pub fn undefined_property(name: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Undefined property: '{}'", name), location)
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
    
    /// Create a new "invalid type" error
    pub fn invalid_type(expected: &str, actual: &str, location: SourceLocation) -> Self {
        Self::new(
            &format!("Invalid type: expected {}, got {}", expected, actual),
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
    
    /// Create a new "invalid literal" error
    pub fn invalid_literal(value: &str, expected_type: &str, location: SourceLocation) -> Self {
        Self::new(
            &format!(
                "Invalid {} literal: '{}'",
                expected_type, value
            ),
            location,
        )
    }
    
    /// Create a new "unknown type" error
    pub fn unknown_type(typ: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Unknown type: '{}'", typ), location)
    }
    
    /// Create a new "unknown operator" error
    pub fn unknown_operator(operator: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Unknown operator: '{}'", operator), location)
    }
    
    /// Create a new "unknown semantic token" error
    pub fn unknown_semantic_token(token: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Unknown semantic token: '{}'", token), location)
    }
    
    /// Create a new "division by zero" error
    pub fn division_by_zero(location: SourceLocation) -> Self {
        Self::new("Division by zero", location)
    }
    
    /// Create a new "not callable" error
    pub fn not_callable(value: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Not callable: {}", value), location)
    }
    
    /// Create a new "missing body" error
    pub fn missing_body(function: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Missing body for function '{}'", function), location)
    }
    
    /// Create a new "time limit exceeded" error
    pub fn time_limit_exceeded(elapsed: u64, limit: u64) -> Self {
        Self::new(
            &format!(
                "Time limit exceeded: {} ms (limit: {} ms)",
                elapsed, limit
            ),
            SourceLocation::new(0, 0, 0, 0, ""),
        )
    }
    
    /// Create a new "memory limit exceeded" error
    pub fn memory_limit_exceeded(usage: usize, limit: usize) -> Self {
        Self::new(
            &format!(
                "Memory limit exceeded: {} bytes (limit: {} bytes)",
                usage, limit
            ),
            SourceLocation::new(0, 0, 0, 0, ""),
        )
    }
    
    /// Create a new "feature disabled" error
    pub fn feature_disabled(feature: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Feature disabled: '{}'", feature), location)
    }
    
    /// Create a new "invalid strategy" error
    pub fn invalid_strategy(strategy: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Invalid strategy: '{}'", strategy), location)
    }
    
    /// Create a new "no paths" error
    pub fn no_paths(location: SourceLocation) -> Self {
        Self::new("No paths in parallel statement", location)
    }
}

impl std::error::Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Runtime error: {} at {}:{}:{}",
            self.message,
            self.location.file,
            self.location.start_line,
            self.location.start_column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_runtime_error_new() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::new("Test error", location.clone());
        
        assert_eq!(error.message, "Test error");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_with_format() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::with_format(format_args!("Error: {}", 42), location.clone());
        
        assert_eq!(error.message, "Error: 42");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_missing_attribute() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::missing_attribute("name", location.clone());
        
        assert_eq!(error.message, "Missing attribute: 'name'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_missing_child() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::missing_child(0, location.clone());
        
        assert_eq!(error.message, "Missing child at index 0");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_undefined_variable() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::undefined_variable("x", location.clone());
        
        assert_eq!(error.message, "Undefined variable: 'x'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_undefined_function() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::undefined_function("foo", location.clone());
        
        assert_eq!(error.message, "Undefined function: 'foo'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_undefined_context() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::undefined_context("MyContext", location.clone());
        
        assert_eq!(error.message, "Undefined context: 'MyContext'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_undefined_property() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::undefined_property("prop", location.clone());
        
        assert_eq!(error.message, "Undefined property: 'prop'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_invalid_assignment_target() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::invalid_assignment_target(location.clone());
        
        assert_eq!(error.message, "Invalid assignment target");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_invalid_argument_count() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::invalid_argument_count("foo", 2, 3, location.clone());
        
        assert_eq!(
            error.message,
            "Invalid argument count for function 'foo': expected 2, got 3"
        );
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_invalid_type() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::invalid_type("Int", "String", location.clone());
        
        assert_eq!(error.message, "Invalid type: expected Int, got String");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_invalid_operation() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::invalid_operation("+", "Int", "String", location.clone());
        
        assert_eq!(error.message, "Invalid operation: Int + String");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_invalid_unary_operation() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::invalid_unary_operation("-", "String", location.clone());
        
        assert_eq!(error.message, "Invalid unary operation: - String");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_invalid_literal() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::invalid_literal("abc", "Int", location.clone());
        
        assert_eq!(error.message, "Invalid Int literal: 'abc'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_unknown_type() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::unknown_type("Foo", location.clone());
        
        assert_eq!(error.message, "Unknown type: 'Foo'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_unknown_operator() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::unknown_operator("$", location.clone());
        
        assert_eq!(error.message, "Unknown operator: '$'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_unknown_semantic_token() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::unknown_semantic_token("@foo", location.clone());
        
        assert_eq!(error.message, "Unknown semantic token: '@foo'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_division_by_zero() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::division_by_zero(location.clone());
        
        assert_eq!(error.message, "Division by zero");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_not_callable() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::not_callable("Int", location.clone());
        
        assert_eq!(error.message, "Not callable: Int");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_missing_body() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::missing_body("foo", location.clone());
        
        assert_eq!(error.message, "Missing body for function 'foo'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_time_limit_exceeded() {
        let error = RuntimeError::time_limit_exceeded(1000, 500);
        
        assert_eq!(error.message, "Time limit exceeded: 1000 ms (limit: 500 ms)");
    }
    
    #[test]
    fn test_runtime_error_memory_limit_exceeded() {
        let error = RuntimeError::memory_limit_exceeded(1000, 500);
        
        assert_eq!(error.message, "Memory limit exceeded: 1000 bytes (limit: 500 bytes)");
    }
    
    #[test]
    fn test_runtime_error_feature_disabled() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::feature_disabled("Vector", location.clone());
        
        assert_eq!(error.message, "Feature disabled: 'Vector'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_invalid_strategy() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::invalid_strategy("foo", location.clone());
        
        assert_eq!(error.message, "Invalid strategy: 'foo'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_no_paths() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::no_paths(location.clone());
        
        assert_eq!(error.message, "No paths in parallel statement");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_runtime_error_display() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = RuntimeError::new("Test error", location);
        
        assert_eq!(
            format!("{}", error),
            "Runtime error: Test error at test.llm:1:1"
        );
    }
}
