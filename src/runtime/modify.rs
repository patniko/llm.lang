//! Modify module for the LLM.lang runtime
//!
//! This module provides the self-modifying capabilities for the LLM.lang runtime,
//! which allows programs to modify their own code at runtime.

use std::collections::HashMap;
use crate::parser::ast::{Ast, Node, NodeKind};
use crate::lexer::token::{Token, TokenKind};
use crate::Value;
use super::error::RuntimeError;
use crate::utils::SourceLocation;

/// A code modifier
pub struct Modify {
    /// The AST cache
    ast_cache: HashMap<String, Ast>,
    
    /// The source code cache
    source_cache: HashMap<String, String>,
}

impl Modify {
    /// Create a new code modifier
    pub fn new() -> Self {
        Self {
            ast_cache: HashMap::new(),
            source_cache: HashMap::new(),
        }
    }
    
    /// Cache an AST
    pub fn cache_ast(&mut self, name: &str, ast: Ast) {
        self.ast_cache.insert(name.to_string(), ast);
    }
    
    /// Get a cached AST
    pub fn get_ast(&self, name: &str) -> Option<&Ast> {
        self.ast_cache.get(name)
    }
    
    /// Cache source code
    pub fn cache_source(&mut self, name: &str, source: &str) {
        self.source_cache.insert(name.to_string(), source.to_string());
    }
    
    /// Get cached source code
    pub fn get_source(&self, name: &str) -> Option<&String> {
        self.source_cache.get(name)
    }
    
