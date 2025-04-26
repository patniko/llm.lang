//! Utility module for LLM.lang
//!
//! This module provides utility functions and types for the LLM.lang programming language.

/// A source location in a file
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    /// The start line
    pub start_line: usize,
    
    /// The start column
    pub start_column: usize,
    
    /// The end line
    pub end_line: usize,
    
    /// The end column
    pub end_column: usize,
    
    /// The source file
    pub file: String,
}

impl SourceLocation {
    /// Create a new source location
    pub fn new(start_line: usize, start_column: usize, end_line: usize, end_column: usize, file: &str) -> Self {
        Self {
            start_line,
            start_column,
            end_line,
            end_column,
            file: file.to_string(),
        }
    }
    
    /// Create a new source location from a single position
    pub fn from_position(line: usize, column: usize, file: &str) -> Self {
        Self::new(line, column, line, column, file)
    }
    
    /// Check if this location contains another location
    pub fn contains(&self, other: &SourceLocation) -> bool {
        if self.file != other.file {
            return false;
        }
        
        if self.start_line > other.start_line || self.end_line < other.end_line {
            return false;
        }
        
        if self.start_line == other.start_line && self.start_column > other.start_column {
            return false;
        }
        
        if self.end_line == other.end_line && self.end_column < other.end_column {
            return false;
        }
        
        true
    }
    
    /// Merge this location with another location
    pub fn merge(&self, other: &SourceLocation) -> SourceLocation {
        if self.file != other.file {
            return self.clone();
        }
        
        let start_line = std::cmp::min(self.start_line, other.start_line);
        let start_column = if self.start_line < other.start_line {
            self.start_column
        } else if self.start_line > other.start_line {
            other.start_column
        } else {
            std::cmp::min(self.start_column, other.start_column)
        };
        
        let end_line = std::cmp::max(self.end_line, other.end_line);
        let end_column = if self.end_line > other.end_line {
            self.end_column
        } else if self.end_line < other.end_line {
            other.end_column
        } else {
            std::cmp::max(self.end_column, other.end_column)
        };
        
        SourceLocation::new(start_line, start_column, end_line, end_column, &self.file)
    }
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}-{}:{}",
            self.file, self.start_line, self.start_column, self.end_line, self.end_column
        )
    }
}

/// A result type with a source location
pub type LocatedResult<T> = Result<T, (String, SourceLocation)>;

/// A span of text in a source file
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    /// The start offset
    pub start: usize,
    
    /// The end offset
    pub end: usize,
    
    /// The source file
    pub file: String,
}

impl Span {
    /// Create a new span
    pub fn new(start: usize, end: usize, file: &str) -> Self {
        Self {
            start,
            end,
            file: file.to_string(),
        }
    }
    
    /// Convert a span to a source location
    pub fn to_location(&self, source: &str) -> SourceLocation {
        let mut start_line = 1;
        let mut start_column = 1;
        let mut end_line = 1;
        let mut end_column = 1;
        
        let mut current_line = 1;
        let mut current_column = 1;
        
        for (i, c) in source.char_indices() {
            if i == self.start {
                start_line = current_line;
                start_column = current_column;
            }
            
            if i == self.end {
                end_line = current_line;
                end_column = current_column;
                break;
            }
            
            if c == '\n' {
                current_line += 1;
                current_column = 1;
            } else {
                current_column += 1;
            }
        }
        
        SourceLocation::new(start_line, start_column, end_line, end_column, &self.file)
    }
}

/// A utility for working with source code
pub struct SourceCode {
    /// The source code
    pub source: String,
    
    /// The file name
    pub file: String,
}

impl SourceCode {
    /// Create a new source code
    pub fn new(source: &str, file: &str) -> Self {
        Self {
            source: source.to_string(),
            file: file.to_string(),
        }
    }
    
    /// Get a line from the source code
    pub fn get_line(&self, line: usize) -> Option<&str> {
        self.source.lines().nth(line - 1)
    }
    
    /// Get a span from the source code
    pub fn get_span(&self, start: usize, end: usize) -> Span {
        Span::new(start, end, &self.file)
    }
    
    /// Get a source location from a span
    pub fn get_location(&self, span: &Span) -> SourceLocation {
        span.to_location(&self.source)
    }
    
