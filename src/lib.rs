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
    let result = engine.execute(analyzed_ast)?;
    
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

// Create placeholder module files to satisfy the compiler
// These would be replaced with actual implementations

/// Lexical analysis module
pub mod lexer {
    /// Token definitions
    pub mod token {
        /// A token in the LLM.lang language
        #[derive(Debug, Clone, PartialEq)]
        pub struct Token {
            /// The token type
            pub kind: TokenKind,
            
            /// The token value
            pub value: String,
            
            /// The token location
            pub location: crate::utils::SourceLocation,
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
    }
    
    /// Lexer error definitions
    pub mod error {
        /// An error that can occur during lexical analysis
        #[derive(Debug, Clone)]
        pub struct LexerError {
            /// The error message
            pub message: String,
            
            /// The error location
            pub location: crate::utils::SourceLocation,
        }
        
        impl LexerError {
            /// Create a new lexer error
            pub fn new(message: &str, location: crate::utils::SourceLocation) -> Self {
                Self {
                    message: message.to_string(),
                    location,
                }
            }
            
            /// Create a new "unexpected character" error
            pub fn unexpected_character(c: char, location: crate::utils::SourceLocation) -> Self {
                Self::new(&format!("Unexpected character: '{}'", c), location)
            }
            
            /// Create a new "unexpected end of input" error
            pub fn unexpected_end_of_input(location: crate::utils::SourceLocation) -> Self {
                Self::new("Unexpected end of input", location)
            }
            
            /// Create a new "unterminated string" error
            pub fn unterminated_string(location: crate::utils::SourceLocation) -> Self {
                Self::new("Unterminated string literal", location)
            }
        }
        
        impl std::error::Error for LexerError {}
        
        impl std::fmt::Display for LexerError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    }
    
    /// A lexer for the LLM.lang language
    pub struct Lexer {
        /// The source code
        source: String,
        
        /// The current position in the source code
        position: usize,
    }
    
    impl Lexer {
        /// Create a new lexer
        pub fn new(source: &str) -> Self {
            Self {
                source: source.to_string(),
                position: 0,
            }
        }
        
        /// Tokenize the source code
        pub fn tokenize(&mut self) -> Result<Vec<crate::lexer::token::Token>, crate::lexer::error::LexerError> {
            // Placeholder implementation
            Ok(vec![])
        }
    }
}

/// Parsing module
pub mod parser {
    /// AST definitions
    pub mod ast {
        /// An abstract syntax tree
        #[derive(Debug)]
        pub struct Ast {
            /// The root node
            pub root: Box<Node>,
        }
        
        /// An AST node
        #[derive(Debug)]
        pub struct Node {
            /// The node type
            pub kind: NodeKind,
            
            /// The node location
            pub location: crate::utils::SourceLocation,
            
            /// The node children
            pub children: Vec<Box<Node>>,
            
            /// The node attributes
            pub attributes: std::collections::HashMap<String, String>,
        }
        
        /// Node types
        #[derive(Debug, Clone, PartialEq)]
        pub enum NodeKind {
            /// A program
            Program,
            
            /// A context declaration
            Context,
            
            /// A function declaration
            Function,
            
            /// A parameter declaration
            Parameter,
            
            /// A variable declaration
            Variable,
            
            /// A statement
            Statement,
            
            /// A block
            Block,
            
            /// An if statement
            If,
            
            /// A when statement
            When,
            
            /// A case in a when statement
            Case,
            
            /// An otherwise case in a when statement
            Otherwise,
            
            /// A for statement
            For,
            
            /// A return statement
            Return,
            
            /// A with statement
            With,
            
            /// A within statement
            Within,
            
            /// An intent statement
            Intent,
            
            /// A parallel statement
            Parallel,
            
            /// A path in a parallel statement
            Path,
            
            /// An apply statement
            Apply,
            
            /// A semantic statement
            Semantic,
            
            /// An assignment expression
            Assignment,
            
            /// A binary expression
            Binary,
            
            /// A unary expression
            Unary,
            
            /// A literal expression
            Literal,
            
            /// An identifier expression
            Identifier,
            
            /// A function call expression
            Call,
            
            /// A natural language expression
            NaturalLanguage,
            
            /// A vector expression
            Vector,
            
            /// A grouping expression
            Grouping,
        }
    }
    
    /// Parser error definitions
    pub mod error {
        /// An error that can occur during parsing
        #[derive(Debug, Clone)]
        pub struct ParserError {
            /// The error message
            pub message: String,
            
            /// The error location
            pub location: crate::utils::SourceLocation,
        }
        
        impl ParserError {
            /// Create a new parser error
            pub fn new(message: &str, location: crate::utils::SourceLocation) -> Self {
                Self {
                    message: message.to_string(),
                    location,
                }
            }
            
            /// Create a new "unexpected token" error
            pub fn unexpected_token(token: &str, expected: &str, location: crate::utils::SourceLocation) -> Self {
                Self::new(
                    &format!("Unexpected token: '{}', expected {}", token, expected),
                    location,
                )
            }
            
            /// Create a new "unexpected end of input" error
            pub fn unexpected_end_of_input(expected: &str, location: crate::utils::SourceLocation) -> Self {
                Self::new(
                    &format!("Unexpected end of input, expected {}", expected),
                    location,
                )
            }
        }
        
