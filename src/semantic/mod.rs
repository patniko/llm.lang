//! Semantic analysis module for LLM.lang
//!
//! This module provides the semantic analyzer for the LLM.lang programming language,
//! which performs type checking and other semantic analyses on the AST.

pub mod analyzer;
pub mod error;

// Re-export commonly used types and functions
pub use self::analyzer::SemanticAnalyzer;
pub use self::error::{SemanticError, SemanticResult};

/// The semantic analyzer for the LLM.lang language
pub struct Semantic {
    /// The analyzer
    analyzer: SemanticAnalyzer,
}

impl Semantic {
    /// Create a new semantic analyzer
    pub fn new() -> Self {
        Self {
            analyzer: SemanticAnalyzer::new(),
        }
    }
    
    /// Analyze an AST
    pub fn analyze(&mut self, ast: crate::parser::ast::Ast) -> SemanticResult<crate::parser::ast::Ast> {
        self.analyzer.analyze(ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Ast, Node, NodeKind};
    use crate::utils::SourceLocation;
    
    #[test]
    fn test_semantic_new() {
        let semantic = Semantic::new();
        
        // Just check that we can create a semantic analyzer
        assert!(true);
    }
}
