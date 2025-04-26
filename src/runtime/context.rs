//! Context module for the LLM.lang runtime
//!
//! This module provides the context manager for the LLM.lang runtime,
//! which manages execution contexts and variable scopes.

use std::collections::HashMap;
use crate::Value;
use crate::parser::ast::Node;
use super::error::RuntimeError;

/// A variable scope
#[derive(Debug, Clone)]
struct Scope {
    /// The variables in this scope
    variables: HashMap<String, Value>,
}

impl Scope {
    /// Create a new scope
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    
    /// Register a variable in this scope
    fn register_variable(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }
    
    /// Get a variable from this scope
    fn get_variable(&self, name: &str) -> Option<Value> {
        self.variables.get(name).cloned()
    }
    
    /// Assign a value to a variable in this scope
    fn assign_variable(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(RuntimeError::new(
                &format!("Cannot assign to undefined variable: '{}'", name),
                crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
}

/// An execution context
#[derive(Debug, Clone)]
struct ExecutionContext {
    /// The context name
    name: String,
    
    /// The functions in this context
    functions: HashMap<String, Node>,
    
    /// The variables in this context
    variables: HashMap<String, Value>,
}

impl ExecutionContext {
    /// Create a new execution context
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }
    
    /// Register a function in this context
    fn register_function(&mut self, name: &str, node: &Node) {
        self.functions.insert(name.to_string(), node.clone());
    }
    
    /// Get a function from this context
    fn get_function(&self, name: &str) -> Option<Node> {
        self.functions.get(name).cloned()
    }
    
    /// Register a variable in this context
    fn register_variable(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }
    
    /// Get a variable from this context
    fn get_variable(&self, name: &str) -> Option<Value> {
        self.variables.get(name).cloned()
    }
    
    /// Assign a value to a variable in this context
    fn assign_variable(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(RuntimeError::new(
                &format!("Cannot assign to undefined variable: '{}'", name),
                crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
}

/// A context manager
pub struct Context {
    /// The execution contexts
    contexts: HashMap<String, ExecutionContext>,
    
    /// The current context
    current_context: String,
    
    /// The context stack
    context_stack: Vec<String>,
    
    /// The scope stack
    scope_stack: Vec<Scope>,
}

impl Context {
    /// Create a new context manager
    pub fn new() -> Self {
        // Create the global context
        let mut contexts = HashMap::new();
        contexts.insert("global".to_string(), ExecutionContext::new("global"));
        
        Self {
            contexts,
            current_context: "global".to_string(),
            context_stack: Vec::new(),
            scope_stack: Vec::new(),
        }
    }
    
    /// Create a new context
    pub fn create_context(&mut self, name: &str) {
        self.contexts.insert(name.to_string(), ExecutionContext::new(name));
    }
    
    /// Switch to a context
    pub fn switch_context(&mut self, name: &str) {
        // Push the current context onto the stack
        self.context_stack.push(self.current_context.clone());
        
        // Switch to the new context
        self.current_context = name.to_string();
    }
    
    /// Switch back to the previous context
    pub fn switch_back(&mut self) {
        if let Some(context) = self.context_stack.pop() {
            self.current_context = context;
        }
    }
    
    /// Push a new scope onto the stack
    pub fn push_frame(&mut self) {
        self.scope_stack.push(Scope::new());
    }
    
    /// Pop a scope from the stack
    pub fn pop_frame(&mut self) {
        self.scope_stack.pop();
    }
    
    /// Register a function in the current context
    pub fn register_function(&mut self, name: &str, node: &Node) {
        if let Some(context) = self.contexts.get_mut(&self.current_context) {
            context.register_function(name, node);
        }
    }
    
    /// Get a function from the current context
    pub fn get_function(&self, name: &str) -> Option<Node> {
        if let Some(context) = self.contexts.get(&self.current_context) {
            context.get_function(name)
        } else {
            None
        }
    }
    
    /// Register a variable in the current scope or context
    pub fn register_variable(&mut self, name: &str, value: Value) {
        // Check if we're in a function scope
        if let Some(scope) = self.scope_stack.last_mut() {
            // Register the variable in the current scope
            scope.register_variable(name, value);
        } else if let Some(context) = self.contexts.get_mut(&self.current_context) {
            // Register the variable in the current context
            context.register_variable(name, value);
        }
    }
    
    /// Get a variable from the current scope or context
    pub fn get_variable(&self, name: &str) -> Option<Value> {
        // Check the scope stack from top to bottom
        for scope in self.scope_stack.iter().rev() {
            if let Some(value) = scope.get_variable(name) {
                return Some(value);
            }
        }
        
        // Check the current context
        if let Some(context) = self.contexts.get(&self.current_context) {
            if let Some(value) = context.get_variable(name) {
                return Some(value);
            }
        }
        
        // Check the global context
        if self.current_context != "global" {
            if let Some(context) = self.contexts.get("global") {
                if let Some(value) = context.get_variable(name) {
                    return Some(value);
                }
            }
        }
        
        None
    }
    
    /// Assign a value to a variable in the current scope or context
    pub fn assign_variable(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        // Check the scope stack from top to bottom
        for scope in self.scope_stack.iter_mut().rev() {
            if scope.variables.contains_key(name) {
                scope.assign_variable(name, value)?;
                return Ok(());
            }
        }
        
        // Check the current context
        if let Some(context) = self.contexts.get_mut(&self.current_context) {
            if context.variables.contains_key(name) {
                context.assign_variable(name, value)?;
                return Ok(());
            }
        }
        
        // Check the global context
        if self.current_context != "global" {
            if let Some(context) = self.contexts.get_mut("global") {
                if context.variables.contains_key(name) {
                    context.assign_variable(name, value)?;
                    return Ok(());
                }
            }
        }
        
        Err(RuntimeError::new(
            &format!("Cannot assign to undefined variable: '{}'", name),
            crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::SourceLocation;
    use crate::parser::ast::{Node, NodeKind};
    
    #[test]
    fn test_context_new() {
        let context = Context::new();
        
        assert_eq!(context.contexts.len(), 1);
        assert_eq!(context.current_context, "global");
        assert_eq!(context.context_stack.len(), 0);
        assert_eq!(context.scope_stack.len(), 0);
    }
    
    #[test]
    fn test_context_create_context() {
        let mut context = Context::new();
        
        context.create_context("test");
        
        assert_eq!(context.contexts.len(), 2);
        assert!(context.contexts.contains_key("test"));
    }
    
    #[test]
    fn test_context_switch_context() {
        let mut context = Context::new();
        
        context.create_context("test");
        context.switch_context("test");
        
        assert_eq!(context.current_context, "test");
        assert_eq!(context.context_stack.len(), 1);
        assert_eq!(context.context_stack[0], "global");
    }
    
    #[test]
    fn test_context_switch_back() {
        let mut context = Context::new();
        
        context.create_context("test");
        context.switch_context("test");
        context.switch_back();
        
        assert_eq!(context.current_context, "global");
        assert_eq!(context.context_stack.len(), 0);
    }
    
    #[test]
    fn test_context_push_frame() {
        let mut context = Context::new();
        
        context.push_frame();
        
        assert_eq!(context.scope_stack.len(), 1);
    }
    
    #[test]
    fn test_context_pop_frame() {
        let mut context = Context::new();
        
        context.push_frame();
        context.pop_frame();
        
        assert_eq!(context.scope_stack.len(), 0);
    }
    
    #[test]
    fn test_context_register_function() {
        let mut context = Context::new();
        
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let node = Node {
            kind: NodeKind::Function,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        context.register_function("test", &node);
        
        assert!(context.get_function("test").is_some());
    }
    
    #[test]
    fn test_context_register_variable() {
        let mut context = Context::new();
        
        context.register_variable("test", Value::Int(42));
        
        assert_eq!(context.get_variable("test"), Some(Value::Int(42)));
    }
    
    #[test]
    fn test_context_register_variable_in_scope() {
        let mut context = Context::new();
        
        context.push_frame();
        context.register_variable("test", Value::Int(42));
        
        assert_eq!(context.get_variable("test"), Some(Value::Int(42)));
    }
    
    #[test]
    fn test_context_assign_variable() {
        let mut context = Context::new();
        
        context.register_variable("test", Value::Int(42));
        context.assign_variable("test", Value::Int(43)).unwrap();
        
        assert_eq!(context.get_variable("test"), Some(Value::Int(43)));
    }
    
    #[test]
    fn test_context_assign_variable_in_scope() {
        let mut context = Context::new();
        
        context.push_frame();
        context.register_variable("test", Value::Int(42));
        context.assign_variable("test", Value::Int(43)).unwrap();
        
        assert_eq!(context.get_variable("test"), Some(Value::Int(43)));
    }
    
    #[test]
    fn test_context_assign_variable_undefined() {
        let mut context = Context::new();
        
        let result = context.assign_variable("test", Value::Int(42));
        
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message,
            "Cannot assign to undefined variable: 'test'"
        );
    }
    
    #[test]
    fn test_context_variable_shadowing() {
        let mut context = Context::new();
        
        context.register_variable("test", Value::Int(42));
        context.push_frame();
        context.register_variable("test", Value::Int(43));
        
        assert_eq!(context.get_variable("test"), Some(Value::Int(43)));
        
        context.pop_frame();
        
        assert_eq!(context.get_variable("test"), Some(Value::Int(42)));
    }
}
