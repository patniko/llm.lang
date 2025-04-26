//! Token definitions for the LLM.lang lexer
//!
//! This module defines the token types and structures used by the lexer.

use crate::utils::SourceLocation;

/// A token in the LLM.lang language
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The token type
    pub kind: TokenKind,
    
    /// The token value
    pub value: String,
    
    /// The token location
    pub location: SourceLocation,
}

/// Token types
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// A keyword
    Keyword,
    
    /// An identifier
    Identifier,
    
    /// An integer literal
    IntLiteral,
    
    /// A floating-point literal
    FloatLiteral,
    
    /// A string literal
    StringLiteral,
    
    /// An operator
    Operator,
    
    /// A delimiter
    Delimiter,
    
    /// A semantic token (e.g., @remember, @recall)
    Semantic,
    
    /// A natural language expression
    NaturalLanguage,
    
    /// A semantic type (e.g., ~EmailAddress~)
    SemanticType,
    
    /// End of file
    Eof,
}

impl Token {
    /// Create a new token
    pub fn new(kind: TokenKind, value: &str, location: SourceLocation) -> Self {
        Self {
            kind,
            value: value.to_string(),
            location,
        }
    }
    
    /// Check if the token is a keyword
    pub fn is_keyword(&self) -> bool {
        self.kind == TokenKind::Keyword
    }
    
    /// Check if the token is an identifier
    pub fn is_identifier(&self) -> bool {
        self.kind == TokenKind::Identifier
    }
    
    /// Check if the token is a specific keyword
    pub fn is_keyword_with_value(&self, value: &str) -> bool {
        self.kind == TokenKind::Keyword && self.value == value
    }
    
    /// Check if the token is a specific identifier
    pub fn is_identifier_with_value(&self, value: &str) -> bool {
        self.kind == TokenKind::Identifier && self.value == value
    }
    
    /// Check if the token is a specific delimiter
    pub fn is_delimiter(&self, value: &str) -> bool {
        self.kind == TokenKind::Delimiter && self.value == value
    }
    
    /// Check if the token is a specific operator
    pub fn is_operator(&self, value: &str) -> bool {
        self.kind == TokenKind::Operator && self.value == value
    }
    
    /// Check if the token is a specific semantic token
    pub fn is_semantic(&self, value: &str) -> bool {
        self.kind == TokenKind::Semantic && self.value == value
    }
    
    /// Check if the token is the end of file
    pub fn is_eof(&self) -> bool {
        self.kind == TokenKind::Eof
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.kind, self.value)
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Keyword => write!(f, "Keyword"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::IntLiteral => write!(f, "IntLiteral"),
            TokenKind::FloatLiteral => write!(f, "FloatLiteral"),
            TokenKind::StringLiteral => write!(f, "StringLiteral"),
            TokenKind::Operator => write!(f, "Operator"),
            TokenKind::Delimiter => write!(f, "Delimiter"),
            TokenKind::Semantic => write!(f, "Semantic"),
            TokenKind::NaturalLanguage => write!(f, "NaturalLanguage"),
            TokenKind::SemanticType => write!(f, "SemanticType"),
            TokenKind::Eof => write!(f, "Eof"),
        }
    }
}

/// Keywords in the LLM.lang language
pub const KEYWORDS: &[&str] = &[
    // Context-related keywords
    "context",
    "with",
    "within",
    
    // Function-related keywords
    "fn",
    "return",
    
    // Variable-related keywords
    "var",
    "const",
    
    // Type-related keywords
    "Int",
    "Float",
    "String",
    "Bool",
    "List",
    "Map",
    "Vector",
    
    // Control flow keywords
    "if",
    "else",
    "when",
    "otherwise",
    "for",
    "in",
    
    // Parallel execution keywords
    "parallel",
    "path",
    "select",
    "fastest",
    "best",
    "all",
    
    // Example-driven programming keywords
    "examples",
    "for",
    "transform",
    "into",
    
    // Vector operation keywords
    "vector",
    "embed",
    "apply",
    "to",
    
    // Natural language keywords
    "intent",
    
    // Boolean literals
    "true",
    "false",
    
    // Null literal
    "null",
    
    // Logical operators
    "and",
    "or",
    "not",
];

