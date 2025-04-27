//! LLM.lang - A programming language optimized for Large Language Models
//!
//! This library provides the core functionality for the LLM.lang programming language,
//! including lexical analysis, parsing, semantic analysis, and runtime execution.

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod runtime;
pub mod stdlib;
pub mod utils;

/// Re-export commonly used types and functions
pub use crate::lexer::token::Token;
pub use crate::lexer::Lexer;
pub use crate::parser::ast::Ast;
pub use crate::parser::Parser;
pub use crate::semantic::analyzer::SemanticAnalyzer;
pub use crate::runtime::engine::Engine;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Compile LLM.lang source code to an executable
///
/// # Arguments
///
/// * `source` - The LLM.lang source code
/// * `options` - Compilation options
///
/// # Returns
///
/// A result containing the compiled program or an error
pub fn compile(source: &str, options: CompileOptions) -> Result<CompiledProgram, CompileError> {
    // Create a lexer
    let mut lexer = Lexer::new(source);
    
    // Tokenize the source code
    let tokens = lexer.tokenize()?;
    
    // Create a parser
    let mut parser = Parser::new(tokens);
    
    // Parse the tokens into an AST
    let ast = parser.parse()?;
    
    // Create a semantic analyzer
    let mut analyzer = SemanticAnalyzer::new();
    
    // Analyze the AST
    let analyzed_ast = analyzer.analyze(ast)?;
    
    // Compile the analyzed AST
    let program = CompiledProgram::new(analyzed_ast);
    
    Ok(program)
}

/// Execute LLM.lang source code
///
/// # Arguments
///
/// * `source` - The LLM.lang source code
/// * `options` - Execution options
///
/// # Returns
///
/// A result containing the execution result or an error
pub fn execute(source: &str, options: ExecuteOptions) -> Result<ExecutionResult, ExecutionError> {
    // Create a lexer
    let mut lexer = Lexer::new(source);
    
    // Tokenize the source code
    let tokens = lexer.tokenize().map_err(|e| ExecutionError::LexerError(e))?;
    
    // Create a parser
    let mut parser = Parser::new(tokens);
    
    // Parse the tokens into an AST
    let ast = parser.parse().map_err(|e| ExecutionError::ParserError(e))?;
    
    // Create a semantic analyzer
    let mut analyzer = SemanticAnalyzer::new();
    
    // Analyze the AST
    let analyzed_ast = analyzer.analyze(ast).map_err(|e| ExecutionError::SemanticError(e))?;
    
    // Create a runtime engine
    let mut engine = Engine::new(options.into());
    
    // Execute the analyzed AST
    let value = engine.execute(analyzed_ast)?;
    
    // Create the execution result
    let stats = ExecutionStats {
        execution_time: 0, // TODO: Implement execution time tracking
        peak_memory: 0,    // TODO: Implement memory usage tracking
        instructions: 0,   // TODO: Implement instruction counting
    };
    
    let result = ExecutionResult {
        value,
        stats,
    };
    
    Ok(result)
}

/// Options for compiling LLM.lang source code
#[derive(Debug, Clone)]
pub struct CompileOptions {
    /// Whether to optimize the compiled code
    pub optimize: bool,
    
    /// The optimization level (0-3)
    pub optimization_level: u8,
    
    /// Whether to include debug information
    pub debug_info: bool,
    
    /// The target platform
    pub target: String,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            optimize: true,
            optimization_level: 2,
            debug_info: false,
            target: String::from("native"),
        }
    }
}

/// Options for executing LLM.lang source code
#[derive(Debug, Clone)]
pub struct ExecuteOptions {
    /// Whether to enable debug mode
    pub debug: bool,
    
    /// The maximum memory usage in bytes
    pub max_memory: Option<usize>,
    
    /// The maximum execution time in milliseconds
    pub max_time: Option<u64>,
    
    /// Whether to enable parallel execution
    pub parallel: bool,
    
    /// Whether to enable vector operations
    pub vectors: bool,
    
    /// Whether to enable natural language processing
    pub nlp: bool,
}

