//! Engine module for the LLM.lang runtime
//!
//! This module provides the execution engine for the LLM.lang runtime,
//! which executes the abstract syntax tree (AST) produced by the parser.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::parser::ast::{Ast, Node, NodeKind};
use crate::Value;

use super::error::RuntimeError;
use super::memory::Memory;
use super::context::Context;
use super::vector::Vector;
use super::parallel::Parallel;
use super::example::Example;
use super::nlp::NLP;
use super::interop::Interop;
use super::modify::{Modify, Modification};

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
    
    /// Whether to enable self-modifying capabilities
    pub self_modifying: bool,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            debug: false,
            max_memory: None,
            max_time: None,
            parallel: true,
            vectors: true,
            nlp: true,
            self_modifying: true,
        }
    }
}

/// The execution engine for the LLM.lang runtime
pub struct Engine {
    /// The engine options
    options: EngineOptions,
    
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
    
    /// The code modifier
    modify: Modify,
    
    /// The start time of execution
    start_time: Option<Instant>,
    
    /// The number of instructions executed
    instructions: u64,
    
    /// The peak memory usage in bytes
    peak_memory: usize,
}

impl Engine {
    /// Create a new execution engine
    pub fn new(options: EngineOptions) -> Self {
        let mut engine = Self {
            options,
            memory: Memory::new(),
            context: Context::new(),
            vector: Vector::new(),
            parallel: Parallel::new(),
            example: Example::new(),
            nlp: NLP::new(),
            interop: Interop::new(),
            modify: Modify::new(),
            start_time: None,
            instructions: 0,
            peak_memory: 0,
        };
        
        // Register standard library functions
        engine.register_stdlib_functions();
        
        engine
    }
    