        impl std::error::Error for ParserError {}
        
        impl std::fmt::Display for ParserError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    }
    
    /// A parser for the LLM.lang language
    pub struct Parser {
        /// The tokens to parse
        tokens: Vec<crate::lexer::token::Token>,
        
        /// The current position in the token stream
        position: usize,
    }
    
    impl Parser {
        /// Create a new parser
        pub fn new(tokens: Vec<crate::lexer::token::Token>) -> Self {
            Self {
                tokens,
                position: 0,
            }
        }
        
        /// Parse the tokens into an AST
        pub fn parse(&mut self) -> Result<crate::parser::ast::Ast, crate::parser::error::ParserError> {
            // Placeholder implementation
            Ok(crate::parser::ast::Ast {
                root: Box::new(crate::parser::ast::Node {
                    kind: crate::parser::ast::NodeKind::Program,
                    location: crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
                    children: vec![],
                    attributes: std::collections::HashMap::new(),
                }),
            })
        }
    }
}

/// Semantic analysis module
pub mod semantic {
    /// Semantic analyzer
    pub mod analyzer {
        /// A semantic analyzer for the LLM.lang language
        pub struct SemanticAnalyzer {
            // Implementation details
        }
        
        impl SemanticAnalyzer {
            /// Create a new semantic analyzer
            pub fn new() -> Self {
                Self {}
            }
            
            /// Analyze an AST
            pub fn analyze(&mut self, ast: crate::parser::ast::Ast) -> Result<crate::parser::ast::Ast, crate::semantic::error::SemanticError> {
                // Placeholder implementation
                Ok(ast)
            }
        }
    }
    
    /// Semantic error definitions
    pub mod error {
        /// An error that can occur during semantic analysis
        #[derive(Debug, Clone)]
        pub struct SemanticError {
            /// The error message
            pub message: String,
            
            /// The error location
            pub location: crate::utils::SourceLocation,
        }
        
        impl SemanticError {
            /// Create a new semantic error
            pub fn new(message: &str, location: crate::utils::SourceLocation) -> Self {
                Self {
                    message: message.to_string(),
                    location,
                }
            }
            
            /// Create a new "undefined variable" error
            pub fn undefined_variable(name: &str, location: crate::utils::SourceLocation) -> Self {
                Self::new(&format!("Undefined variable: '{}'", name), location)
            }
            
            /// Create a new "undefined function" error
            pub fn undefined_function(name: &str, location: crate::utils::SourceLocation) -> Self {
                Self::new(&format!("Undefined function: '{}'", name), location)
            }
            
            /// Create a new "invalid type" error
            pub fn invalid_type(expected: &str, actual: &str, location: crate::utils::SourceLocation) -> Self {
                Self::new(
                    &format!("Invalid type: expected {}, got {}", expected, actual),
                    location,
                )
            }
        }
        
        impl std::error::Error for SemanticError {}
        
        impl std::fmt::Display for SemanticError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    }
}

/// Runtime module
pub mod runtime {
    /// Runtime engine
    pub mod engine {
        /// Options for the runtime engine
        #[derive(Debug, Clone)]
        pub struct EngineOptions {
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
        
        /// A runtime engine for the LLM.lang language
        pub struct Engine {
            /// The engine options
            options: EngineOptions,
        }
        
        impl Engine {
            /// Create a new runtime engine
            pub fn new(options: EngineOptions) -> Self {
                Self { options }
            }
            
            /// Execute an AST
            pub fn execute(&mut self, ast: crate::parser::ast::Ast) -> Result<crate::ExecutionResult, crate::runtime::error::RuntimeError> {
                // Placeholder implementation
                Ok(crate::ExecutionResult {
                    value: crate::Value::Void,
                    stats: crate::ExecutionStats {
                        execution_time: 0,
                        peak_memory: 0,
                        instructions: 0,
                    },
                })
            }
        }
    }
    
    /// Runtime error definitions
    pub mod error {
        /// An error that can occur during runtime execution
        #[derive(Debug, Clone)]
        pub struct RuntimeError {
            /// The error message
            pub message: String,
            
            /// The error location
            pub location: crate::utils::SourceLocation,
        }
        
        impl RuntimeError {
            /// Create a new runtime error
            pub fn new(message: &str, location: crate::utils::SourceLocation) -> Self {
                Self {
                    message: message.to_string(),
                    location,
                }
            }
            
            /// Create a new "undefined variable" error
            pub fn undefined_variable(name: &str, location: crate::utils::SourceLocation) -> Self {
                Self::new(&format!("Undefined variable: '{}'", name), location)
            }
            
            /// Create a new "undefined function" error
            pub fn undefined_function(name: &str, location: crate::utils::SourceLocation) -> Self {
                Self::new(&format!("Undefined function: '{}'", name), location)
            }
            
            /// Create a new "invalid type" error
            pub fn invalid_type(expected: &str, actual: &str, location: crate::utils::SourceLocation) -> Self {
                Self::new(
                    &format!("Invalid type: expected {}, got {}", expected, actual),
                    location,
                )
            }
        }
        
        impl std::error::Error for RuntimeError {}
        
        impl std::fmt::Display for RuntimeError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    }
}

/// Standard library module
pub mod stdlib {
    // Standard library implementation
}

/// Utility module
pub mod utils {
    /// A source location
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
    }
}