    /// Modify an AST
    pub fn modify_ast(&mut self, ast: &mut Ast, modifications: &[Modification]) -> Result<(), RuntimeError> {
        for modification in modifications {
            match modification {
                Modification::ReplaceNode { path, new_node } => {
                    self.replace_node(ast, path, new_node)?;
                }
                Modification::InsertNode { path, new_node, position } => {
                    self.insert_node(ast, path, new_node, *position)?;
                }
                Modification::DeleteNode { path } => {
                    self.delete_node(ast, path)?;
                }
                Modification::ModifyAttribute { path, name, value } => {
                    self.modify_attribute(ast, path, name, value)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Replace a node in an AST
    fn replace_node(&self, ast: &mut Ast, path: &[usize], new_node: &Node) -> Result<(), RuntimeError> {
        let node = self.get_node_mut(ast, path)?;
        *node = new_node.clone();
        Ok(())
    }
    
    /// Insert a node in an AST
    fn insert_node(&self, ast: &mut Ast, path: &[usize], new_node: &Node, position: usize) -> Result<(), RuntimeError> {
        let node = self.get_node_mut(ast, path)?;
        
        if position > node.children.len() {
            return Err(RuntimeError::new(
                &format!("Invalid position: {}", position),
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        node.children.insert(position, Box::new(new_node.clone()));
        Ok(())
    }
    
    /// Delete a node from an AST
    fn delete_node(&self, ast: &mut Ast, path: &[usize]) -> Result<(), RuntimeError> {
        if path.is_empty() {
            return Err(RuntimeError::new(
                "Cannot delete the root node",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let parent_path = &path[0..path.len() - 1];
        let index = path[path.len() - 1];
        
        let parent = self.get_node_mut(ast, parent_path)?;
        
        if index >= parent.children.len() {
            return Err(RuntimeError::new(
                &format!("Invalid index: {}", index),
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        parent.children.remove(index);
        Ok(())
    }
    
    /// Modify an attribute of a node
    fn modify_attribute(&self, ast: &mut Ast, path: &[usize], name: &str, value: &str) -> Result<(), RuntimeError> {
        let node = self.get_node_mut(ast, path)?;
        node.attributes.insert(name.to_string(), value.to_string());
        Ok(())
    }
    
    /// Get a node from an AST
    fn get_node<'a>(&self, ast: &'a Ast, path: &[usize]) -> Result<&'a Node, RuntimeError> {
        let mut node = ast.root();
        
        for &index in path {
            if index >= node.children.len() {
                return Err(RuntimeError::new(
                    &format!("Invalid path: {:?}", path),
                    SourceLocation::new(0, 0, 0, 0, ""),
                ));
            }
            
            node = node.children[index].as_ref();
        }
        
        Ok(node)
    }
    
    /// Get a mutable node from an AST
    fn get_node_mut<'a>(&self, ast: &'a mut Ast, path: &[usize]) -> Result<&'a mut Node, RuntimeError> {
        let root = ast.root_mut();
        self.get_node_mut_recursive(root, path, 0)
    }
    
    /// Recursively get a mutable node from an AST
    fn get_node_mut_recursive<'a>(&self, node: &'a mut Node, path: &[usize], depth: usize) -> Result<&'a mut Node, RuntimeError> {
        if depth == path.len() {
            return Ok(node);
        }
        
        let index = path[depth];
        
        if index >= node.children.len() {
            return Err(RuntimeError::new(
                &format!("Invalid path: {:?}", path),
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        self.get_node_mut_recursive(node.children[index].as_mut(), path, depth + 1)
    }
    
    /// Generate source code from an AST
    pub fn generate_source(&self, ast: &Ast) -> Result<String, RuntimeError> {
        let mut source = String::new();
        self.generate_source_recursive(ast.root(), &mut source, 0)?;
        Ok(source)
    }
    
    /// Recursively generate source code from an AST
    fn generate_source_recursive(&self, node: &Node, source: &mut String, indent: usize) -> Result<(), RuntimeError> {
        let indent_str = "    ".repeat(indent);
        
        match node.kind {
            NodeKind::Program => {
                for child in &node.children {
                    self.generate_source_recursive(child, source, indent)?;
                    source.push_str("\n");
                }
            }
            NodeKind::Context => {
                let name = node.get_attribute("name").ok_or_else(|| {
                    RuntimeError::missing_attribute("name", node.location.clone())
                })?;
                
                source.push_str(&format!("{}context {} {{\n", indent_str, name));
                
                for child in &node.children {
                    self.generate_source_recursive(child, source, indent + 1)?;
                    source.push_str("\n");
                }
                
                source.push_str(&format!("{}}}", indent_str));
            }
            NodeKind::Function => {
                let name = node.get_attribute("name").ok_or_else(|| {
                    RuntimeError::missing_attribute("name", node.location.clone())
                })?;
                
                source.push_str(&format!("{}fn {}(", indent_str, name));
                
                // Add parameters
                let mut first = true;
                for child in &node.children {
                    if child.kind == NodeKind::Parameter {
                        if !first {
                            source.push_str(", ");
                        }
                        
                        let param_name = child.get_attribute("name").ok_or_else(|| {
                            RuntimeError::missing_attribute("name", child.location.clone())
                        })?;
                        
                        let param_type = child.get_attribute("type").map_or("Any", |v| v).to_string();
                        
                        source.push_str(&format!("{}: {}", param_name, param_type));
                        
                        first = false;
                    }
                }
                
                source.push_str(")");
                
                // Add return type
                if let Some(return_type) = node.get_attribute("return_type") {
                    source.push_str(&format!(" -> {}", return_type));
                }
                
                source.push_str(" {\n");
                
                // Add body
                for child in &node.children {
                    if child.kind == NodeKind::Block {
                        for body_child in &child.children {
                            self.generate_source_recursive(body_child, source, indent + 1)?;
                            source.push_str("\n");
                        }
                    }
                }
                
                source.push_str(&format!("{}}}", indent_str));
            }
            NodeKind::Variable => {
                let name = node.get_attribute("name").ok_or_else(|| {
                    RuntimeError::missing_attribute("name", node.location.clone())
                })?;
                
                source.push_str(&format!("{}var {}", indent_str, name));
                
                // Add type
                if let Some(typ) = node.get_attribute("type") {
                    source.push_str(&format!(": {}", typ));
                }
                
                // Add initializer
                if !node.children.is_empty() {
                    source.push_str(" = ");
                    self.generate_source_recursive(&node.children[0], source, 0)?;
                }
                
                source.push_str(";");
            }
            NodeKind::Statement => {
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, indent)?;
                    
                    // Add semicolon if needed
                    if node.children[0].kind != NodeKind::Block &&
                       node.children[0].kind != NodeKind::If &&
                       node.children[0].kind != NodeKind::For &&
                       node.children[0].kind != NodeKind::When {
                        source.push_str(";");
                    }
                }
            }
            NodeKind::Block => {
                source.push_str(&format!("{}{{", indent_str));
                
                if !node.children.is_empty() {
                    source.push_str("\n");
                    
                    for child in &node.children {
                        self.generate_source_recursive(child, source, indent + 1)?;
                        source.push_str("\n");
                    }
                    
                    source.push_str(&indent_str);
                }
                
                source.push_str("}");
            }
            NodeKind::If => {
                source.push_str(&format!("{}if (", indent_str));
                
                // Add condition
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, 0)?;
                }
                
                source.push_str(") ");
                
                // Add then branch
                if node.children.len() > 1 {
                    self.generate_source_recursive(&node.children[1], source, indent)?;
                }
                
                // Add else branch
                if node.children.len() > 2 {
                    source.push_str(&format!(" else "));
                    self.generate_source_recursive(&node.children[2], source, indent)?;
                }
            }
            NodeKind::When => {
                source.push_str(&format!("{}when (", indent_str));
                
                // Add expression
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, 0)?;
                }
                
                source.push_str(") {\n");
                
                // Add cases
                for i in 1..node.children.len() {
                    let child = &node.children[i];
                    
                    if child.kind == NodeKind::Case {
                        // Add case expression
                        if !child.children.is_empty() {
                            self.generate_source_recursive(&child.children[0], source, indent + 1)?;
                        }
                        
                        source.push_str(" => ");
                        
                        // Add case body
                        if child.children.len() > 1 {
                            self.generate_source_recursive(&child.children[1], source, indent + 1)?;
                        }
                        
                        source.push_str("\n");
                    } else if child.kind == NodeKind::Otherwise {
                        source.push_str(&format!("{}otherwise => ", indent_str));
                        
                        // Add otherwise body
                        if !child.children.is_empty() {
                            self.generate_source_recursive(&child.children[0], source, indent + 1)?;
                        }
                        
                        source.push_str("\n");
                    }
                }
                
                source.push_str(&format!("{}}}", indent_str));
            }
            NodeKind::For => {
                let variable = node.get_attribute("variable").ok_or_else(|| {
                    RuntimeError::missing_attribute("variable", node.location.clone())
                })?;
                
                source.push_str(&format!("{}for ({} in ", indent_str, variable));
                
                // Add collection
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, 0)?;
                }
                
                source.push_str(") ");
                
                // Add body
                if node.children.len() > 1 {
                    self.generate_source_recursive(&node.children[1], source, indent)?;
                }
            }
            NodeKind::Return => {
                source.push_str(&format!("{}return", indent_str));
                
                // Add value
                if !node.children.is_empty() {
                    source.push_str(" ");
                    self.generate_source_recursive(&node.children[0], source, 0)?;
                }
                
                source.push_str(";");
            }
            NodeKind::With => {
                let name = node.get_attribute("name").ok_or_else(|| {
                    RuntimeError::missing_attribute("name", node.location.clone())
                })?;
                
                source.push_str(&format!("{}with context \"{}\" ", indent_str, name));
                
                // Add body
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, indent)?;
                }
            }
            NodeKind::Within => {
                let name = node.get_attribute("name").ok_or_else(|| {
                    RuntimeError::missing_attribute("name", node.location.clone())
                })?;
                
                source.push_str(&format!("{}within \"{}\" ", indent_str, name));
                
                // Add body
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, indent)?;
                }
            }
            NodeKind::Intent => {
                source.push_str(&format!("{}intent: ", indent_str));
                
                // Add expression
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, 0)?;
                }
                
                source.push_str(";");
            }
            NodeKind::Parallel => {
                source.push_str(&format!("{}parallel {{\n", indent_str));
                
                // Add paths
                for child in &node.children {
                    if child.kind == NodeKind::Path {
                        let name = child.get_attribute("name").ok_or_else(|| {
                            RuntimeError::missing_attribute("name", child.location.clone())
                        })?;
                        
                        source.push_str(&format!("{}{}: ", indent_str, name));
                        
                        // Add path body
                        if !child.children.is_empty() {
                            self.generate_source_recursive(&child.children[0], source, indent + 1)?;
                        }
                        
                        source.push_str("\n");
                    }
                }
                
                // Add selection strategy
                let strategy = node.get_attribute("strategy").map_or("all", |v| v).to_string();
                source.push_str(&format!("{}}} select {};\n", indent_str, strategy));
            }
            NodeKind::Apply => {
                source.push_str(&format!("{}apply ", indent_str));
                
                // Add vector
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, 0)?;
                }
                
                source.push_str(" to ");
                
                // Add body
                if node.children.len() > 1 {
                    self.generate_source_recursive(&node.children[1], source, indent)?;
                }
            }
            NodeKind::Semantic => {
                let token = node.get_attribute("token").ok_or_else(|| {
                    RuntimeError::missing_attribute("token", node.location.clone())
                })?;
                
                if token == "@remember" {
                    let name = node.get_attribute("name").ok_or_else(|| {
                        RuntimeError::missing_attribute("name", node.location.clone())
                    })?;
                    
                    source.push_str(&format!("{}@remember {} = ", indent_str, name));
                    
                    // Add value
                    if !node.children.is_empty() {
                        self.generate_source_recursive(&node.children[0], source, 0)?;
                    }
                    
                    source.push_str(";");
                } else if token == "@recall" {
                    source.push_str(&format!("{}@recall", indent_str));
                    
                    // Add key
                    if let Some(key) = node.get_attribute("key") {
                        source.push_str(&format!("(\"{}\")", key));
                    }
                    
                    source.push_str(";");
                } else {
                    source.push_str(&format!("{}{}", indent_str, token));
                }
            }
            NodeKind::Assignment => {
                // Add target
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, indent)?;
                }
                
                source.push_str(" = ");
                
                // Add value
                if node.children.len() > 1 {
                    self.generate_source_recursive(&node.children[1], source, 0)?;
                }
            }
            NodeKind::Binary => {
                // Add left operand
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, 0)?;
                }
                
                // Add operator
                let operator = node.get_attribute("operator").ok_or_else(|| {
                    RuntimeError::missing_attribute("operator", node.location.clone())
                })?;
                
                source.push_str(&format!(" {} ", operator));
                
                // Add right operand
                if node.children.len() > 1 {
                    self.generate_source_recursive(&node.children[1], source, 0)?;
                }
            }
            NodeKind::Unary => {
                // Add operator
                let operator = node.get_attribute("operator").ok_or_else(|| {
                    RuntimeError::missing_attribute("operator", node.location.clone())
                })?;
                
                source.push_str(operator);
                
                // Add operand
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, 0)?;
                }
            }
            NodeKind::Literal => {
                // Add value
                let typ = node.get_attribute("type").ok_or_else(|| {
                    RuntimeError::missing_attribute("type", node.location.clone())
                })?;
                
                let value = node.get_attribute("value").ok_or_else(|| {
                    RuntimeError::missing_attribute("value", node.location.clone())
                })?;
                
                match typ.as_str() {
                    "Int" | "Float" | "Bool" => {
                        source.push_str(value);
                    }
                    "String" => {
                        source.push_str(&format!("\"{}\"", value));
                    }
                    "Null" => {
                        source.push_str("null");
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            &format!("Unknown literal type: {}", typ),
                            node.location.clone(),
                        ));
                    }
                }
            }
            NodeKind::Identifier => {
                // Add name
                let name = node.get_attribute("name").ok_or_else(|| {
                    RuntimeError::missing_attribute("name", node.location.clone())
                })?;
                
                source.push_str(name);
            }
            NodeKind::Call => {
                // Add callee
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, 0)?;
                }
                
                source.push_str("(");
                
                // Add arguments
                let mut first = true;
                for i in 1..node.children.len() {
                    if !first {
                        source.push_str(", ");
                    }
                    
                    self.generate_source_recursive(&node.children[i], source, 0)?;
                    
                    first = false;
                }
                
                source.push_str(")");
            }
            NodeKind::NaturalLanguage => {
                // Add value
                let value = node.get_attribute("value").ok_or_else(|| {
                    RuntimeError::missing_attribute("value", node.location.clone())
                })?;
                
                source.push_str(&format!("#\"{}\"#", value));
            }
            NodeKind::Vector => {
                // Add name
                let name = node.get_attribute("name").ok_or_else(|| {
                    RuntimeError::missing_attribute("name", node.location.clone())
                })?;
                
                source.push_str(&format!("vector {} = ", name));
                
                // Add value
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, 0)?;
                }
            }
            NodeKind::Grouping => {
                source.push_str("(");
                
                // Add expression
                if !node.children.is_empty() {
                    self.generate_source_recursive(&node.children[0], source, 0)?;
                }
                
                source.push_str(")");
            }
            _ => {
                return Err(RuntimeError::new(
                    &format!("Cannot generate source for node kind: {:?}", node.kind),
                    node.location.clone(),
                ));
            }
        }
        
        Ok(())
    }
    
    /// Parse source code into an AST
    pub fn parse_source(&self, source: &str) -> Result<Ast, RuntimeError> {
        // Create a lexer
        let mut lexer = crate::lexer::Lexer::new(source);
        
        // Tokenize the source code
        let tokens = lexer.tokenize().map_err(|e| {
            RuntimeError::new(
                &format!("Lexer error: {}", e.message),
                e.location,
            )
        })?;
        
        // Create a parser
        let mut parser = crate::parser::Parser::new(tokens);
        
        // Parse the tokens into an AST
        let ast = parser.parse().map_err(|e| {
            RuntimeError::new(
                &format!("Parser error: {}", e.message),
                e.location,
            )
        })?;
        
        Ok(ast)
    }
    
    /// Execute a self-modifying function
    pub fn execute_self_modifying(&mut self, function: &str, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        // Get the AST
        let ast = self.ast_cache.get(function).ok_or_else(|| {
            RuntimeError::new(
                &format!("Unknown function: {}", function),
                SourceLocation::new(0, 0, 0, 0, ""),
            )
        })?.clone();
        
        // Create a list of modifications
        let mut modifications = Vec::new();
        
        // TODO: Execute the function and collect modifications
        
        // Apply the modifications
        let mut modified_ast = ast.clone();
        self.modify_ast(&mut modified_ast, &modifications)?;
        
        // Update the AST cache
        self.ast_cache.insert(function.to_string(), modified_ast.clone());
        
        // Generate the new source code
        let source = self.generate_source(&modified_ast)?;
        
        // Update the source cache
        self.source_cache.insert(function.to_string(), source);
        
        Ok(Value::Void)
    }
}

