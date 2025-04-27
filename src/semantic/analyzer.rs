//! Semantic analyzer for the LLM.lang language
//!
//! This module provides the semantic analyzer for the LLM.lang language,
//! which performs type checking and other semantic analyses on the AST.

use std::collections::HashMap;
use crate::parser::ast::{Ast, Node, NodeKind};
use crate::utils::SourceLocation;
use super::error::{SemanticError, SemanticResult};

/// A symbol in the symbol table
#[derive(Debug, Clone)]
pub enum Symbol {
    /// A variable symbol
    Variable {
        /// The variable name
        name: String,
        
        /// The variable type
        typ: String,
        
        /// Whether the variable is mutable
        mutable: bool,
    },
    
    /// A function symbol
    Function {
        /// The function name
        name: String,
        
        /// The function parameters
        parameters: Vec<(String, String)>,
        
        /// The function return type
        return_type: String,
    },
    
    /// A context symbol
    Context {
        /// The context name
        name: String,
        
        /// The context properties
        properties: HashMap<String, String>,
    },
}

/// A scope in the symbol table
#[derive(Debug, Clone)]
pub struct Scope {
    /// The symbols in this scope
    symbols: HashMap<String, Symbol>,
    
    /// The parent scope
    parent: Option<Box<Scope>>,
}

impl Scope {
    /// Create a new scope
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            parent: None,
        }
    }
    
    /// Create a new scope with a parent
    pub fn with_parent(parent: Scope) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }
    
    /// Define a symbol in this scope
    pub fn define(&mut self, name: &str, symbol: Symbol) -> bool {
        if self.symbols.contains_key(name) {
            false
        } else {
            self.symbols.insert(name.to_string(), symbol);
            true
        }
    }
    
    /// Get a symbol from this scope or a parent scope
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        if let Some(symbol) = self.symbols.get(name) {
            Some(symbol)
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }
    
    /// Get a mutable reference to a symbol from this scope or a parent scope
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        if self.symbols.contains_key(name) {
            self.symbols.get_mut(name)
        } else if let Some(parent) = &mut self.parent {
            parent.get_mut(name)
        } else {
            None
        }
    }
    
    /// Check if a symbol is defined in this scope
    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }
    
    /// Check if a symbol is defined in this scope or a parent scope
    pub fn contains_recursive(&self, name: &str) -> bool {
        if self.symbols.contains_key(name) {
            true
        } else if let Some(parent) = &self.parent {
            parent.contains_recursive(name)
        } else {
            false
        }
    }
}

/// A semantic analyzer for the LLM.lang language
pub struct SemanticAnalyzer {
    /// The current scope
    scope: Scope,
    
    /// Whether we're in a function
    in_function: bool,
    
    /// Whether we're in a loop
    in_loop: bool,
    
    /// The current function return type
    current_return_type: Option<String>,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer
    pub fn new() -> Self {
        let mut analyzer = Self {
            scope: Scope::new(),
            in_function: false,
            in_loop: false,
            current_return_type: None,
        };
        
        // Register standard library functions
        analyzer.register_stdlib_functions();
        
        analyzer
    }
    
    /// Register standard library functions
    fn register_stdlib_functions(&mut self) {
        // Core functions
        self.register_stdlib_function("print", vec![("value", "Any")], "Void");
        self.register_stdlib_function("toString", vec![("value", "Any")], "String");
        self.register_stdlib_function("parseInt", vec![("value", "String")], "Int");
        self.register_stdlib_function("parseFloat", vec![("value", "String")], "Float");
        
        // Collection functions
        self.register_stdlib_function("length", vec![("collection", "Any")], "Int");
        self.register_stdlib_function("isEmpty", vec![("collection", "Any")], "Bool");
        self.register_stdlib_function("contains", vec![("collection", "Any"), ("value", "Any")], "Bool");
        
        // String functions
        self.register_stdlib_function("substring", vec![("string", "String"), ("start", "Int"), ("end", "Int")], "String");
        self.register_stdlib_function("indexOf", vec![("string", "String"), ("substring", "String")], "Int");
        self.register_stdlib_function("toLowerCase", vec![("string", "String")], "String");
        self.register_stdlib_function("toUpperCase", vec![("string", "String")], "String");
        self.register_stdlib_function("trim", vec![("string", "String")], "String");
        
        // Vector functions
        self.register_stdlib_function("embed", vec![("text", "String")], "Vector");
        self.register_stdlib_function("similarity", vec![("vector1", "Vector"), ("vector2", "Vector")], "Float");
        
        // Context functions
        self.register_stdlib_function("currentContext", vec![], "Context");
        self.register_stdlib_function("switchContext", vec![("name", "String")], "Context");
    }
    
