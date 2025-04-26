//! Runtime module for LLM.lang
//!
//! This module provides the runtime environment for the LLM.lang programming language,
//! which executes the abstract syntax tree (AST) produced by the parser.

pub mod engine;
pub mod error;
pub mod memory;
pub mod context;
pub mod vector;
pub mod parallel;
pub mod example;
pub mod nlp;
pub mod interop;

// Re-export commonly used types and functions
pub use self::engine::Engine;
pub use self::error::RuntimeError;
pub use self::memory::Memory;
pub use self::context::Context;
pub use self::vector::Vector;
pub use self::parallel::Parallel;
pub use self::example::Example;
pub use self::nlp::NLP;
pub use self::interop::Interop;

/// The runtime environment for the LLM.lang language
pub struct Runtime {
    /// The execution engine
    engine: Engine,
    
    /// The memory manager
    memory: Memory,
    
    /// The context manager
    context: Context,
    
    /// The vector engine
    vector: Vector,
    
    /// The parallel executor
    parallel: Parallel,
    
    /// The example executor
    example: Example,
    
    /// The natural language processor
    nlp: NLP,
    
    /// The interoperability manager
    interop: Interop,
}

impl Runtime {
    /// Create a new runtime environment
    pub fn new(options: engine::EngineOptions) -> Self {
        Self {
            engine: Engine::new(options),
            memory: Memory::new(),
            context: Context::new(),
            vector: Vector::new(),
            parallel: Parallel::new(),
            example: Example::new(),
            nlp: NLP::new(),
            interop: Interop::new(),
        }
    }
    
    /// Execute an AST
    pub fn execute(&mut self, ast: crate::parser::ast::Ast) -> Result<crate::Value, error::RuntimeError> {
        self.engine.execute(ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Ast, Node, NodeKind};
    use crate::utils::SourceLocation;
    
    #[test]
    fn test_runtime_new() {
        let options = engine::EngineOptions {
            debug: false,
            max_memory: None,
            max_time: None,
            parallel: true,
            vectors: true,
            nlp: true,
        };
        
        let runtime = Runtime::new(options);
        
        // Just check that we can create a runtime
        assert!(true);
    }
}