    /// Get a source location from a range
    pub fn get_location_from_range(&self, start: usize, end: usize) -> SourceLocation {
        let span = self.get_span(start, end);
        self.get_location(&span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_source_location_new() {
        let location = SourceLocation::new(1, 1, 1, 5, "test.llm");
        
        assert_eq!(location.start_line, 1);
        assert_eq!(location.start_column, 1);
        assert_eq!(location.end_line, 1);
        assert_eq!(location.end_column, 5);
        assert_eq!(location.file, "test.llm");
    }
    
    #[test]
    fn test_source_location_from_position() {
        let location = SourceLocation::from_position(1, 1, "test.llm");
        
        assert_eq!(location.start_line, 1);
        assert_eq!(location.start_column, 1);
        assert_eq!(location.end_line, 1);
        assert_eq!(location.end_column, 1);
        assert_eq!(location.file, "test.llm");
    }
    
    #[test]
    fn test_source_location_contains() {
        let location1 = SourceLocation::new(1, 1, 2, 5, "test.llm");
        let location2 = SourceLocation::new(1, 2, 2, 3, "test.llm");
        let location3 = SourceLocation::new(1, 1, 2, 5, "other.llm");
        
        assert!(location1.contains(&location2));
        assert!(!location2.contains(&location1));
        assert!(!location1.contains(&location3));
    }
    
    #[test]
    fn test_source_location_merge() {
        let location1 = SourceLocation::new(1, 1, 2, 5, "test.llm");
        let location2 = SourceLocation::new(2, 3, 3, 7, "test.llm");
        let location3 = SourceLocation::new(1, 1, 2, 5, "other.llm");
        
        let merged1 = location1.merge(&location2);
        let merged2 = location1.merge(&location3);
        
        assert_eq!(merged1.start_line, 1);
        assert_eq!(merged1.start_column, 1);
        assert_eq!(merged1.end_line, 3);
        assert_eq!(merged1.end_column, 7);
        assert_eq!(merged1.file, "test.llm");
        
        assert_eq!(merged2, location1);
    }
    
    #[test]
    fn test_source_location_display() {
        let location = SourceLocation::new(1, 1, 2, 5, "test.llm");
        
        assert_eq!(format!("{}", location), "test.llm:1:1-2:5");
    }
    
    #[test]
    fn test_span_new() {
        let span = Span::new(0, 5, "test.llm");
        
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 5);
        assert_eq!(span.file, "test.llm");
    }
    
    #[test]
    fn test_span_to_location() {
        let source = "hello\nworld";
        let span = Span::new(0, 5, "test.llm");
        
        let location = span.to_location(source);
        
        assert_eq!(location.start_line, 1);
        assert_eq!(location.start_column, 1);
        assert_eq!(location.end_line, 1);
        assert_eq!(location.end_column, 6);
        assert_eq!(location.file, "test.llm");
    }
    
    #[test]
    fn test_source_code_new() {
        let source_code = SourceCode::new("hello\nworld", "test.llm");
        
        assert_eq!(source_code.source, "hello\nworld");
        assert_eq!(source_code.file, "test.llm");
    }
    
    #[test]
    fn test_source_code_get_line() {
        let source_code = SourceCode::new("hello\nworld", "test.llm");
        
        assert_eq!(source_code.get_line(1), Some("hello"));
        assert_eq!(source_code.get_line(2), Some("world"));
        assert_eq!(source_code.get_line(3), None);
    }
    
    #[test]
    fn test_source_code_get_span() {
        let source_code = SourceCode::new("hello\nworld", "test.llm");
        
        let span = source_code.get_span(0, 5);
        
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 5);
        assert_eq!(span.file, "test.llm");
    }
    
    #[test]
    fn test_source_code_get_location() {
        let source_code = SourceCode::new("hello\nworld", "test.llm");
        
        let span = source_code.get_span(0, 5);
        let location = source_code.get_location(&span);
        
        assert_eq!(location.start_line, 1);
        assert_eq!(location.start_column, 1);
        assert_eq!(location.end_line, 1);
        assert_eq!(location.end_column, 6);
        assert_eq!(location.file, "test.llm");
    }
    
    #[test]
    fn test_source_code_get_location_from_range() {
        let source_code = SourceCode::new("hello\nworld", "test.llm");
        
        let location = source_code.get_location_from_range(0, 5);
        
        assert_eq!(location.start_line, 1);
        assert_eq!(location.start_column, 1);
        assert_eq!(location.end_line, 1);
        assert_eq!(location.end_column, 6);
        assert_eq!(location.file, "test.llm");
    }
}
