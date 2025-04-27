//! Lexical analysis module for LLM.lang
//!
//! This module provides the lexer for the LLM.lang programming language,
//! which converts source code into a stream of tokens.

pub mod token;
pub mod error;

use std::iter::Peekable;
use std::str::Chars;

use self::error::LexerError;
use self::token::{Token, TokenKind};
use crate::utils::SourceLocation;

/// A lexer for the LLM.lang language
pub struct Lexer {
    /// The source code
    source: String,
    
    /// The current position in the source code
    position: usize,
    
    /// The current line number
    line: usize,
    
    /// The current column number
    column: usize,
    
    /// The source file name
    file: String,
    
    /// Character iterator
    chars: Peekable<Chars<'static>>,
}

impl Lexer {
    /// Create a new lexer
    pub fn new(source: &str) -> Self {
        // This is a bit of a hack to make the lifetime work
        // In a real implementation, we would use a more robust approach
        let source_owned = source.to_string();
        let chars = unsafe {
            std::mem::transmute::<Peekable<Chars>, Peekable<Chars<'static>>>(
                source_owned.chars().peekable()
            )
        };
        
        Self {
            source: source.to_string(),
            position: 0,
            line: 1,
            column: 1,
            file: "<input>".to_string(),
            chars,
        }
    }
    
    /// Set the source file name
    pub fn set_file(&mut self, file: &str) {
        self.file = file.to_string();
    }
    
    /// Tokenize the source code
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        
        // Reset the lexer state
        self.position = 0;
        self.line = 1;
        self.column = 1;
        self.chars = unsafe {
            std::mem::transmute::<Peekable<Chars>, Peekable<Chars<'static>>>(
                self.source.chars().peekable()
            )
        };
        
        // Tokenize until we reach the end of the source code
        loop {
            // Skip whitespace
            self.skip_whitespace();
            
            // Check if we've reached the end of the source code
            if self.peek().is_none() {
                // Add an EOF token
                tokens.push(Token {
                    kind: TokenKind::Eof,
                    value: "".to_string(),
                    location: self.current_location(),
                });
                
                break;
            }
            
            // Get the next token
            let token = self.next_token()?;
            
            // Add the token to the list
            tokens.push(token);
        }
        
        Ok(tokens)
    }
    
    /// Get the next token
    fn next_token(&mut self) -> Result<Token, LexerError> {
        // Get the current character
        let c = match self.peek() {
            Some(c) => c,
            None => {
                return Err(LexerError {
                    message: "Unexpected end of input".to_string(),
                    location: self.current_location(),
                });
            }
        };
        
        // Get the token based on the current character
        let token = match c {
            // Whitespace (should be handled by skip_whitespace, but just in case)
            c if self.is_whitespace(c) => {
                self.skip_whitespace();
                return self.next_token();
            }
            
            // Comments
            '/' if self.peek_next() == Some('/') => {
                self.skip_line_comment();
                return self.next_token();
            }
            '/' if self.peek_next() == Some('*') => {
                self.skip_block_comment()?;
                return self.next_token();
            }
            
            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => self.identifier_or_keyword(),
            
            // Numbers
            '0'..='9' => self.number(),
            
            // Strings
            '"' => self.string(),
            
            // Natural language
            '#' if self.peek_next() == Some('"') => self.natural_language(),
            
            // Semantic tokens
            '@' => self.semantic_token(),
            
            // Semantic types
            '~' => self.semantic_type(),
            
            // Operators
            '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' | '^' => self.operator(),
            
            // Delimiters
            '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | '.' | ':' => self.delimiter(),
            
            // Unknown character
            _ => {
                // Get surrounding context (up to 10 chars before and after)
                let position = self.position;
                let start = if position > 10 { position - 10 } else { 0 };
                let end = std::cmp::min(position + 10, self.source.len());
                let context = &self.source[start..end];
                
                // Determine if it's a control or whitespace character
                let char_type = if c.is_control() {
                    "control character"
                } else if c.is_whitespace() {
                    "whitespace character"
                } else {
                    "character"
                };
                
                return Err(LexerError {
                    message: format!(
                        "Unexpected {}: '{}' (U+{:04X}, hex: {:02X}) at position {}:{}\nContext: \"{}\"", 
                        char_type, 
                        c, 
                        c as u32, 
                        c as u32,
                        self.line,
                        self.column,
                        context
                    ),
                    location: self.current_location(),
                });
            }
        };
        
        Ok(token)
    }
    
    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            
            // Update line and column numbers
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            
            // Consume the character
            self.next();
        }
    }
    
    /// Check if a character is a whitespace character
    fn is_whitespace(&self, c: char) -> bool {
        // Consider all standard whitespace plus newlines
        c.is_whitespace()
    }
    
    /// Skip a line comment
    fn skip_line_comment(&mut self) {
        // Consume the '//'
        self.next();
        self.next();
        
        // Skip until the end of the line
        while let Some(c) = self.peek() {
            if c == '\n' {
                // Consume the newline character too
                self.next();
                break;
            }
            
            self.next();
        }
    }
    
    /// Skip a block comment
    fn skip_block_comment(&mut self) -> Result<(), LexerError> {
        // Consume the '/*'
        self.next();
        self.next();
        
        // Skip until the end of the comment
        let mut nesting = 1;
        
        while nesting > 0 {
            match self.peek() {
                Some('/') if self.peek_next() == Some('*') => {
                    self.next();
                    self.next();
                    nesting += 1;
                }
                Some('*') if self.peek_next() == Some('/') => {
                    self.next();
                    self.next();
                    nesting -= 1;
                }
                Some('\n') => {
                    self.next();
                    self.line += 1;
                    self.column = 1;
                }
                Some(_) => {
                    self.next();
                }
                None => {
                    return Err(LexerError {
                        message: "Unterminated block comment".to_string(),
                        location: self.current_location(),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    /// Tokenize an identifier or keyword
    fn identifier_or_keyword(&mut self) -> Token {
        let start_position = self.position;
        let start_line = self.line;
        let start_column = self.column;
        
        // Consume the first character
        self.next();
        
        // Consume the rest of the identifier
        while let Some(c) = self.peek() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            
            self.next();
        }
        
        // Get the identifier
        let identifier = &self.source[start_position..self.position];
        
        // Check if it's a keyword
        let kind = match identifier {
            "context" | "fn" | "var" | "if" | "else" | "when" | "otherwise" | "parallel" | "select" |
            "return" | "with" | "within" | "intent" | "examples" | "transform" | "into" | "apply" |
            "for" | "in" | "true" | "false" | "null" | "and" | "or" | "not" | "vector" | "to" |
            "Int" | "Float" | "String" | "Bool" | "List" | "Map" | "Vector" | "Context" |
            "fastest" | "best" | "all" | "path" => TokenKind::Keyword,
            _ => TokenKind::Identifier,
        };
        
        // Create the token
        Token {
            kind,
            value: identifier.to_string(),
            location: SourceLocation::new(
                start_line,
                start_column,
                self.line,
                self.column,
                &self.file,
            ),
        }
    }
    
    /// Tokenize a number
    fn number(&mut self) -> Token {
        let start_position = self.position;
        let start_line = self.line;
        let start_column = self.column;
        
        // Consume the first digit
        self.next();
        
        // Consume the rest of the number
        let mut is_float = false;
        
        while let Some(c) = self.peek() {
            if c == '.' && !is_float && self.peek_next().map_or(false, |next| next.is_digit(10)) {
                is_float = true;
                self.next();
            } else if c.is_digit(10) {
                self.next();
            } else if c == 'e' || c == 'E' {
                // Handle scientific notation
                self.next();
                
                // Handle optional sign
                if self.peek() == Some('+') || self.peek() == Some('-') {
                    self.next();
                }
                
                // Ensure there's at least one digit in the exponent
                if !self.peek().map_or(false, |c| c.is_digit(10)) {
                    break;
                }
                
                // Consume the exponent
                while self.peek().map_or(false, |c| c.is_digit(10)) {
                    self.next();
                }
                
                is_float = true;
                break;
            } else {
                break;
            }
        }
        
        // Get the number
        let number = &self.source[start_position..self.position];
        
        // Create the token
        Token {
            kind: if is_float { TokenKind::FloatLiteral } else { TokenKind::IntLiteral },
            value: number.to_string(),
            location: SourceLocation::new(
                start_line,
                start_column,
                self.line,
                self.column,
                &self.file,
            ),
        }
    }
    
    /// Tokenize a string
    fn string(&mut self) -> Token {
        let start_position = self.position;
        let start_line = self.line;
        let start_column = self.column;
        
        // Consume the opening quote
        self.next();
        
        // Consume the string content
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            
            if c == '\\' {
                // Handle escape sequences
                self.next();
                
                // Ensure there's a character after the backslash
                if self.peek().is_none() {
                    break;
                }
                
                // Consume the escaped character
                self.next();
                continue;
            }
            
            // Consume the character
            self.next();
        }
        
        // Consume the closing quote
        if self.peek() == Some('"') {
            self.next();
        }
        
        // Get the string (including quotes)
        let string = &self.source[start_position..self.position];
        
        // Create the token
        Token {
            kind: TokenKind::StringLiteral,
            value: string.to_string(),
            location: SourceLocation::new(
                start_line,
                start_column,
                self.line,
                self.column,
                &self.file,
            ),
        }
    }
    
    /// Tokenize a natural language expression
    fn natural_language(&mut self) -> Token {
        let start_position = self.position;
        let start_line = self.line;
        let start_column = self.column;
        
        // Consume the '#"'
        self.next();
        self.next();
        
        // Consume the natural language content
        while let Some(c) = self.peek() {
            if c == '"' && self.peek_next() == Some('#') {
                break;
            }
            
            // Consume the character
            self.next();
        }
        
        // Consume the closing '"#'
        if self.peek() == Some('"') && self.peek_next() == Some('#') {
            self.next();
            self.next();
        }
        
        // Get the natural language expression (including delimiters)
        let natural = &self.source[start_position..self.position];
        
        // Create the token
        Token {
            kind: TokenKind::NaturalLanguage,
            value: natural.to_string(),
            location: SourceLocation::new(
                start_line,
                start_column,
                self.line,
                self.column,
                &self.file,
            ),
        }
    }
    
    /// Tokenize a semantic token
    fn semantic_token(&mut self) -> Token {
        let start_position = self.position;
        let start_line = self.line;
        let start_column = self.column;
        
        // Consume the '@'
        self.next();
        
        // Consume the rest of the token
        while let Some(c) = self.peek() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            
            self.next();
        }
        
        // Get the semantic token
        let semantic = &self.source[start_position..self.position];
        
        // Create the token
        Token {
            kind: TokenKind::Semantic,
            value: semantic.to_string(),
            location: SourceLocation::new(
                start_line,
                start_column,
                self.line,
                self.column,
                &self.file,
            ),
        }
    }
    
    /// Tokenize a semantic type
    fn semantic_type(&mut self) -> Token {
        let start_position = self.position;
        let start_line = self.line;
        let start_column = self.column;
        
        // Consume the opening '~'
        self.next();
        
        // Consume the type name
        while let Some(c) = self.peek() {
            if c == '~' {
                break;
            }
            
            // Consume the character
            self.next();
        }
        
        // Consume the closing '~'
        if self.peek() == Some('~') {
            self.next();
        }
        
        // Get the semantic type (including delimiters)
        let semantic_type = &self.source[start_position..self.position];
        
        // Create the token
        Token {
            kind: TokenKind::SemanticType,
            value: semantic_type.to_string(),
            location: SourceLocation::new(
                start_line,
                start_column,
                self.line,
                self.column,
                &self.file,
            ),
        }
    }
    
    /// Tokenize an operator
    fn operator(&mut self) -> Token {
        let start_position = self.position;
        let start_line = self.line;
        let start_column = self.column;
        
        // Consume the first character
        let first = self.next().unwrap();
        
        // Check for multi-character operators
        match first {
            '+' => {
                if self.peek() == Some('=') {
                    self.next();
                }
            }
            '-' => {
                if self.peek() == Some('=') || self.peek() == Some('>') {
                    self.next();
                }
            }
            '*' => {
                if self.peek() == Some('=') {
                    self.next();
                }
            }
            '/' => {
                if self.peek() == Some('=') {
                    self.next();
                }
            }
            '%' => {
                if self.peek() == Some('=') {
                    self.next();
                }
            }
            '=' => {
                if self.peek() == Some('=') || self.peek() == Some('>') {
                    self.next();
                }
            }
            '!' => {
                if self.peek() == Some('=') {
                    self.next();
                }
            }
            '<' => {
                if self.peek() == Some('=') {
                    self.next();
                }
            }
            '>' => {
                if self.peek() == Some('=') {
                    self.next();
                }
            }
            '&' => {
                if self.peek() == Some('&') {
                    self.next();
                }
            }
            '|' => {
                if self.peek() == Some('|') {
                    self.next();
                }
            }
            _ => {}
        }
        
        // Get the operator
        let operator = &self.source[start_position..self.position];
        
        // Create the token
        Token {
            kind: TokenKind::Operator,
            value: operator.to_string(),
            location: SourceLocation::new(
                start_line,
                start_column,
                self.line,
                self.column,
                &self.file,
            ),
        }
    }
    
    /// Tokenize a delimiter
    fn delimiter(&mut self) -> Token {
        let start_position = self.position;
        let start_line = self.line;
        let start_column = self.column;
        
        // Consume the delimiter
        self.next();
        
        // Get the delimiter
        let delimiter = &self.source[start_position..self.position];
        
        // Create the token
        Token {
            kind: TokenKind::Delimiter,
            value: delimiter.to_string(),
            location: SourceLocation::new(
                start_line,
                start_column,
                self.line,
                self.column,
                &self.file,
            ),
        }
    }
    
    /// Get the current source location
    fn current_location(&self) -> SourceLocation {
        SourceLocation::new(
            self.line,
            self.column,
            self.line,
            self.column,
            &self.file,
        )
    }
    
    /// Peek at the current character without consuming it
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }
    
    /// Peek at the next character without consuming it
    fn peek_next(&mut self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next()
    }
    
    /// Consume the current character and return it
    fn next(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        
        // Update the position based on the character's UTF-8 encoding
        self.position += c.len_utf8();
        
        // Update the line and column numbers
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        
        Some(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenize_empty() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_identifier() {
        let mut lexer = Lexer::new("identifier");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].value, "identifier");
        assert_eq!(tokens[1].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_keyword() {
        let mut lexer = Lexer::new("context");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Keyword);
        assert_eq!(tokens[0].value, "context");
        assert_eq!(tokens[1].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_number() {
        let mut lexer = Lexer::new("123 3.14");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::IntLiteral);
        assert_eq!(tokens[0].value, "123");
        assert_eq!(tokens[1].kind, TokenKind::FloatLiteral);
        assert_eq!(tokens[1].value, "3.14");
        assert_eq!(tokens[2].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_string() {
        let mut lexer = Lexer::new("\"hello\"");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
        assert_eq!(tokens[0].value, "\"hello\"");
        assert_eq!(tokens[1].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_natural_language() {
        let mut lexer = Lexer::new("#\"Find all users\"#");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::NaturalLanguage);
        assert_eq!(tokens[0].value, "#\"Find all users\"#");
        assert_eq!(tokens[1].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_semantic_token() {
        let mut lexer = Lexer::new("@remember");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Semantic);
        assert_eq!(tokens[0].value, "@remember");
        assert_eq!(tokens[1].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_semantic_type() {
        let mut lexer = Lexer::new("~EmailAddress~");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::SemanticType);
        assert_eq!(tokens[0].value, "~EmailAddress~");
        assert_eq!(tokens[1].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_operator() {
        let mut lexer = Lexer::new("+ - * / % = == != < > <= >= && ||");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 14);
        assert_eq!(tokens[0].kind, TokenKind::Operator);
        assert_eq!(tokens[0].value, "+");
        assert_eq!(tokens[1].kind, TokenKind::Operator);
        assert_eq!(tokens[1].value, "-");
        assert_eq!(tokens[2].kind, TokenKind::Operator);
        assert_eq!(tokens[2].value, "*");
        assert_eq!(tokens[3].kind, TokenKind::Operator);
        assert_eq!(tokens[3].value, "/");
        assert_eq!(tokens[4].kind, TokenKind::Operator);
        assert_eq!(tokens[4].value, "%");
        assert_eq!(tokens[5].kind, TokenKind::Operator);
        assert_eq!(tokens[5].value, "=");
        assert_eq!(tokens[6].kind, TokenKind::Operator);
        assert_eq!(tokens[6].value, "==");
        assert_eq!(tokens[7].kind, TokenKind::Operator);
        assert_eq!(tokens[7].value, "!=");
        assert_eq!(tokens[8].kind, TokenKind::Operator);
        assert_eq!(tokens[8].value, "<");
        assert_eq!(tokens[9].kind, TokenKind::Operator);
        assert_eq!(tokens[9].value, ">");
        assert_eq!(tokens[10].kind, TokenKind::Operator);
        assert_eq!(tokens[10].value, "<=");
        assert_eq!(tokens[11].kind, TokenKind::Operator);
        assert_eq!(tokens[11].value, ">=");
        assert_eq!(tokens[12].kind, TokenKind::Operator);
        assert_eq!(tokens[12].value, "&&");
        assert_eq!(tokens[13].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_delimiter() {
        let mut lexer = Lexer::new("( ) { } [ ] ; , . :");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0].kind, TokenKind::Delimiter);
        assert_eq!(tokens[0].value, "(");
        assert_eq!(tokens[1].kind, TokenKind::Delimiter);
        assert_eq!(tokens[1].value, ")");
        assert_eq!(tokens[2].kind, TokenKind::Delimiter);
        assert_eq!(tokens[2].value, "{");
        assert_eq!(tokens[3].kind, TokenKind::Delimiter);
        assert_eq!(tokens[3].value, "}");
        assert_eq!(tokens[4].kind, TokenKind::Delimiter);
        assert_eq!(tokens[4].value, "[");
        assert_eq!(tokens[5].kind, TokenKind::Delimiter);
        assert_eq!(tokens[5].value, "]");
        assert_eq!(tokens[6].kind, TokenKind::Delimiter);
        assert_eq!(tokens[6].value, ";");
        assert_eq!(tokens[7].kind, TokenKind::Delimiter);
        assert_eq!(tokens[7].value, ",");
        assert_eq!(tokens[8].kind, TokenKind::Delimiter);
        assert_eq!(tokens[8].value, ".");
        assert_eq!(tokens[9].kind, TokenKind::Delimiter);
        assert_eq!(tokens[9].value, ":");
        assert_eq!(tokens[10].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_comment() {
        let mut lexer = Lexer::new("// This is a comment\nidentifier");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].value, "identifier");
        assert_eq!(tokens[1].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_block_comment() {
        let mut lexer = Lexer::new("/* This is a\nblock comment */identifier");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].value, "identifier");
        assert_eq!(tokens[1].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_hello_world() {
        let source = r#"
            context MainProgram {
                fn main() {
                    print("Hello, World!");
                }
            }
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        // Check that we have the expected number of tokens
        // (not including whitespace and comments)
        assert_eq!(tokens.len(), 13);
        
        // Check the token types
        assert_eq!(tokens[0].kind, TokenKind::Keyword); // context
        assert_eq!(tokens[1].kind, TokenKind::Identifier); // MainProgram
        assert_eq!(tokens[2].kind, TokenKind::Delimiter); // {
        assert_eq!(tokens[3].kind, TokenKind::Keyword); // fn
        assert_eq!(tokens[4].kind, TokenKind::Identifier); // main
        assert_eq!(tokens[5].kind, TokenKind::Delimiter); // (
        assert_eq!(tokens[6].kind, TokenKind::Delimiter); // )
        assert_eq!(tokens[7].kind, TokenKind::Delimiter); // {
        assert_eq!(tokens[8].kind, TokenKind::Identifier); // print
        assert_eq!(tokens[9].kind, TokenKind::Delimiter); // (
        assert_eq!(tokens[10].kind, TokenKind::StringLiteral); // "Hello, World!"
        assert_eq!(tokens[11].kind, TokenKind::Delimiter); // )
        assert_eq!(tokens[12].kind, TokenKind::Eof);
    }
    
    #[test]
    fn test_tokenize_with_empty_lines() {
        // Test with a comment followed by an empty line
        let source = "// This is a comment\n\ncontext MainProgram {}";
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        // Check that we have the expected number of tokens
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::Keyword); // context
        assert_eq!(tokens[1].kind, TokenKind::Identifier); // MainProgram
        assert_eq!(tokens[2].kind, TokenKind::Delimiter); // {
        assert_eq!(tokens[3].kind, TokenKind::Eof);
        
        // Test with multiple consecutive empty lines
        let source = "\n\n\ncontext MainProgram {}";
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        // Check that we have the expected number of tokens
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::Keyword); // context
        assert_eq!(tokens[1].kind, TokenKind::Identifier); // MainProgram
        assert_eq!(tokens[2].kind, TokenKind::Delimiter); // {
        assert_eq!(tokens[3].kind, TokenKind::Eof);
    }
}