    /// Register standard library functions
    fn register_stdlib_functions(&mut self) {
        // Create a standard library
        let stdlib = crate::stdlib::StdLib::new();
        
        // Register the standard library functions in the global context
        for (name, _) in &stdlib.functions {
            // Create a function node
            let location = crate::utils::SourceLocation::new(0, 0, 0, 0, "stdlib");
            let mut node = crate::parser::ast::Node {
                kind: crate::parser::ast::NodeKind::Function,
                location: location.clone(),
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Set the function name
            node.attributes.insert("name".to_string(), name.clone());
            
            // Register the function in the global context
            self.context.register_function(name, &node);
        }
    }
    
    /// Execute an AST
    pub fn execute(&mut self, ast: Ast) -> Result<Value, RuntimeError> {
        // Record the start time
        self.start_time = Some(Instant::now());
        
        // Reset execution statistics
        self.instructions = 0;
        self.peak_memory = 0;
        
        // Execute the AST
        let result = self.execute_node(&ast.root)?;
        
        // Create the execution result
        let execution_time = self.start_time.unwrap().elapsed().as_millis() as u64;
        
        // Create the execution statistics
        let stats = crate::ExecutionStats {
            execution_time,
            peak_memory: self.peak_memory,
            instructions: self.instructions,
        };
        
        // Create the execution result
        let execution_result = crate::ExecutionResult {
            value: result.clone(),
            stats,
        };
        
        Ok(result)
    }
    
    /// Execute a node
    fn execute_node(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Increment the instruction count
        self.instructions += 1;
        
        // Check if we've exceeded the maximum execution time
        if let Some(max_time) = self.options.max_time {
            let elapsed = self.start_time.unwrap().elapsed().as_millis() as u64;
            if elapsed > max_time {
                return Err(RuntimeError::time_limit_exceeded(elapsed, max_time));
            }
        }
        
        // Check if we've exceeded the maximum memory usage
        if let Some(max_memory) = self.options.max_memory {
            let memory_usage = self.memory.memory_usage();
            if memory_usage > max_memory {
                return Err(RuntimeError::memory_limit_exceeded(memory_usage, max_memory));
            }
        }
        
        // Update the peak memory usage
        let memory_usage = self.memory.memory_usage();
        if memory_usage > self.peak_memory {
            self.peak_memory = memory_usage;
        }
        
        // Execute the node based on its kind
        match node.kind {
            NodeKind::Program => self.execute_program(node),
            NodeKind::Context => self.execute_context(node),
            NodeKind::Function => self.execute_function(node),
            NodeKind::Parameter => Ok(Value::Void), // Parameters are handled by the function
            NodeKind::Variable => self.execute_variable(node),
            NodeKind::Statement => self.execute_statement(node),
            NodeKind::Block => self.execute_block(node),
            NodeKind::If => self.execute_if(node),
            NodeKind::When => self.execute_when(node),
            NodeKind::Case => Ok(Value::Void), // Cases are handled by the when statement
            NodeKind::Otherwise => Ok(Value::Void), // Otherwise is handled by the when statement
            NodeKind::For => self.execute_for(node),
            NodeKind::Return => self.execute_return(node),
            NodeKind::With => self.execute_with(node),
            NodeKind::Within => self.execute_within(node),
            NodeKind::Intent => self.execute_intent(node),
            NodeKind::Parallel => self.execute_parallel(node),
            NodeKind::Path => Ok(Value::Void), // Paths are handled by the parallel statement
            NodeKind::Apply => self.execute_apply(node),
            NodeKind::Semantic => self.execute_semantic(node),
            NodeKind::Examples => self.execute_examples(node),
            NodeKind::Example => self.execute_example(node),
            NodeKind::Assignment => self.execute_assignment(node),
            NodeKind::Binary => self.execute_binary(node),
            NodeKind::Unary => self.execute_unary(node),
            NodeKind::Literal => self.execute_literal(node),
            NodeKind::Identifier => self.execute_identifier(node),
            NodeKind::Call => self.execute_call(node),
            NodeKind::NaturalLanguage => self.execute_natural_language(node),
            NodeKind::Vector => self.execute_vector(node),
            NodeKind::Grouping => self.execute_grouping(node),
        }
    }
    
    /// Execute a program node
    fn execute_program(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Execute each child node
        let mut result = Value::Void;
        
        for child in &node.children {
            result = self.execute_node(child)?;
        }
        
        Ok(result)
    }
    
    /// Execute a context node
    fn execute_context(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the context name
        let name = node.get_attribute("name").ok_or_else(|| {
            RuntimeError::missing_attribute("name", node.location.clone())
        })?;
        
        // Create a new context
        self.context.create_context(name);
        
        // Execute each child node
        let mut result = Value::Void;
        
        for child in &node.children {
            result = self.execute_node(child)?;
        }
        
        // Return the context
        Ok(Value::Context(name.clone()))
    }
    
    /// Execute a function node
    fn execute_function(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the function name
        let name = node.get_attribute("name").ok_or_else(|| {
            RuntimeError::missing_attribute("name", node.location.clone())
        })?;
        
        // Register the function in the current context
        self.context.register_function(name, node);
        
        // Return the function
        Ok(Value::Function(name.clone()))
    }
    
    /// Execute a variable node
    fn execute_variable(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the variable name
        let name = node.get_attribute("name").ok_or_else(|| {
            RuntimeError::missing_attribute("name", node.location.clone())
        })?;
        
        // Execute the initializer
        let initializer = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        let value = self.execute_node(initializer)?;
        
        // Register the variable in the current context
        self.context.register_variable(name, value.clone());
        
        // Return the value
        Ok(value)
    }
    
    /// Execute a statement node
    fn execute_statement(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Execute the child node
        let child = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        self.execute_node(child)
    }
    
    /// Execute a block node
    fn execute_block(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Execute each child node
        let mut result = Value::Void;
        
        for child in &node.children {
            result = self.execute_node(child)?;
        }
        
        Ok(result)
    }
    
    /// Execute an if node
    fn execute_if(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Execute the condition
        let condition = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        let condition_value = self.execute_node(condition)?;
        
        // Check if the condition is true
        if Self::is_truthy(&condition_value) {
            // Execute the then branch
            let then_branch = node.get_child(1).ok_or_else(|| {
                RuntimeError::missing_child(1, node.location.clone())
            })?;
            
            self.execute_node(then_branch)
        } else if node.child_count() > 2 {
            // Execute the else branch
            let else_branch = node.get_child(2).ok_or_else(|| {
                RuntimeError::missing_child(2, node.location.clone())
            })?;
            
            self.execute_node(else_branch)
        } else {
            // No else branch
            Ok(Value::Void)
        }
    }
    
    /// Execute a when node
    fn execute_when(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Execute the expression
        let expression = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        let expression_value = self.execute_node(expression)?;
        
        // Check each case
        for i in 1..node.child_count() {
            let case = node.get_child(i).ok_or_else(|| {
                RuntimeError::missing_child(i, node.location.clone())
            })?;
            
            // Check if it's an otherwise case
            if case.get_child(0).map_or(false, |child| child.kind == NodeKind::Otherwise) {
                // Execute the otherwise case
                let body = case.get_child(1).ok_or_else(|| {
                    RuntimeError::missing_child(1, case.location.clone())
                })?;
                
                return self.execute_node(body);
            }
            
            // Execute the case expression
            let case_expression = case.get_child(0).ok_or_else(|| {
                RuntimeError::missing_child(0, case.location.clone())
            })?;
            
            let case_value = self.execute_node(case_expression)?;
            
            // Check if the case matches
            if Self::values_equal(&expression_value, &case_value) {
                // Execute the case body
                let body = case.get_child(1).ok_or_else(|| {
                    RuntimeError::missing_child(1, case.location.clone())
                })?;
                
                return self.execute_node(body);
            }
        }
        
        // No matching case
        Ok(Value::Void)
    }
    
    /// Execute a for node
    fn execute_for(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the loop variable name
        let variable = node.get_attribute("variable").ok_or_else(|| {
            RuntimeError::missing_attribute("variable", node.location.clone())
        })?;
        
        // Execute the collection expression
        let collection = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        let collection_value = self.execute_node(collection)?;
        
        // Get the loop body
        let body = node.get_child(1).ok_or_else(|| {
            RuntimeError::missing_child(1, node.location.clone())
        })?;
        
        // Iterate over the collection
        match collection_value {
            Value::List(items) => {
                let mut result = Value::Void;
                
                for item in items {
                    // Register the loop variable
                    self.context.register_variable(variable, item.clone());
                    
                    // Execute the loop body
                    result = self.execute_node(body)?;
                }
                
                Ok(result)
            }
            _ => Err(RuntimeError::invalid_type(
                "List",
                &format!("{:?}", collection_value),
                collection.location.clone(),
            )),
        }
    }
    
    /// Execute a return node
    fn execute_return(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Check if there's a return value
        if node.child_count() > 0 {
            // Execute the return value
            let value = node.get_child(0).ok_or_else(|| {
                RuntimeError::missing_child(0, node.location.clone())
            })?;
            
            self.execute_node(value)
        } else {
            // No return value
            Ok(Value::Void)
        }
    }
    
    /// Execute a with node
    fn execute_with(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the context name
        let name = node.get_attribute("name").ok_or_else(|| {
            RuntimeError::missing_attribute("name", node.location.clone())
        })?;
        
        // Create a new context
        self.context.create_context(name);
        
        // Switch to the context
        self.context.switch_context(name);
        
        // Execute the body
        let body = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        let result = self.execute_node(body)?;
        
        // Switch back to the previous context
        self.context.switch_back();
        
        Ok(result)
    }
    
    /// Execute a within node
    fn execute_within(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the context name
        let name = node.get_attribute("name").ok_or_else(|| {
            RuntimeError::missing_attribute("name", node.location.clone())
        })?;
        
        // Switch to the context
        self.context.switch_context(name);
        
        // Execute the body
        let body = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        let result = self.execute_node(body)?;
        
        // Switch back to the previous context
        self.context.switch_back();
        
        Ok(result)
    }
    
    /// Execute an intent node
    fn execute_intent(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Check if NLP is enabled
        if !self.options.nlp {
            return Err(RuntimeError::feature_disabled("NLP", node.location.clone()));
        }
        
        // Execute the intent expression
        let expression = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        let expression_value = self.execute_node(expression)?;
        
        // Process the intent
        match expression_value {
            Value::String(intent) => {
                self.nlp.process_intent(&intent)
            }
            _ => Err(RuntimeError::invalid_type(
                "String",
                &format!("{:?}", expression_value),
                expression.location.clone(),
            )),
        }
    }
    
    /// Execute a parallel node
    fn execute_parallel(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Check if parallel execution is enabled
        if !self.options.parallel {
            return Err(RuntimeError::feature_disabled("Parallel", node.location.clone()));
        }
        
        // Get the selection strategy
        let strategy = node.get_attribute("strategy").ok_or_else(|| {
            RuntimeError::missing_attribute("strategy", node.location.clone())
        })?;
        
        // Execute each path
        let mut results = Vec::new();
        let mut execution_times = Vec::new();
        
        for child in &node.children {
            // Check if it's a path node
            if child.kind != NodeKind::Path {
                continue;
            }
            
            // Get the path name
            let name = child.get_attribute("name").ok_or_else(|| {
                RuntimeError::missing_attribute("name", child.location.clone())
            })?;
            
            // Get the path body
            let body = child.get_child(0).ok_or_else(|| {
                RuntimeError::missing_child(0, child.location.clone())
            })?;
            
            // Execute the path
            let start_time = Instant::now();
            let result = self.execute_node(body)?;
            let execution_time = start_time.elapsed();
            
            // Store the result and execution time
            results.push((name.clone(), result));
            execution_times.push((name.clone(), execution_time));
        }
        
        // Apply the selection strategy
        match strategy.as_str() {
            "fastest" => {
                // Find the fastest path
                let fastest = execution_times.iter().min_by_key(|(_, time)| time).ok_or_else(|| {
                    RuntimeError::no_paths(node.location.clone())
                })?;
                
                // Return the result of the fastest path
                let (name, _) = fastest;
                let (_, result) = results.iter().find(|(n, _)| n == name).unwrap();
                
                Ok(result.clone())
            }
            "best" => {
                // In a real implementation, this would use a quality metric
                // For now, just return the first result
                if results.is_empty() {
                    Err(RuntimeError::no_paths(node.location.clone()))
                } else {
                    let (_, result) = &results[0];
                    Ok(result.clone())
                }
            }
            "all" => {
                // Return all results as a list
                let values: Vec<Value> = results.into_iter().map(|(_, r)| r).collect();
                Ok(Value::List(values))
            }
            _ => Err(RuntimeError::invalid_strategy(strategy, node.location.clone())),
        }
    }
    
    /// Execute an apply node
    fn execute_apply(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Check if vector operations are enabled
        if !self.options.vectors {
            return Err(RuntimeError::feature_disabled("Vector", node.location.clone()));
        }
        
        // Execute the vector expression
        let vector_expr = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        let vector_value = self.execute_node(vector_expr)?;
        
        // Apply the vector to the body
        match vector_value {
            Value::Vector(vector) => {
                // Set the current vector
                self.vector.set_current_vector(vector);
                
                // Execute the body
                let body = node.get_child(1).ok_or_else(|| {
                    RuntimeError::missing_child(1, node.location.clone())
                })?;
                
                let result = self.execute_node(body)?;
                
                // Clear the current vector
                self.vector.clear_current_vector();
                
                Ok(result)
            }
            _ => Err(RuntimeError::invalid_type(
                "Vector",
                &format!("{:?}", vector_value),
                vector_expr.location.clone(),
            )),
        }
    }
    
    /// Execute a semantic node
    fn execute_semantic(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the token
        let token = node.get_attribute("token").ok_or_else(|| {
            RuntimeError::missing_attribute("token", node.location.clone())
        })?;
        
        // Execute the semantic token
        match token.as_str() {
            "@remember" => {
                // Get the name
                let name = node.get_attribute("name").ok_or_else(|| {
                    RuntimeError::missing_attribute("name", node.location.clone())
                })?;
                
                // Execute the value
                let value_node = node.get_child(0).ok_or_else(|| {
                    RuntimeError::missing_child(0, node.location.clone())
                })?;
                
                let value = self.execute_node(value_node)?;
                
                // Remember the value
                self.memory.remember(name, value.clone());
                
                Ok(value)
            }
            "@recall" => {
                // Check if there's a specific key
                if let Some(key) = node.get_attribute("key") {
                    // Recall the value with the key
                    self.memory.recall(key)
                } else {
                    // Recall the most relevant value
                    self.memory.recall_most_relevant()
                }
            }
            "@modify" => {
                // Check if self-modifying capabilities are enabled
                if !self.options.self_modifying {
                    return Err(RuntimeError::feature_disabled("Self-modifying", node.location.clone()));
                }
                
                // Get the target
                let target = node.get_attribute("target").ok_or_else(|| {
                    RuntimeError::missing_attribute("target", node.location.clone())
                })?;
                
                // Get the operation
                let operation = node.get_attribute("operation").ok_or_else(|| {
                    RuntimeError::missing_attribute("operation", node.location.clone())
                })?;
                
                // Execute the modification
                self.execute_modification(target, operation, node)?;
                
                Ok(Value::Void)
            }
            _ => Err(RuntimeError::unknown_semantic_token(token, node.location.clone())),
        }
    }
    
    /// Execute a code modification
    fn execute_modification(&mut self, target: &str, operation: &str, node: &Node) -> Result<(), RuntimeError> {
        // Get the source code
        let source = if let Some(source) = self.modify.get_source(target) {
            source.clone()
        } else {
            // Try to load the source code from a file
            match std::fs::read_to_string(target) {
                Ok(source) => {
                    // Cache the source code
                    self.modify.cache_source(target, &source);
                    source
                }
                Err(e) => {
                    return Err(RuntimeError::new(
                        &format!("Failed to load source code: {}", e),
                        node.location.clone(),
                    ));
                }
            }
        };
        
        // Parse the source code into an AST
        let ast = self.modify.parse_source(&source)?;
        
        // Cache the AST
        self.modify.cache_ast(target, ast.clone());
        
        // Create a list of modifications
        let mut modifications = Vec::new();
        
        // Apply the operation
        match operation {
            "replace" => {
                // Get the path
                let path_str = node.get_attribute("path").ok_or_else(|| {
                    RuntimeError::missing_attribute("path", node.location.clone())
                })?;
                
                // Parse the path
                let path = path_str.split('.').map(|s| s.parse::<usize>().unwrap_or(0)).collect::<Vec<usize>>();
                
                // Get the new code
                let new_code = node.get_attribute("code").ok_or_else(|| {
                    RuntimeError::missing_attribute("code", node.location.clone())
                })?;
                
                // Parse the new code into an AST
                let new_ast = self.modify.parse_source(new_code)?;
                
                // Add the modification
                modifications.push(Modification::ReplaceNode {
                    path,
                    new_node: (*new_ast.root()).clone(),
                });
            }
            "insert" => {
                // Get the path
                let path_str = node.get_attribute("path").ok_or_else(|| {
                    RuntimeError::missing_attribute("path", node.location.clone())
                })?;
                
                // Parse the path
                let path = path_str.split('.').map(|s| s.parse::<usize>().unwrap_or(0)).collect::<Vec<usize>>();
                
                // Get the position
                let position = node.get_attribute("position").ok_or_else(|| {
                    RuntimeError::missing_attribute("position", node.location.clone())
                })?;
                
                let position = position.parse::<usize>().map_err(|_| {
                    RuntimeError::new(
                        &format!("Invalid position: {}", position),
                        node.location.clone(),
                    )
                })?;
                
                // Get the new code
                let new_code = node.get_attribute("code").ok_or_else(|| {
                    RuntimeError::missing_attribute("code", node.location.clone())
                })?;
                
                // Parse the new code into an AST
                let new_ast = self.modify.parse_source(new_code)?;
                
                // Add the modification
                modifications.push(Modification::InsertNode {
                    path,
                    new_node: (*new_ast.root()).clone(),
                    position,
                });
            }
            "delete" => {
                // Get the path
                let path_str = node.get_attribute("path").ok_or_else(|| {
                    RuntimeError::missing_attribute("path", node.location.clone())
                })?;
                
                // Parse the path
                let path = path_str.split('.').map(|s| s.parse::<usize>().unwrap_or(0)).collect::<Vec<usize>>();
                
                // Add the modification
                modifications.push(Modification::DeleteNode {
                    path,
                });
            }
            "modify" => {
                // Get the path
                let path_str = node.get_attribute("path").ok_or_else(|| {
                    RuntimeError::missing_attribute("path", node.location.clone())
                })?;
                
                // Parse the path
                let path = path_str.split('.').map(|s| s.parse::<usize>().unwrap_or(0)).collect::<Vec<usize>>();
                
                // Get the attribute name
                let name = node.get_attribute("name").ok_or_else(|| {
                    RuntimeError::missing_attribute("name", node.location.clone())
                })?;
                
                // Get the attribute value
                let value = node.get_attribute("value").ok_or_else(|| {
                    RuntimeError::missing_attribute("value", node.location.clone())
                })?;
                
                // Add the modification
                modifications.push(Modification::ModifyAttribute {
                    path,
                    name: name.to_string(),
                    value: value.to_string(),
                });
            }
            _ => {
                return Err(RuntimeError::new(
                    &format!("Unknown operation: {}", operation),
                    node.location.clone(),
                ));
            }
        }
        
        // Apply the modifications
        let mut modified_ast = ast.clone();
        self.modify.modify_ast(&mut modified_ast, &modifications)?;
        
        // Update the AST cache
        self.modify.cache_ast(target, modified_ast.clone());
        
        // Generate the new source code
        let new_source = self.modify.generate_source(&modified_ast)?;
        
        // Update the source cache
        self.modify.cache_source(target, &new_source);
        
        // Try to write the new source code to a file
        if target.ends_with(".llm") {
            match std::fs::write(target, new_source) {
                Ok(_) => {}
                Err(e) => {
                    return Err(RuntimeError::new(
                        &format!("Failed to write source code: {}", e),
                        node.location.clone(),
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    /// Execute an assignment node
    fn execute_assignment(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the left-hand side
        let lhs = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        // Get the right-hand side
        let rhs = node.get_child(1).ok_or_else(|| {
            RuntimeError::missing_child(1, node.location.clone())
        })?;
        
        // Execute the right-hand side
        let value = self.execute_node(rhs)?;
        
        // Assign the value to the left-hand side
        match lhs.kind {
            NodeKind::Identifier => {
                // Get the variable name
                let name = lhs.get_attribute("name").ok_or_else(|| {
                    RuntimeError::missing_attribute("name", lhs.location.clone())
                })?;
                
                // Assign the value to the variable
                self.context.assign_variable(name, value.clone())?;
                
                Ok(value)
            }
            NodeKind::Binary if lhs.get_attribute("operator") == Some(&".".to_string()) => {
                // Property access
                let object = lhs.get_child(0).ok_or_else(|| {
                    RuntimeError::missing_child(0, lhs.location.clone())
                })?;
                
                let property = lhs.get_attribute("name").ok_or_else(|| {
                    RuntimeError::missing_attribute("name", lhs.location.clone())
                })?;
                
                // Execute the object
                let object_value = self.execute_node(object)?;
                
                // Assign the value to the property
                match object_value {
                    Value::Map(mut map) => {
                        map.insert(property.clone(), value.clone());
                        
                        // Update the variable
                        if let NodeKind::Identifier = object.kind {
                            let name = object.get_attribute("name").ok_or_else(|| {
                                RuntimeError::missing_attribute("name", object.location.clone())
                            })?;
                            
                            self.context.assign_variable(name, Value::Map(map))?;
                        }
                        
                        Ok(value)
                    }
                    _ => Err(RuntimeError::invalid_assignment_target(lhs.location.clone())),
                }
            }
            _ => Err(RuntimeError::invalid_assignment_target(lhs.location.clone())),
        }
    }
    
    /// Execute a binary node
    fn execute_binary(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the operator
        let operator = node.get_attribute("operator").ok_or_else(|| {
            RuntimeError::missing_attribute("operator", node.location.clone())
        })?;
        
        // Get the left-hand side
        let lhs = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        // Get the right-hand side
        let rhs = node.get_child(1).ok_or_else(|| {
            RuntimeError::missing_child(1, node.location.clone())
        })?;
        
        // Execute the left-hand side
        let lhs_value = self.execute_node(lhs)?;
        
        // Execute the right-hand side
        let rhs_value = self.execute_node(rhs)?;
        
        // Apply the operator
        match operator.as_str() {
            "+" => Self::add(&lhs_value, &rhs_value, node.location.clone()),
            "-" => Self::subtract(&lhs_value, &rhs_value, node.location.clone()),
            "*" => Self::multiply(&lhs_value, &rhs_value, node.location.clone()),
            "/" => Self::divide(&lhs_value, &rhs_value, node.location.clone()),
            "%" => Self::modulo(&lhs_value, &rhs_value, node.location.clone()),
            "==" => Ok(Value::Bool(Self::values_equal(&lhs_value, &rhs_value))),
            "!=" => Ok(Value::Bool(!Self::values_equal(&lhs_value, &rhs_value))),
            "<" => Self::less_than(&lhs_value, &rhs_value, node.location.clone()),
            ">" => Self::greater_than(&lhs_value, &rhs_value, node.location.clone()),
            "<=" => Self::less_than_or_equal(&lhs_value, &rhs_value, node.location.clone()),
            ">=" => Self::greater_than_or_equal(&lhs_value, &rhs_value, node.location.clone()),
            "and" => Ok(Value::Bool(Self::is_truthy(&lhs_value) && Self::is_truthy(&rhs_value))),
            "or" => Ok(Value::Bool(Self::is_truthy(&lhs_value) || Self::is_truthy(&rhs_value))),
            "." => Self::property_access(&lhs_value, &rhs_value, node.location.clone()),
            _ => Err(RuntimeError::unknown_operator(operator, node.location.clone())),
        }
    }
    
    /// Execute a unary node
    fn execute_unary(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the operator
        let operator = node.get_attribute("operator").ok_or_else(|| {
            RuntimeError::missing_attribute("operator", node.location.clone())
        })?;
        
        // Get the operand
        let operand = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        // Execute the operand
        let operand_value = self.execute_node(operand)?;
        
        // Apply the operator
        match operator.as_str() {
            "-" => Self::negate(&operand_value, node.location.clone()),
            "!" => Ok(Value::Bool(!Self::is_truthy(&operand_value))),
            _ => Err(RuntimeError::unknown_operator(operator, node.location.clone())),
        }
    }
    
    /// Execute a literal node
    fn execute_literal(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the type
        let typ = node.get_attribute("type").ok_or_else(|| {
            RuntimeError::missing_attribute("type", node.location.clone())
        })?;
        
        // Get the value
        let value = node.get_attribute("value").ok_or_else(|| {
            RuntimeError::missing_attribute("value", node.location.clone())
        })?;
        
        // Create the value
        match typ.as_str() {
            "Int" => {
                let int = value.parse::<i64>().map_err(|_| {
                    RuntimeError::invalid_literal(value, "Int", node.location.clone())
                })?;
                
                Ok(Value::Int(int))
            }
            "Float" => {
                let float = value.parse::<f64>().map_err(|_| {
                    RuntimeError::invalid_literal(value, "Float", node.location.clone())
                })?;
                
                Ok(Value::Float(float))
            }
            "String" => {
                // Remove the quotes
                let string = value[1..value.len() - 1].to_string();
                
                Ok(Value::String(string))
            }
            "Bool" => {
                let bool = value.parse::<bool>().map_err(|_| {
                    RuntimeError::invalid_literal(value, "Bool", node.location.clone())
                })?;
                
                Ok(Value::Bool(bool))
            }
            "Null" => Ok(Value::Void),
            _ => Err(RuntimeError::unknown_type(typ, node.location.clone())),
        }
    }
    
    /// Execute an identifier node
    fn execute_identifier(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the name
        let name = node.get_attribute("name").ok_or_else(|| {
            RuntimeError::missing_attribute("name", node.location.clone())
        })?;
        
        // Look up the variable
        self.context.get_variable(name).ok_or_else(|| {
            RuntimeError::undefined_variable(name, node.location.clone())
        })
    }
    
    /// Execute a call node
    fn execute_call(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the callee
        let callee = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        // Execute the callee
        let callee_value = self.execute_node(callee)?;
        
        // Get the arguments
        let mut arguments = Vec::new();
        
        for i in 1..node.child_count() {
            let argument = node.get_child(i).ok_or_else(|| {
                RuntimeError::missing_child(i, node.location.clone())
            })?;
            
            let argument_value = self.execute_node(argument)?;
            arguments.push(argument_value);
        }
        
        // Call the function
        match callee_value {
            Value::Function(name) => {
                // Check if it's a standard library function
                let stdlib = crate::stdlib::StdLib::new();
                if let Some(function) = stdlib.get_function(&name) {
                    // Call the standard library function
                    return function(arguments);
                }
                
                // Look up the function
                let function = self.context.get_function(&name).ok_or_else(|| {
                    RuntimeError::undefined_function(&name, node.location.clone())
                })?;
                
                // Check the argument count
                let parameter_count = function.children.iter().filter(|child| child.kind == NodeKind::Parameter).count();
                
                if arguments.len() != parameter_count {
                    return Err(RuntimeError::invalid_argument_count(
                        &name,
                        parameter_count,
                        arguments.len(),
                        node.location.clone(),
                    ));
                }
                
                // Create a new context for the function
                self.context.push_frame();
                
                // Register the parameters
                let mut parameter_index = 0;
                
                for i in 0..function.child_count() {
                    let child = function.get_child(i).unwrap();
                    
                    if child.kind == NodeKind::Parameter {
                        let parameter_name = child.get_attribute("name").ok_or_else(|| {
                            RuntimeError::missing_attribute("name", child.location.clone())
                        })?;
                        
                        self.context.register_variable(parameter_name, arguments[parameter_index].clone());
                        parameter_index += 1;
                    }
                }
                
                // Execute the function body
                let body = function.children.iter().find(|child| child.kind == NodeKind::Block).ok_or_else(|| {
                    RuntimeError::missing_body(&name, function.location.clone())
                })?;
                
                let result = self.execute_node(body)?;
                
                // Pop the function context
                self.context.pop_frame();
                
                Ok(result)
            }
            _ => Err(RuntimeError::not_callable(
                &format!("{:?}", callee_value),
                node.location.clone(),
            )),
        }
    }
    
    /// Execute a natural language node
    fn execute_natural_language(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Check if NLP is enabled
        if !self.options.nlp {
            return Err(RuntimeError::feature_disabled("NLP", node.location.clone()));
        }
        
        // Get the value
        let value = node.get_attribute("value").ok_or_else(|| {
            RuntimeError::missing_attribute("value", node.location.clone())
        })?;
        
        // Process the natural language
        self.nlp.process_natural_language(value)
    }
    
    /// Execute a vector node
    fn execute_vector(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Check if vector operations are enabled
        if !self.options.vectors {
            return Err(RuntimeError::feature_disabled("Vector", node.location.clone()));
        }
        
        // Get the vector name
        let name = node.get_attribute("name").ok_or_else(|| {
            RuntimeError::missing_attribute("name", node.location.clone())
        })?;
        
        // Execute the vector value
        let value = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        let value_result = self.execute_node(value)?;
        
        // Create the vector
        match value_result {
            Value::String(text) => {
                let vector = self.vector.embed(&text)?;
                
                // Register the vector
                self.context.register_variable(name, Value::Vector(vector.clone()));
                
                Ok(Value::Vector(vector))
            }
            _ => Err(RuntimeError::invalid_type(
                "String",
                &format!("{:?}", value_result),
                value.location.clone(),
            )),
        }
    }
    
    /// Execute a grouping node
    fn execute_grouping(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Execute the expression
        let expression = node.get_child(0).ok_or_else(|| {
            RuntimeError::missing_child(0, node.location.clone())
        })?;
        
        self.execute_node(expression)
    }
    
    /// Execute an examples node
    fn execute_examples(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Get the function name
        let function_name = node.get_attribute("function").ok_or_else(|| {
            RuntimeError::missing_attribute("function", node.location.clone())
        })?;
        
        // Create a function from the examples
        let mut examples = Vec::new();
        
        // Process each example
        for child in &node.children {
            if child.kind == NodeKind::Example {
                // Get the input and output
                let input = child.get_child(0).ok_or_else(|| {
                    RuntimeError::missing_child(0, child.location.clone())
                })?;
                
                let output = child.get_child(1).ok_or_else(|| {
                    RuntimeError::missing_child(1, child.location.clone())
                })?;
                
                // Execute the input and output
                let input_value = self.execute_node(input)?;
                let output_value = self.execute_node(output)?;
                
                // Add the example
                examples.push((input_value, output_value));
            }
        }
        
        // Register the function with the example executor
        self.example.register_function(function_name, examples);
        
        // Return the function
        Ok(Value::Function(function_name.clone()))
    }
    
    /// Execute an example node
    fn execute_example(&mut self, node: &Node) -> Result<Value, RuntimeError> {
        // Examples are handled by the examples statement
        Ok(Value::Void)
    }
    
    /// Check if a value is truthy
    fn is_truthy(value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Map(m) => !m.is_empty(),
            Value::Vector(_) => true,
            Value::Function(_) => true,
            Value::Context(_) => true,
            Value::Void => false,
        }
    }
    
    /// Check if two values are equal
    fn values_equal(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::List(a), Value::List(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                
                for (a_item, b_item) in a.iter().zip(b.iter()) {
                    if !Self::values_equal(a_item, b_item) {
                        return false;
                    }
                }
                
                true
            }
            (Value::Map(a), Value::Map(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                
                for (key, a_value) in a {
                    if let Some(b_value) = b.get(key) {
                        if !Self::values_equal(a_value, b_value) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                true
            }
            (Value::Vector(a), Value::Vector(b)) => a == b,
            (Value::Function(a), Value::Function(b)) => a == b,
            (Value::Context(a), Value::Context(b)) => a == b,
            (Value::Void, Value::Void) => true,
            _ => false,
        }
    }
    
    /// Add two values
    fn add(a: &Value, b: &Value, location: crate::utils::SourceLocation) -> Result<Value, RuntimeError> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a.clone() + b)),
            (Value::List(a), Value::List(b)) => {
                let mut result = a.clone();
                result.extend(b.clone());
                Ok(Value::List(result))
            }
            (Value::Vector(a), Value::Vector(b)) => {
                let mut result = a.clone();
                for (i, val) in b.iter().enumerate() {
                    if i < result.len() {
                        result[i] += val;
                    }
                }
                Ok(Value::Vector(result))
            }
            _ => Err(RuntimeError::invalid_operation(
                "+",
                &format!("{:?}", a),
                &format!("{:?}", b),
                location,
            )),
        }
    }
    
    /// Subtract two values
    fn subtract(a: &Value, b: &Value, location: crate::utils::SourceLocation) -> Result<Value, RuntimeError> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
            (Value::Vector(a), Value::Vector(b)) => {
                let mut result = a.clone();
                for (i, val) in b.iter().enumerate() {
                    if i < result.len() {
                        result[i] -= val;
                    }
                }
                Ok(Value::Vector(result))
            }
            _ => Err(RuntimeError::invalid_operation(
                "-",
                &format!("{:?}", a),
                &format!("{:?}", b),
                location,
            )),
        }
    }
    
    /// Multiply two values
    fn multiply(a: &Value, b: &Value, location: crate::utils::SourceLocation) -> Result<Value, RuntimeError> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
            (Value::String(a), Value::Int(b)) => {
                if *b < 0 {
                    return Err(RuntimeError::invalid_operation(
                        "*",
                        &format!("{:?}", a),
                        &format!("{:?}", b),
                        location,
                    ));
                }
                
                Ok(Value::String(a.repeat(*b as usize)))
            }
            (Value::List(a), Value::Int(b)) => {
                if *b < 0 {
                    return Err(RuntimeError::invalid_operation(
                        "*",
                        &format!("{:?}", a),
                        &format!("{:?}", b),
                        location,
                    ));
                }
                
                let mut result = Vec::new();
                for _ in 0..*b {
                    result.extend(a.clone());
                }
                
                Ok(Value::List(result))
            }
            (Value::Vector(a), Value::Float(b)) => {
                let mut result = a.clone();
                for val in &mut result {
                    *val *= *b;
                }
                Ok(Value::Vector(result))
            }
            _ => Err(RuntimeError::invalid_operation(
                "*",
                &format!("{:?}", a),
                &format!("{:?}", b),
                location,
            )),
        }
    }
    