/// Operators in the LLM.lang language
pub const OPERATORS: &[&str] = &[
    // Arithmetic operators
    "+", "-", "*", "/", "%",
    
    // Assignment operators
    "=", "+=", "-=", "*=", "/=", "%=",
    
    // Comparison operators
    "==", "!=", "<", ">", "<=", ">=",
    
    // Logical operators
    "&&", "||", "!",
    
    // Arrow operators
    "->", "=>",
    
    // Other operators
    ".", ":", "::",
];

/// Delimiters in the LLM.lang language
pub const DELIMITERS: &[&str] = &[
    // Parentheses
    "(", ")",
    
    // Braces
    "{", "}",
    
    // Brackets
    "[", "]",
    
    // Other delimiters
    ";", ",",
];

/// Semantic tokens in the LLM.lang language
pub const SEMANTIC_TOKENS: &[&str] = &[
    // Memory-related tokens
    "@remember",
    "@recall",
    
    // Context-related tokens
    "@context",
    
    // Other semantic tokens
    "@type",
    "@doc",
    "@example",
    "@test",
];

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_new() {
        let location = SourceLocation::new(1, 1, 1, 5, "test.llm");
        let token = Token::new(TokenKind::Keyword, "context", location.clone());
        
        assert_eq!(token.kind, TokenKind::Keyword);
        assert_eq!(token.value, "context");
        assert_eq!(token.location, location);
    }
    
    #[test]
    fn test_token_is_keyword() {
        let location = SourceLocation::new(1, 1, 1, 5, "test.llm");
        let token = Token::new(TokenKind::Keyword, "context", location);
        
        assert!(token.is_keyword());
        assert!(!token.is_identifier());
        assert!(token.is_keyword_with_value("context"));
        assert!(!token.is_keyword_with_value("fn"));
    }
    
    #[test]
    fn test_token_is_identifier() {
        let location = SourceLocation::new(1, 1, 1, 5, "test.llm");
        let token = Token::new(TokenKind::Identifier, "foo", location);
        
        assert!(!token.is_keyword());
        assert!(token.is_identifier());
        assert!(token.is_identifier_with_value("foo"));
        assert!(!token.is_identifier_with_value("bar"));
    }
    
    #[test]
    fn test_token_is_delimiter() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let token = Token::new(TokenKind::Delimiter, "{", location);
        
        assert!(token.is_delimiter("{"));
        assert!(!token.is_delimiter("}"));
    }
    
    #[test]
    fn test_token_is_operator() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let token = Token::new(TokenKind::Operator, "+", location);
        
        assert!(token.is_operator("+"));
        assert!(!token.is_operator("-"));
    }
    
    #[test]
    fn test_token_is_semantic() {
        let location = SourceLocation::new(1, 1, 1, 9, "test.llm");
        let token = Token::new(TokenKind::Semantic, "@remember", location);
        
        assert!(token.is_semantic("@remember"));
        assert!(!token.is_semantic("@recall"));
    }
    
    #[test]
    fn test_token_is_eof() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let token = Token::new(TokenKind::Eof, "", location);
        
        assert!(token.is_eof());
    }
    
    #[test]
    fn test_token_display() {
        let location = SourceLocation::new(1, 1, 1, 5, "test.llm");
        let token = Token::new(TokenKind::Keyword, "context", location);
        
        assert_eq!(format!("{}", token), "Keyword(context)");
    }
    
    #[test]
    fn test_token_kind_display() {
        assert_eq!(format!("{}", TokenKind::Keyword), "Keyword");
        assert_eq!(format!("{}", TokenKind::Identifier), "Identifier");
        assert_eq!(format!("{}", TokenKind::IntLiteral), "IntLiteral");
        assert_eq!(format!("{}", TokenKind::FloatLiteral), "FloatLiteral");
        assert_eq!(format!("{}", TokenKind::StringLiteral), "StringLiteral");
        assert_eq!(format!("{}", TokenKind::Operator), "Operator");
        assert_eq!(format!("{}", TokenKind::Delimiter), "Delimiter");
        assert_eq!(format!("{}", TokenKind::Semantic), "Semantic");
        assert_eq!(format!("{}", TokenKind::NaturalLanguage), "NaturalLanguage");
        assert_eq!(format!("{}", TokenKind::SemanticType), "SemanticType");
        assert_eq!(format!("{}", TokenKind::Eof), "Eof");
    }
}