impl Default for ExecuteOptions {
    fn default() -> Self {
        Self {
            debug: false,
            max_memory: None,
            max_time: None,
            parallel: true,
            vectors: true,
            nlp: true,
        }
    }
}

/// A compiled LLM.lang program
#[derive(Debug)]
pub struct CompiledProgram {
    /// The analyzed AST
    ast: Ast,
    
    /// Compilation metadata
    metadata: std::collections::HashMap<String, String>,
}

impl CompiledProgram {
    /// Create a new compiled program
    fn new(ast: Ast) -> Self {
        Self {
            ast,
            metadata: std::collections::HashMap::new(),
        }
    }
    
    /// Get the analyzed AST
    pub fn ast(&self) -> &Ast {
        &self.ast
    }
    
    /// Get the compilation metadata
    pub fn metadata(&self) -> &std::collections::HashMap<String, String> {
        &self.metadata
    }
}

/// The result of executing an LLM.lang program
#[derive(Debug)]
pub struct ExecutionResult {
    /// The return value of the program
    pub value: Value,
    
    /// Execution statistics
    pub stats: ExecutionStats,
}

/// Execution statistics
#[derive(Debug)]
pub struct ExecutionStats {
    /// The execution time in milliseconds
    pub execution_time: u64,
    
    /// The peak memory usage in bytes
    pub peak_memory: usize,
    
    /// The number of instructions executed
    pub instructions: u64,
}

/// A value in the LLM.lang runtime
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// No value
    Void,
    
    /// A boolean value
    Bool(bool),
    
    /// An integer value
    Int(i64),
    
    /// A floating-point value
    Float(f64),
    
    /// A string value
    String(String),
    
    /// A list value
    List(Vec<Value>),
    
    /// A map value
    Map(std::collections::HashMap<String, Value>),
    
    /// A function value
    Function(String),
    
    /// A vector value
    Vector(Vec<f64>),
    
    /// A context value
    Context(String),
}

/// Errors that can occur during compilation
#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    /// An error occurred during lexical analysis
    #[error("Lexer error: {0}")]
    LexerError(#[from] lexer::error::LexerError),
    
    /// An error occurred during parsing
    #[error("Parser error: {0}")]
    ParserError(#[from] parser::error::ParserError),
    
    /// An error occurred during semantic analysis
    #[error("Semantic error: {0}")]
    SemanticError(#[from] semantic::error::SemanticError),
}

/// Errors that can occur during execution
#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    /// An error occurred during lexical analysis
    #[error("Lexer error: {0}")]
    LexerError(lexer::error::LexerError),
    
    /// An error occurred during parsing
    #[error("Parser error: {0}")]
    ParserError(parser::error::ParserError),
    
    /// An error occurred during semantic analysis
    #[error("Semantic error: {0}")]
    SemanticError(semantic::error::SemanticError),
    
    /// An error occurred during runtime execution
    #[error("Runtime error: {0}")]
    RuntimeError(#[from] runtime::error::RuntimeError),
    
    /// The program exceeded the maximum allowed memory
    #[error("Memory limit exceeded: used {used} bytes, limit {limit} bytes")]
    MemoryLimitExceeded {
        /// The amount of memory used
        used: usize,
        
        /// The memory limit
        limit: usize,
    },
    
    /// The program exceeded the maximum allowed execution time
    #[error("Time limit exceeded: executed for {executed} ms, limit {limit} ms")]
    TimeLimitExceeded {
        /// The execution time
        executed: u64,
        
        /// The time limit
        limit: u64,
    },
}

// Implement conversion from ExecuteOptions to runtime::engine::EngineOptions
impl From<ExecuteOptions> for runtime::engine::EngineOptions {
    fn from(options: ExecuteOptions) -> Self {
        Self {
            debug: options.debug,
            max_memory: options.max_memory,
            max_time: options.max_time,
            parallel: options.parallel,
            vectors: options.vectors,
            nlp: options.nlp,
        }
    }
}

// The actual module implementations are in their respective files