    /// Divide two values
    fn divide(a: &Value, b: &Value, location: crate::utils::SourceLocation) -> Result<Value, RuntimeError> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::division_by_zero(location));
                }
                
                Ok(Value::Int(a / b))
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::division_by_zero(location));
                }
                
                Ok(Value::Float(a / b))
            }
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::division_by_zero(location));
                }
                
                Ok(Value::Float(*a as f64 / b))
            }
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::division_by_zero(location));
                }
                
                Ok(Value::Float(a / *b as f64))
            }
            (Value::Vector(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::division_by_zero(location));
                }
                
                let mut result = a.clone();
                for val in &mut result {
                    *val /= *b;
                }
                Ok(Value::Vector(result))
            }
            _ => Err(RuntimeError::invalid_operation(
                "/",
                &format!("{:?}", a),
                &format!("{:?}", b),
                location,
            )),
        }
    }
    
    /// Compute the modulo of two values
    fn modulo(a: &Value, b: &Value, location: crate::utils::SourceLocation) -> Result<Value, RuntimeError> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::division_by_zero(location));
                }
                
                Ok(Value::Int(a % b))
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::division_by_zero(location));
                }
                
                Ok(Value::Float(a % b))
            }
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::division_by_zero(location));
                }
                
                Ok(Value::Float(*a as f64 % b))
            }
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::division_by_zero(location));
                }
                
                Ok(Value::Float(a % *b as f64))
            }
            _ => Err(RuntimeError::invalid_operation(
                "%",
                &format!("{:?}", a),
                &format!("{:?}", b),
                location,
            )),
        }
    }
    
    /// Check if a value is less than another
    fn less_than(a: &Value, b: &Value, location: crate::utils::SourceLocation) -> Result<Value, RuntimeError> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) < *b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a < (*b as f64))),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a < b)),
            _ => Err(RuntimeError::invalid_operation(
                "<",
                &format!("{:?}", a),
                &format!("{:?}", b),
                location,
            )),
        }
    }
    
    /// Check if a value is greater than another
    fn greater_than(a: &Value, b: &Value, location: crate::utils::SourceLocation) -> Result<Value, RuntimeError> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) > *b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a > (*b as f64))),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a > b)),
            _ => Err(RuntimeError::invalid_operation(
                ">",
                &format!("{:?}", a),
                &format!("{:?}", b),
                location,
            )),
        }
    }
    
    /// Check if a value is less than or equal to another
    fn less_than_or_equal(a: &Value, b: &Value, location: crate::utils::SourceLocation) -> Result<Value, RuntimeError> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) <= *b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a <= (*b as f64))),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a <= b)),
            _ => Err(RuntimeError::invalid_operation(
                "<=",
                &format!("{:?}", a),
                &format!("{:?}", b),
                location,
            )),
        }
    }
    
    /// Check if a value is greater than or equal to another
    fn greater_than_or_equal(a: &Value, b: &Value, location: crate::utils::SourceLocation) -> Result<Value, RuntimeError> {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) >= *b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a >= (*b as f64))),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a >= b)),
            _ => Err(RuntimeError::invalid_operation(
                ">=",
                &format!("{:?}", a),
                &format!("{:?}", b),
                location,
            )),
        }
    }
    
    /// Access a property of a value
    fn property_access(a: &Value, b: &Value, location: crate::utils::SourceLocation) -> Result<Value, RuntimeError> {
        match (a, b) {
            (Value::Map(map), Value::String(key)) => {
                map.get(key).cloned().ok_or_else(|| {
                    RuntimeError::undefined_property(key, location)
                })
            }
            _ => Err(RuntimeError::invalid_operation(
                ".",
                &format!("{:?}", a),
                &format!("{:?}", b),
                location,
            )),
        }
    }
    
    /// Negate a value
    fn negate(a: &Value, location: crate::utils::SourceLocation) -> Result<Value, RuntimeError> {
        match a {
            Value::Int(a) => Ok(Value::Int(-a)),
            Value::Float(a) => Ok(Value::Float(-a)),
            Value::Vector(a) => {
                let mut result = a.clone();
                for val in &mut result {
                    *val = -*val;
                }
                Ok(Value::Vector(result))
            }
            _ => Err(RuntimeError::invalid_unary_operation(
                "-",
                &format!("{:?}", a),
                location,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Ast, Node, NodeKind};
    use crate::utils::SourceLocation;
    
    #[test]
    fn test_execute_literal() {
        let mut engine = Engine::new(EngineOptions::default());
        
        // Create a literal node
        let location = SourceLocation::new(1, 1, 1, 1, "test.llm");
        let mut node = Node {
            kind: NodeKind::Literal,
            location: location.clone(),
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Test an integer literal
        node.attributes.insert("type".to_string(), "Int".to_string());
        node.attributes.insert("value".to_string(), "42".to_string());
        
        let result = engine.execute_literal(&node).unwrap();
        assert_eq!(result, Value::Int(42));
        
        // Test a float literal
        node.attributes.insert("type".to_string(), "Float".to_string());
        node.attributes.insert("value".to_string(), "3.14".to_string());
        
        let result = engine.execute_literal(&node).unwrap();
        assert_eq!(result, Value::Float(3.14));
        
        // Test a string literal
        node.attributes.insert("type".to_string(), "String".to_string());
        node.attributes.insert("value".to_string(), "\"hello\"".to_string());
        
        let result = engine.execute_literal(&node).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
        
        // Test a boolean literal
        node.attributes.insert("type".to_string(), "Bool".to_string());
        node.attributes.insert("value".to_string(), "true".to_string());
        
        let result = engine.execute_literal(&node).unwrap();
        assert_eq!(result, Value::Bool(true));
        
        // Test a null literal
        node.attributes.insert("type".to_string(), "Null".to_string());
        node.attributes.insert("value".to_string(), "null".to_string());
        
        let result = engine.execute_literal(&node).unwrap();
        assert_eq!(result, Value::Void);
    }
    
    #[test]
    fn test_is_truthy() {
        assert!(Engine::is_truthy(&Value::Bool(true)));
        assert!(!Engine::is_truthy(&Value::Bool(false)));
        assert!(Engine::is_truthy(&Value::Int(42)));
        assert!(!Engine::is_truthy(&Value::Int(0)));
        assert!(Engine::is_truthy(&Value::Float(3.14)));
        assert!(!Engine::is_truthy(&Value::Float(0.0)));
        assert!(Engine::is_truthy(&Value::String("hello".to_string())));
        assert!(!Engine::is_truthy(&Value::String("".to_string())));
        assert!(Engine::is_truthy(&Value::List(vec![Value::Int(42)])));
        assert!(!Engine::is_truthy(&Value::List(vec![])));
        assert!(Engine::is_truthy(&Value::Map(std::collections::HashMap::from([
            ("key".to_string(), Value::Int(42))
        ]))));
        assert!(!Engine::is_truthy(&Value::Map(std::collections::HashMap::new())));
        assert!(Engine::is_truthy(&Value::Vector(vec![1.0, 2.0, 3.0])));
        assert!(Engine::is_truthy(&Value::Function("main".to_string())));
        assert!(Engine::is_truthy(&Value::Context("MainProgram".to_string())));
        assert!(!Engine::is_truthy(&Value::Void));
    }
    
    #[test]
    fn test_values_equal() {
        assert!(Engine::values_equal(&Value::Bool(true), &Value::Bool(true)));
        assert!(!Engine::values_equal(&Value::Bool(true), &Value::Bool(false)));
        assert!(Engine::values_equal(&Value::Int(42), &Value::Int(42)));
        assert!(!Engine::values_equal(&Value::Int(42), &Value::Int(43)));
        assert!(Engine::values_equal(&Value::Float(3.14), &Value::Float(3.14)));
        assert!(!Engine::values_equal(&Value::Float(3.14), &Value::Float(3.15)));
        assert!(Engine::values_equal(&Value::String("hello".to_string()), &Value::String("hello".to_string())));
        assert!(!Engine::values_equal(&Value::String("hello".to_string()), &Value::String("world".to_string())));
        assert!(Engine::values_equal(&Value::List(vec![Value::Int(42)]), &Value::List(vec![Value::Int(42)])));
        assert!(!Engine::values_equal(&Value::List(vec![Value::Int(42)]), &Value::List(vec![Value::Int(43)])));
        assert!(Engine::values_equal(&Value::Map(std::collections::HashMap::from([
            ("key".to_string(), Value::Int(42))
        ])), &Value::Map(std::collections::HashMap::from([
            ("key".to_string(), Value::Int(42))
        ]))));
        assert!(!Engine::values_equal(&Value::Map(std::collections::HashMap::from([
            ("key".to_string(), Value::Int(42))
        ])), &Value::Map(std::collections::HashMap::from([
            ("key".to_string(), Value::Int(43))
        ]))));
        assert!(Engine::values_equal(&Value::Vector(vec![1.0, 2.0, 3.0]), &Value::Vector(vec![1.0, 2.0, 3.0])));
        assert!(!Engine::values_equal(&Value::Vector(vec![1.0, 2.0, 3.0]), &Value::Vector(vec![1.0, 2.0, 4.0])));
        assert!(Engine::values_equal(&Value::Function("main".to_string()), &Value::Function("main".to_string())));
        assert!(!Engine::values_equal(&Value::Function("main".to_string()), &Value::Function("other".to_string())));
        assert!(Engine::values_equal(&Value::Context("MainProgram".to_string()), &Value::Context("MainProgram".to_string())));
        assert!(!Engine::values_equal(&Value::Context("MainProgram".to_string()), &Value::Context("OtherProgram".to_string())));
        assert!(Engine::values_equal(&Value::Void, &Value::Void));
        assert!(!Engine::values_equal(&Value::Int(42), &Value::String("42".to_string())));
    }
}