    /// Register a standard library function
    fn register_stdlib_function(&mut self, name: &str, parameters: Vec<(&str, &str)>, return_type: &str) {
        let function = Symbol::Function {
            name: name.to_string(),
            parameters: parameters.iter().map(|(n, t)| (n.to_string(), t.to_string())).collect(),
            return_type: return_type.to_string(),
        };
        
        self.scope.define(name, function);
    }
    
    /// Analyze an AST
    pub fn analyze(&mut self, ast: Ast) -> SemanticResult<Ast> {
        // Create a new AST with the same root node
        let mut new_ast = Ast::new((*ast.root()).clone());
        
        // Analyze the root node
        self.analyze_node(new_ast.root_mut())?;
        
        Ok(new_ast)
    }
    
    /// Analyze a node
    fn analyze_node(&mut self, node: &mut Node) -> SemanticResult<()> {
        match node.kind {
            NodeKind::Program => self.analyze_program(node),
            NodeKind::Context => self.analyze_context(node),
            NodeKind::Function => self.analyze_function(node),
            NodeKind::Variable => self.analyze_variable(node),
            NodeKind::Statement => self.analyze_statement(node),
            NodeKind::Block => self.analyze_block(node),
            NodeKind::If => self.analyze_if(node),
            NodeKind::When => self.analyze_when(node),
            NodeKind::For => self.analyze_for(node),
            NodeKind::Return => self.analyze_return(node),
            NodeKind::With => self.analyze_with(node),
            NodeKind::Within => self.analyze_within(node),
            NodeKind::Intent => self.analyze_intent(node),
            NodeKind::Parallel => self.analyze_parallel(node),
            NodeKind::Apply => self.analyze_apply(node),
            NodeKind::Semantic => self.analyze_semantic(node),
            NodeKind::Assignment => self.analyze_assignment(node),
            NodeKind::Binary => self.analyze_binary(node),
            NodeKind::Unary => self.analyze_unary(node),
            NodeKind::Literal => self.analyze_literal(node),
            NodeKind::Identifier => self.analyze_identifier(node),
            NodeKind::Call => self.analyze_call(node),
            NodeKind::NaturalLanguage => self.analyze_natural_language(node),
            NodeKind::Vector => self.analyze_vector(node),
            NodeKind::Grouping => self.analyze_grouping(node),
            _ => Ok(()),
        }
    }
    
    /// Analyze a program node
    fn analyze_program(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Analyze all children
        for i in 0..node.child_count() {
            if let Some(child) = node.get_child_mut(i) {
                self.analyze_node(child)?;
            }
        }
        
        Ok(())
    }
    
    /// Analyze a context node
    fn analyze_context(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Get the context name
        let name = node.get_attribute("name").ok_or_else(|| {
            SemanticError::missing_attribute("name", node.location.clone())
        })?;
        
        // Check if the context is already defined
        if self.scope.contains(name) {
            return Err(SemanticError::redefined_context(name, node.location.clone()));
        }
        
        // Create a clone of the current scope
        let current_scope_clone = self.scope.clone();
        // Replace the current scope with a new one that has the clone as parent
        let old_scope = std::mem::replace(&mut self.scope, Scope::with_parent(current_scope_clone));
        
        // Define the context
        let context = Symbol::Context {
            name: name.clone(),
            properties: HashMap::new(),
        };
        
        self.scope.define(name, context);
        
        // Analyze all children
        for i in 0..node.child_count() {
            if let Some(child) = node.get_child_mut(i) {
                self.analyze_node(child)?;
            }
        }
        
        // Restore the old scope
        self.scope = old_scope;
        
        Ok(())
    }
    
