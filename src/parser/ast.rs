//! Abstract Syntax Tree (AST) definitions for the LLM.lang parser
//!
//! This module defines the node types and structures used by the parser.

use std::collections::HashMap;
use std::fmt;

use crate::utils::SourceLocation;

/// An abstract syntax tree
#[derive(Debug, Clone)]
pub struct Ast {
    /// The root node
    pub root: Box<Node>,
}

/// An AST node
#[derive(Debug, Clone)]
pub struct Node {
    /// The node type
    pub kind: NodeKind,
    
    /// The node location
    pub location: SourceLocation,
    
    /// The node children
    pub children: Vec<Box<Node>>,
    
    /// The node attributes
    pub attributes: HashMap<String, String>,
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
    
    /// An examples statement
    Examples,
    
    /// An example
    Example,
    
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

/// Expression types
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A literal expression
    Literal(Literal),
    
    /// An identifier expression
    Identifier(String),
    
    /// A function call expression
    Call {
        /// The function name
        callee: Box<Expression>,
        
        /// The arguments
        arguments: Vec<Expression>,
    },
    
    /// A binary expression
    Binary {
        /// The left-hand side
        left: Box<Expression>,
        
        /// The operator
        operator: String,
        
        /// The right-hand side
        right: Box<Expression>,
    },
    
    /// A unary expression
    Unary {
        /// The operator
        operator: String,
        
        /// The operand
        operand: Box<Expression>,
    },
    
    /// A natural language expression
    NaturalLanguage(String),
    
    /// A vector expression
    Vector {
        /// The vector name
        name: String,
        
        /// The vector value
        value: Box<Expression>,
    },
    
    /// A grouping expression
    Grouping(Box<Expression>),
}

/// Literal types
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// An integer literal
    Int(i64),
    
    /// A floating-point literal
    Float(f64),
    
    /// A string literal
    String(String),
    
    /// A boolean literal
    Bool(bool),
    
    /// A null literal
    Null,
}

/// Statement types
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// An expression statement
    Expression(Expression),
    
    /// A variable declaration
    Variable {
        /// The variable name
        name: String,
        
        /// The variable type (if any)
        typ: Option<Type>,
        
        /// The initializer
        initializer: Expression,
    },
    
    /// A block statement
    Block(Vec<Statement>),
    
    /// An if statement
    If {
        /// The condition
        condition: Expression,
        
        /// The then branch
        then_branch: Box<Statement>,
        
        /// The else branch (if any)
        else_branch: Option<Box<Statement>>,
    },
    
    /// A when statement
    When {
        /// The expression
        expression: Expression,
        
        /// The cases
        cases: Vec<(Expression, Statement)>,
        
        /// The otherwise case (if any)
        otherwise: Option<Box<Statement>>,
    },
    
    /// A for statement
    For {
        /// The loop variable
        variable: String,
        
        /// The collection expression
        collection: Expression,
        
        /// The loop body
        body: Box<Statement>,
    },
    
    /// A return statement
    Return(Option<Expression>),
    
    /// A with statement
    With {
        /// The context name
        name: String,
        
        /// The body
        body: Box<Statement>,
    },
    
    /// A within statement
    Within {
        /// The context name
        name: String,
        
        /// The body
        body: Box<Statement>,
    },
    
    /// An intent statement
    Intent(Expression),
    
    /// A parallel statement
    Parallel {
        /// The paths
        paths: Vec<(String, Statement)>,
        
        /// The selection strategy
        strategy: String,
    },
    
    /// An apply statement
    Apply {
        /// The vector expression
        vector: Expression,
        
        /// The body
        body: Box<Statement>,
    },
    
    /// A semantic statement
    Semantic {
        /// The token
        token: String,
        
        /// The name (if any)
        name: Option<String>,
        
        /// The value (if any)
        value: Option<Expression>,
        
        /// The key (if any)
        key: Option<String>,
    },
}

/// Type definitions
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// An integer type
    Int,
    
    /// A floating-point type
    Float,
    
    /// A string type
    String,
    
    /// A boolean type
    Bool,
    
    /// A list type
    List,
    
    /// A map type
    Map,
    
    /// A vector type
    Vector,
    
    /// A context type
    Context,
    
    /// A semantic type
    Semantic(String),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::String => write!(f, "String"),
            Type::Bool => write!(f, "Bool"),
            Type::List => write!(f, "List"),
            Type::Map => write!(f, "Map"),
            Type::Vector => write!(f, "Vector"),
            Type::Context => write!(f, "Context"),
            Type::Semantic(name) => write!(f, "{}", name),
        }
    }
}

impl Node {
    /// Create a new node
    pub fn new(kind: NodeKind, location: SourceLocation) -> Self {
        Self {
            kind,
            location,
            children: Vec::new(),
            attributes: HashMap::new(),
        }
    }
    
    /// Add a child node
    pub fn add_child(&mut self, child: Node) {
        self.children.push(Box::new(child));
    }
    
    /// Add an attribute
    pub fn add_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }
    
    /// Get an attribute
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }
    
    /// Check if the node has an attribute
    pub fn has_attribute(&self, key: &str) -> bool {
        self.attributes.contains_key(key)
    }
    
    /// Get the number of children
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
    
    /// Get a child node
    pub fn get_child(&self, index: usize) -> Option<&Node> {
        self.children.get(index).map(|child| child.as_ref())
    }
    
    /// Get a mutable child node
    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.children.get_mut(index).map(|child| child.as_mut())
    }
}

impl Ast {
    /// Create a new AST
    pub fn new(root: Node) -> Self {
        Self {
            root: Box::new(root),
        }
    }
    
    /// Get the root node
    pub fn root(&self) -> &Node {
        &self.root
    }
    
    /// Get a mutable reference to the root node
    pub fn root_mut(&mut self) -> &mut Node {
        &mut self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_node_new() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let node = Node::new(NodeKind::Program, location.clone());
        
        assert_eq!(node.kind, NodeKind::Program);
        assert_eq!(node.location, location);
        assert_eq!(node.children.len(), 0);
        assert_eq!(node.attributes.len(), 0);
    }
    
    #[test]
    fn test_node_add_child() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let mut node = Node::new(NodeKind::Program, location.clone());
        
        let child = Node::new(NodeKind::Function, location.clone());
        node.add_child(child);
        
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.get_child(0).unwrap().kind, NodeKind::Function);
    }
    
    #[test]
    fn test_node_add_attribute() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let mut node = Node::new(NodeKind::Function, location);
        
        node.add_attribute("name", "main");
        
        assert_eq!(node.attributes.len(), 1);
        assert_eq!(node.get_attribute("name").unwrap(), "main");
        assert!(node.has_attribute("name"));
        assert!(!node.has_attribute("return_type"));
    }
    
    #[test]
    fn test_ast_new() {
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let root = Node::new(NodeKind::Program, location);
        
        let ast = Ast::new(root);
        
        assert_eq!(ast.root().kind, NodeKind::Program);
    }
    
    #[test]
    fn test_type_display() {
        assert_eq!(format!("{}", Type::Int), "Int");
        assert_eq!(format!("{}", Type::Float), "Float");
        assert_eq!(format!("{}", Type::String), "String");
        assert_eq!(format!("{}", Type::Bool), "Bool");
        assert_eq!(format!("{}", Type::List), "List");
        assert_eq!(format!("{}", Type::Map), "Map");
        assert_eq!(format!("{}", Type::Vector), "Vector");
        assert_eq!(format!("{}", Type::Context), "Context");
        assert_eq!(format!("{}", Type::Semantic("~EmailAddress~".to_string())), "~EmailAddress~");
    }
}
