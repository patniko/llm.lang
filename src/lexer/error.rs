//! Error definitions for the LLM.lang lexer
//!
//! This module defines the error types and structures used by the lexer.

use crate::utils::SourceLocation;
use std::fmt;

/// An error that can occur during lexical analysis
#[derive(Debug, Clone)]
pub struct LexerError {
    /// The error message
    pub message: String,
    
    /// The error location
    pub location: SourceLocation,
}

impl LexerError {
    /// Create a new lexer error
    pub fn new(message: &str, location: SourceLocation) -> Self {
        Self {
            message: message.to_string(),
            location,
        }
    }
    
    /// Create a new lexer error with a formatted message
    pub fn with_format(format: fmt::Arguments<'_>, location: SourceLocation) -> Self {
        Self {
            message: format!("{}", format),
            location,
        }
    }
    
    /// Create a new "unexpected character" error
    pub fn unexpected_character(c: char, location: SourceLocation) -> Self {
        Self::new(&format!("Unexpected character: '{}'", c), location)
    }
    
    /// Create a new "unexpected end of input" error
    pub fn unexpected_end_of_input(location: SourceLocation) -> Self {
        Self::new("Unexpected end of input", location)
    }
    
    /// Create a new "unterminated string" error
    pub fn unterminated_string(location: SourceLocation) -> Self {
        Self::new("Unterminated string literal", location)
    }
    
    /// Create a new "unterminated block comment" error
    pub fn unterminated_block_comment(location: SourceLocation) -> Self {
        Self::new("Unterminated block comment", location)
    }
    
    /// Create a new "unterminated natural language" error
    pub fn unterminated_natural_language(location: SourceLocation) -> Self {
        Self::new("Unterminated natural language expression", location)
    }
    
    /// Create a new "unterminated semantic type" error
    pub fn unterminated_semantic_type(location: SourceLocation) -> Self {
        Self::new("Unterminated semantic type", location)
    }
    
    /// Create a new "invalid escape sequence" error
    pub fn invalid_escape_sequence(sequence: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Invalid escape sequence: '{}'", sequence), location)
    }
    
    /// Create a new "invalid number" error
    pub fn invalid_number(number: &str, location: SourceLocation) -> Self {
        Self::new(&format!("Invalid number: '{}'", number), location)
    }
}

impl std::error::Error for LexerError {}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexer error: {} at {}:{}:{}",
            self.message,
            self.location.file,
            self.location.start_line,
            self.location.start_column
        )
    }
}

/// A result type for lexical analysis
pub type LexerResult<T> = Result<T, LexerError>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lexer_error_new() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = LexerError::new("Test error", location.clone());
        
        assert_eq!(error.message, "Test error");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_lexer_error_with_format() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = LexerError::with_format(format_args!("Error: {}", 42), location.clone());
        
        assert_eq!(error.message, "Error: 42");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_lexer_error_unexpected_character() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = LexerError::unexpected_character('$', location.clone());
        
        assert_eq!(error.message, "Unexpected character: '$'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_lexer_error_unexpected_end_of_input() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = LexerError::unexpected_end_of_input(location.clone());
        
        assert_eq!(error.message, "Unexpected end of input");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_lexer_error_unterminated_string() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = LexerError::unterminated_string(location.clone());
        
        assert_eq!(error.message, "Unterminated string literal");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_lexer_error_unterminated_block_comment() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = LexerError::unterminated_block_comment(location.clone());
        
        assert_eq!(error.message, "Unterminated block comment");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_lexer_error_unterminated_natural_language() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = LexerError::unterminated_natural_language(location.clone());
        
        assert_eq!(error.message, "Unterminated natural language expression");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_lexer_error_unterminated_semantic_type() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = LexerError::unterminated_semantic_type(location.clone());
        
        assert_eq!(error.message, "Unterminated semantic type");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_lexer_error_invalid_escape_sequence() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = LexerError::invalid_escape_sequence("\\z", location.clone());
        
        assert_eq!(error.message, "Invalid escape sequence: '\\z'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_lexer_error_invalid_number() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = LexerError::invalid_number("123.456.789", location.clone());
        
        assert_eq!(error.message, "Invalid number: '123.456.789'");
        assert_eq!(error.location, location);
    }
    
    #[test]
    fn test_lexer_error_display() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let error = LexerError::new("Test error", location);
        
        assert_eq!(
            format!("{}", error),
            "Lexer error: Test error at test.llm:1:1"
        );
    }
}