    /// Analyze a function node
    fn analyze_function(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Get the function name
        let name = node.get_attribute("name").ok_or_else(|| {
            SemanticError::missing_attribute("name", node.location.clone())
        })?;
        
        // Get the function return type
        let return_type = node.get_attribute("return_type").unwrap_or(&"Void".to_string()).clone();
        
        // Check if the function is already defined
        if self.scope.contains(name) {
            return Err(SemanticError::redefined_function(name, node.location.clone()));
        }
        
        // Get the parameters
        let mut parameters = Vec::new();
        
        for i in 0..node.child_count() {
            if let Some(child) = node.get_child(i) {
                if child.kind == NodeKind::Parameter {
                    let param_name = child.get_attribute("name").ok_or_else(|| {
                        SemanticError::missing_attribute("name", child.location.clone())
                    })?;
                    
                    let param_type = child.get_attribute("type").unwrap_or(&"Any".to_string()).clone();
                    
                    parameters.push((param_name.clone(), param_type));
                }
            }
        }
        
        // Define the function
        let function = Symbol::Function {
            name: name.clone(),
            parameters: parameters.clone(),
            return_type: return_type.clone(),
        };
        
        self.scope.define(name, function);
        
        // Create a clone of the current scope
        let current_scope_clone = self.scope.clone();
        // Replace the current scope with a new one that has the clone as parent
        let old_scope = std::mem::replace(&mut self.scope, Scope::with_parent(current_scope_clone));
        
        // Define the parameters in the new scope
        for (param_name, param_type) in &parameters {
            let variable = Symbol::Variable {
                name: param_name.clone(),
                typ: param_type.clone(),
                mutable: false,
            };
            
            self.scope.define(param_name, variable);
        }
        
        // Set the current function return type
        let old_return_type = self.current_return_type.clone();
        self.current_return_type = Some(return_type);
        
        // Set the in_function flag
        let old_in_function = self.in_function;
        self.in_function = true;
        
        // Analyze the function body
        for i in 0..node.child_count() {
            if let Some(child) = node.get_child_mut(i) {
                if child.kind != NodeKind::Parameter {
                    self.analyze_node(child)?;
                }
            }
        }
        
        // Restore the old scope, return type, and in_function flag
        self.scope = old_scope;
        self.current_return_type = old_return_type;
        self.in_function = old_in_function;
        
        Ok(())
    }
    
    /// Analyze a variable node
    fn analyze_variable(&mut self, node: &mut Node) -> SemanticResult<()> {
        // First, extract all the information we need from the node
        let location = node.location.clone();
        
        // Get the variable name
        let name = match node.get_attribute("name") {
            Some(name) => name.clone(),
            None => return Err(SemanticError::missing_attribute("name", location)),
        };
        
        // Get the variable type
        let typ = node.get_attribute("type").unwrap_or(&"Any".to_string()).clone();
        
        // Get the variable mutability
        let mutable = node.get_attribute("mutable").map_or(false, |m| m == "true");
        
        // Check if the variable is already defined in this scope
        if self.scope.contains(&name) {
            return Err(SemanticError::redefined_variable(&name, location));
        }
        
        // Get the initializer if it exists
        let has_initializer = node.child_count() > 0;
        
        // Analyze the initializer if it exists
        if has_initializer {
            if let Some(initializer) = node.get_child_mut(0) {
                self.analyze_node(initializer)?;
            }
        }
        
        // Define the variable
        let variable = Symbol::Variable {
            name: name.clone(),
            typ,
            mutable,
        };
        
        self.scope.define(&name, variable);
        
        Ok(())
    }
    
    /// Analyze a statement node
    fn analyze_statement(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Analyze all children
        for i in 0..node.child_count() {
            if let Some(child) = node.get_child_mut(i) {
                self.analyze_node(child)?;
            }
        }
        
        Ok(())
    }
    
    /// Analyze a block node
    fn analyze_block(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Create a clone of the current scope
        let current_scope_clone = self.scope.clone();
        // Replace the current scope with a new one that has the clone as parent
        let old_scope = std::mem::replace(&mut self.scope, Scope::with_parent(current_scope_clone));
        
        // Analyze all children
        for i in 0..node.child_count() {
            if let Some(child) = node.get_child_mut(i) {
                self.analyze_node(child)?;
            }
        }
        
        // Restore the old scope
        self.scope = old_scope;
        
        Ok(())
    }
    
    /// Analyze an if node
    fn analyze_if(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Analyze the condition
        if let Some(condition) = node.get_child_mut(0) {
            self.analyze_node(condition)?;
        } else {
            return Err(SemanticError::missing_child(0, node.location.clone()));
        }
        
        // Analyze the then branch
        if let Some(then_branch) = node.get_child_mut(1) {
            self.analyze_node(then_branch)?;
        } else {
            return Err(SemanticError::missing_child(1, node.location.clone()));
        }
        
        // Analyze the else branch if it exists
        if let Some(else_branch) = node.get_child_mut(2) {
            self.analyze_node(else_branch)?;
        }
        
        Ok(())
    }
    