/// A modification to an AST
#[derive(Debug, Clone)]
pub enum Modification {
    /// Replace a node
    ReplaceNode {
        /// The path to the node
        path: Vec<usize>,
        
        /// The new node
        new_node: Node,
    },
    
    /// Insert a node
    InsertNode {
        /// The path to the parent node
        path: Vec<usize>,
        
        /// The new node
        new_node: Node,
        
        /// The position to insert at
        position: usize,
    },
    
    /// Delete a node
    DeleteNode {
        /// The path to the node
        path: Vec<usize>,
    },
    
    /// Modify an attribute
    ModifyAttribute {
        /// The path to the node
        path: Vec<usize>,
        
        /// The attribute name
        name: String,
        
        /// The new attribute value
        value: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Ast, Node, NodeKind};
    use crate::utils::SourceLocation;
    
    #[test]
    fn test_modify_new() {
        let modify = Modify::new();
        
        assert_eq!(modify.ast_cache.len(), 0);
        assert_eq!(modify.source_cache.len(), 0);
    }
    
    #[test]
    fn test_modify_cache_ast() {
        let mut modify = Modify::new();
        
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let node = Node {
            kind: NodeKind::Program,
            location: location.clone(),
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        let ast = Ast::new(node);
        
        modify.cache_ast("test", ast);
        
        assert_eq!(modify.ast_cache.len(), 1);
        assert!(modify.get_ast("test").is_some());
    }
    
    #[test]
    fn test_modify_cache_source() {
        let mut modify = Modify::new();
        
        modify.cache_source("test", "context Test {}");
        
        assert_eq!(modify.source_cache.len(), 1);
        assert!(modify.get_source("test").is_some());
    }
    
    #[test]
    fn test_modify_get_node() {
        let modify = Modify::new();
        
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let mut node = Node {
            kind: NodeKind::Program,
            location: location.clone(),
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        let child = Node {
            kind: NodeKind::Context,
            location: location.clone(),
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        node.children.push(Box::new(child));
        
        let ast = Ast::new(node);
        
        let result = modify.get_node(&ast, &[0]);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().kind, NodeKind::Context);
    }
    
    #[test]
    fn test_modify_get_node_invalid_path() {
        let modify = Modify::new();
        
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let node = Node {
            kind: NodeKind::Program,
            location: location.clone(),
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        let ast = Ast::new(node);
        
        let result = modify.get_node(&ast, &[0]);
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_modify_replace_node() {
        let modify = Modify::new();
        
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let mut node = Node {
            kind: NodeKind::Program,
            location: location.clone(),
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        let child = Node {
            kind: NodeKind::Context,
            location: location.clone(),
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        node.children.push(Box::new(child));
        
        let mut ast = Ast::new(node);
        
        let new_node = Node {
            kind: NodeKind::Function,
            location: location.clone(),
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        let result = modify.replace_node(&mut ast, &[0], &new_node);
        
        assert!(result.is_ok());
        assert_eq!(ast.root().children[0].kind, NodeKind::Function);
    }
}
