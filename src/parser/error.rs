//! Error definitions for the LLM.lang parser
//!
//! This module defines the error types and structures used by the parser.

use crate::utils::SourceLocation;
use std::fmt;

/// An error that can occur during parsing
#[derive(Debug, Clone)]
pub struct ParserError {
    /// The error message
    pub message: String,
    
    /// The error location
    pub location: SourceLocation,
}

impl ParserError {
    /// Create a new parser error
    pub fn new(message: &str, location: SourceLocation) -> Self {
        Self {
            message: message.to_string(),
            location,
        }
    }
    
    /// Create a new parser error with a formatted message
    pub fn with_format(format: fmt::Arguments<'_>, location: SourceLocation) -> Self {
        Self {
            message: format!("{}", format),
            location,
        }
    }
    
    /// Create a new "unexpected token" error
    pub fn unexpected_token(token: &str, expected: &str, location: SourceLocation) -> Self {
        Self::new(
            &format!("Unexpected token: '{}', expected {}", token, expected),
            location,
        )
    }
    
    /// Create a new "unexpected end of input" error
    pub fn unexpected_end_of_input(expected: &str, location: SourceLocation) -> Self {
        Self::new(
            &format!("Unexpected end of input, expected {}", expected),
            location,
        )
    }
    
    /// Create a new "invalid syntax" error
    pub fn invalid_syntax(message: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Invalid syntax: {}", message), location)
    }
    
    /// Create a new "invalid type" error
    pub fn invalid_type(actual: &str, expected: &str, location: SourceLocation) -> Self {
        Self::new(
            &format!("Invalid type: expected {}, got {}", expected, actual),
            location,
        )
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
}

impl std::error::Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parser error: {} at {}:{}:{}",
            self.message,
            self.location.file,
            self.location.start_line,
            self.location.start_column
        )
    }
}

/// A result type for parsing
pub type ParserResult<T> = Result<T, ParserError>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parser_error_new() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = ParserError::new("Test error", location.clone());
        
        assert_eq!(error.message, "Test error");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_parser_error_with_format() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = ParserError::with_format(format_args!("Error: {}", 42), location.clone());
        
        assert_eq!(error.message, "Error: 42");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_parser_error_unexpected_token() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = ParserError::unexpected_token("+", "identifier", location.clone());
        
        assert_eq!(error.message, "Unexpected token: '+', expected identifier");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_parser_error_unexpected_end_of_input() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = ParserError::unexpected_end_of_input("identifier", location.clone());
        
        assert_eq!(error.message, "Unexpected end of input, expected identifier");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_parser_error_invalid_syntax() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = ParserError::invalid_syntax("Missing semicolon", location.clone());
        
        assert_eq!(error.message, "Invalid syntax: Missing semicolon");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_parser_error_invalid_type() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = ParserError::invalid_type("String", "Int", location.clone());
        
        assert_eq!(error.message, "Invalid type: expected Int, got String");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_parser_error_undefined_variable() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = ParserError::undefined_variable("x", location.clone());
        
        assert_eq!(error.message, "Undefined variable: 'x'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_parser_error_undefined_function() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = ParserError::undefined_function("foo", location.clone());
        
        assert_eq!(error.message, "Undefined function: 'foo'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_parser_error_undefined_context() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = ParserError::undefined_context("MyContext", location.clone());
        
        assert_eq!(error.message, "Undefined context: 'MyContext'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_parser_error_invalid_assignment_target() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = ParserError::invalid_assignment_target(location.clone());
        
        assert_eq!(error.message, "Invalid assignment target");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_parser_error_invalid_argument_count() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = ParserError::invalid_argument_count("foo", 2, 3, location.clone());
        
        assert_eq!(
            error.message,
            "Invalid argument count for function 'foo': expected 2, got 3"
        );
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_parser_error_display() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = ParserError::new("Test error", location);
        
        assert_eq!(
            format!("{}", error),
            "Parser error: Test error at test.llm:1:1"
        );
    }
}