    /// Analyze a when node
    fn analyze_when(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Analyze the expression
        if let Some(expression) = node.get_child_mut(0) {
            self.analyze_node(expression)?;
        } else {
            return Err(SemanticError::missing_child(0, node.location.clone()));
        }
        
        // Analyze all cases
        for i in 1..node.child_count() {
            if let Some(child) = node.get_child_mut(i) {
                if child.kind == NodeKind::Case {
                    // Analyze the case expression
                    if let Some(case_expr) = child.get_child_mut(0) {
                        self.analyze_node(case_expr)?;
                    } else {
                        return Err(SemanticError::missing_child(0, child.location.clone()));
                    }
                    
                    // Analyze the case body
                    if let Some(case_body) = child.get_child_mut(1) {
                        self.analyze_node(case_body)?;
                    } else {
                        return Err(SemanticError::missing_child(1, child.location.clone()));
                    }
                } else if child.kind == NodeKind::Otherwise {
                    // Analyze the otherwise body
                    if let Some(otherwise_body) = child.get_child_mut(0) {
                        self.analyze_node(otherwise_body)?;
                    } else {
                        return Err(SemanticError::missing_child(0, child.location.clone()));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Analyze a for node
    fn analyze_for(&mut self, node: &mut Node) -> SemanticResult<()> {
        // First, extract all the information we need from the node
        let location = node.location.clone();
        
        // Get the loop variable name
        let variable = match node.get_attribute("variable") {
            Some(variable) => variable.clone(),
            None => return Err(SemanticError::missing_attribute("variable", location.clone())),
        };
        
        // Check if the collection expression exists
        let has_collection = node.child_count() > 0;
        if !has_collection {
            return Err(SemanticError::missing_child(0, location.clone()));
        }
        
        // Analyze the collection expression
        if let Some(collection) = node.get_child_mut(0) {
            self.analyze_node(collection)?;
        }
        
        // Check if the body exists
        let has_body = node.child_count() > 1;
        if !has_body {
            return Err(SemanticError::missing_child(1, location.clone()));
        }
        
        // Create a clone of the current scope
        let current_scope_clone = self.scope.clone();
        // Replace the current scope with a new one that has the clone as parent
        let old_scope = std::mem::replace(&mut self.scope, Scope::with_parent(current_scope_clone));
        
        // Define the loop variable
        let loop_var = Symbol::Variable {
            name: variable.clone(),
            typ: "Any".to_string(),
            mutable: false,
        };
        
        self.scope.define(&variable, loop_var);
        
        // Set the in_loop flag
        let old_in_loop = self.in_loop;
        self.in_loop = true;
        
        // Analyze the loop body
        if let Some(body) = node.get_child_mut(1) {
            self.analyze_node(body)?;
        }
        
        // Restore the old scope and in_loop flag
        self.scope = old_scope;
        self.in_loop = old_in_loop;
        
        Ok(())
    }
    
    /// Analyze a return node
    fn analyze_return(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Check if we're in a function
        if !self.in_function {
            return Err(SemanticError::return_outside_function(node.location.clone()));
        }
        
        // Analyze the return value if it exists
        if let Some(value) = node.get_child_mut(0) {
            self.analyze_node(value)?;
        }
        
        Ok(())
    }
    
    /// Analyze a with node
    fn analyze_with(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Get the context name
        let name = node.get_attribute("name").ok_or_else(|| {
            SemanticError::missing_attribute("name", node.location.clone())
        })?;
        
        // Check if the context exists
        if !self.scope.contains_recursive(name) {
            return Err(SemanticError::undefined_context(name, node.location.clone()));
        }
        
        // Analyze the body
        if let Some(body) = node.get_child_mut(0) {
            self.analyze_node(body)?;
        } else {
            return Err(SemanticError::missing_child(0, node.location.clone()));
        }
        
        Ok(())
    }
    
    /// Analyze a within node
    fn analyze_within(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Get the context name
        let name = node.get_attribute("name").ok_or_else(|| {
            SemanticError::missing_attribute("name", node.location.clone())
        })?;
        
        // Check if the context exists
        if !self.scope.contains_recursive(name) {
            return Err(SemanticError::undefined_context(name, node.location.clone()));
        }
        
        // Analyze the body
        if let Some(body) = node.get_child_mut(0) {
            self.analyze_node(body)?;
        } else {
            return Err(SemanticError::missing_child(0, node.location.clone()));
        }
        
        Ok(())
    }
    
    /// Analyze an intent node
    fn analyze_intent(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Analyze the intent expression
        if let Some(expression) = node.get_child_mut(0) {
            self.analyze_node(expression)?;
        } else {
            return Err(SemanticError::missing_child(0, node.location.clone()));
        }
        
        Ok(())
    }
    
    /// Analyze a parallel node
    fn analyze_parallel(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Get the selection strategy
        let strategy = node.get_attribute("strategy").unwrap_or(&"all".to_string()).clone();
        
        // Check if the strategy is valid
        if strategy != "all" && strategy != "fastest" && strategy != "best" {
            return Err(SemanticError::invalid_strategy(&strategy, node.location.clone()));
        }
        
        // Check if there are any paths
        if node.child_count() == 0 {
            return Err(SemanticError::no_paths(node.location.clone()));
        }
        
        // Analyze all paths
        for i in 0..node.child_count() {
            if let Some(child) = node.get_child_mut(i) {
                if child.kind == NodeKind::Path {
                    // Get the path name
                    let name = child.get_attribute("name").ok_or_else(|| {
                        SemanticError::missing_attribute("name", child.location.clone())
                    })?;
                    
                    // Analyze the path body
                    if let Some(body) = child.get_child_mut(0) {
                        self.analyze_node(body)?;
                    } else {
                        return Err(SemanticError::missing_child(0, child.location.clone()));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Analyze an apply node
    fn analyze_apply(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Analyze the vector expression
        if let Some(vector) = node.get_child_mut(0) {
            self.analyze_node(vector)?;
        } else {
            return Err(SemanticError::missing_child(0, node.location.clone()));
        }
        
        // Analyze the body
        if let Some(body) = node.get_child_mut(1) {
            self.analyze_node(body)?;
        } else {
            return Err(SemanticError::missing_child(1, node.location.clone()));
        }
        
        Ok(())
    }
    
    /// Analyze a semantic node
    fn analyze_semantic(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Get the semantic token
        let token = node.get_attribute("token").ok_or_else(|| {
            SemanticError::missing_attribute("token", node.location.clone())
        })?;
        
        // Check if the token is valid
        if !token.starts_with('@') {
            return Err(SemanticError::invalid_semantic_token(token, node.location.clone()));
        }
        
        // Analyze the value if it exists
        if let Some(value) = node.get_child_mut(0) {
            self.analyze_node(value)?;
        }
        
        Ok(())
    }
    
    /// Analyze an assignment node
    fn analyze_assignment(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Analyze the target
        if let Some(target) = node.get_child_mut(0) {
            self.analyze_node(target)?;
            
            // Check if the target is a valid assignment target
            if target.kind != NodeKind::Identifier {
                return Err(SemanticError::invalid_assignment_target(target.location.clone()));
            }
            
            // Get the target name
            let name = target.get_attribute("name").ok_or_else(|| {
                SemanticError::missing_attribute("name", target.location.clone())
            })?;
            
            // Check if the target is defined
            if let Some(symbol) = self.scope.get(name) {
                // Check if the target is mutable
                if let Symbol::Variable { mutable, .. } = symbol {
                    if !mutable {
                        return Err(SemanticError::invalid_assignment_target(target.location.clone()));
                    }
                } else {
                    return Err(SemanticError::invalid_assignment_target(target.location.clone()));
                }
            } else {
                return Err(SemanticError::undefined_variable(name, target.location.clone()));
            }
        } else {
            return Err(SemanticError::missing_child(0, node.location.clone()));
        }
        
        // Analyze the value
        if let Some(value) = node.get_child_mut(1) {
            self.analyze_node(value)?;
        } else {
            return Err(SemanticError::missing_child(1, node.location.clone()));
        }
        
        Ok(())
    }
    
    /// Analyze a binary node
    fn analyze_binary(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Get the operator
        let operator = node.get_attribute("operator").ok_or_else(|| {
            SemanticError::missing_attribute("operator", node.location.clone())
        })?;
        
        // Analyze the left operand
        if let Some(left) = node.get_child_mut(0) {
            self.analyze_node(left)?;
        } else {
            return Err(SemanticError::missing_child(0, node.location.clone()));
        }
        
        // Analyze the right operand
        if let Some(right) = node.get_child_mut(1) {
            self.analyze_node(right)?;
        } else {
            return Err(SemanticError::missing_child(1, node.location.clone()));
        }
        
        Ok(())
    }
    
    /// Analyze a unary node
    fn analyze_unary(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Get the operator
        let operator = node.get_attribute("operator").ok_or_else(|| {
            SemanticError::missing_attribute("operator", node.location.clone())
        })?;
        
        // Analyze the operand
        if let Some(operand) = node.get_child_mut(0) {
            self.analyze_node(operand)?;
        } else {
            return Err(SemanticError::missing_child(0, node.location.clone()));
        }
        
        Ok(())
    }
    
    /// Analyze a literal node
    fn analyze_literal(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Nothing to do for literals
        Ok(())
    }
    
    /// Analyze an identifier node
    fn analyze_identifier(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Get the identifier name
        let name = node.get_attribute("name").ok_or_else(|| {
            SemanticError::missing_attribute("name", node.location.clone())
        })?;
        
        // Check if the identifier is defined
        if !self.scope.contains_recursive(name) {
            return Err(SemanticError::undefined_variable(name, node.location.clone()));
        }
        
        Ok(())
    }
    
    /// Analyze a call node
    fn analyze_call(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Store the node's location and child count before any mutable operations
        let location = node.location.clone();
        let child_count = node.child_count();
        
        // Analyze the callee
        if let Some(callee) = node.get_child_mut(0) {
            self.analyze_node(callee)?;
            
            // Check if the callee is a valid function call target
            if callee.kind == NodeKind::Identifier {
                // Get the function name
                let name = callee.get_attribute("name").ok_or_else(|| {
                    SemanticError::missing_attribute("name", callee.location.clone())
                })?;
                
                let callee_location = callee.location.clone();
                
                // Check if the function is defined
                if let Some(symbol) = self.scope.get(name) {
                    match symbol {
                        Symbol::Function { parameters, .. } => {
                            // Check if the argument count matches
                            let expected = parameters.len();
                            let actual = child_count - 1;
                            
                            if expected != actual {
                                return Err(SemanticError::invalid_argument_count(
                                    name,
                                    expected,
                                    actual,
                                    location,
                                ));
                            }
                        },
                        _ => {
                            return Err(SemanticError::undefined_function(name, callee_location));
                        }
                    }
                } else {
                    return Err(SemanticError::undefined_function(name, callee_location));
                }
            }
        } else {
            return Err(SemanticError::missing_child(0, location));
        }
        
        // Analyze all arguments
        for i in 1..node.child_count() {
            if let Some(arg) = node.get_child_mut(i) {
                self.analyze_node(arg)?;
            }
        }
        
        Ok(())
    }
    
    /// Analyze a natural language node
    fn analyze_natural_language(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Nothing to do for natural language expressions
        Ok(())
    }
    
    /// Analyze a vector node
    fn analyze_vector(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Analyze the vector value
        if let Some(value) = node.get_child_mut(0) {
            self.analyze_node(value)?;
        } else {
            return Err(SemanticError::missing_child(0, node.location.clone()));
        }
        
        Ok(())
    }
    
    /// Analyze a grouping node
    fn analyze_grouping(&mut self, node: &mut Node) -> SemanticResult<()> {
        // Analyze the expression
        if let Some(expression) = node.get_child_mut(0) {
            self.analyze_node(expression)?;
        } else {
            return Err(SemanticError::missing_child(0, node.location.clone()));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Ast, Node, NodeKind};
    
    #[test]
    fn test_semantic_analyzer_new() {
        let analyzer = SemanticAnalyzer::new();
        
        assert!(!analyzer.in_function);
        assert!(!analyzer.in_loop);
        assert!(analyzer.current_return_type.is_none());
    }
    
    #[test]
    fn test_scope_new() {
        let scope = Scope::new();
        
        assert_eq!(scope.symbols.len(), 0);
        assert!(scope.parent.is_none());
    }
    
    #[test]
    fn test_scope_with_parent() {
        let parent = Scope::new();
        let scope = Scope::with_parent(parent);
        
        assert_eq!(scope.symbols.len(), 0);
        assert!(scope.parent.is_some());
    }
    
    #[test]
    fn test_scope_define() {
        let mut scope = Scope::new();
        
        let symbol = Symbol::Variable {
            name: "x".to_string(),
            typ: "Int".to_string(),
            mutable: false,
        };
        
        assert!(scope.define("x", symbol));
        assert!(!scope.define("x", Symbol::Variable {
            name: "x".to_string(),
            typ: "Int".to_string(),
            mutable: false,
        }));
    }
}
