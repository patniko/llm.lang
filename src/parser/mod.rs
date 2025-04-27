//! Parser module for LLM.lang
//!
//! This module provides the parser for the LLM.lang programming language,
//! which converts a stream of tokens into an abstract syntax tree (AST).

pub mod ast;
pub mod error;

use std::iter::Peekable;
use std::slice::Iter;

use crate::lexer::token::{Token, TokenKind};
use crate::utils::SourceLocation;

use self::ast::{Ast, Node, NodeKind, Expression, Statement, Type};
use self::error::{ParserError, ParserResult};

/// A parser for the LLM.lang language
pub struct Parser {
    /// The tokens to parse
    tokens: Vec<Token>,
    
    /// The current position in the token stream
    position: usize,
}

impl Parser {
    /// Create a new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
    
    /// Parse the tokens into an AST
    pub fn parse(&mut self) -> ParserResult<Ast> {
        // Reset the parser state
        self.position = 0;
        
        // Parse the program
        let program = self.parse_program()?;
        
        // Create the AST
        let ast = Ast {
            root: Box::new(program),
        };
        
        Ok(ast)
    }
    
    /// Parse a program
    fn parse_program(&mut self) -> ParserResult<Node> {
        // Create a program node
        let location = self.current_location();
        let mut program = Node {
            kind: NodeKind::Program,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Parse declarations and statements until we reach the end of the file
        while !self.is_at_end() {
            if self.match_keyword("context") {
                // Parse a context declaration
                let context = self.parse_context_declaration()?;
                program.children.push(Box::new(context));
            } else if self.match_keyword("fn") {
                // Parse a function declaration
                let function = self.parse_function_declaration()?;
                program.children.push(Box::new(function));
            } else {
                // Parse a statement
                let statement = self.parse_statement()?;
                program.children.push(Box::new(statement));
            }
        }
        
        Ok(program)
    }
    
    /// Parse a context declaration
    fn parse_context_declaration(&mut self) -> ParserResult<Node> {
        // We've already consumed the "context" keyword
        
        // Parse the context name
        let name = self.consume_identifier("Expected context name")?;
        
        // Parse the context body
        self.consume_delimiter("{", "Expected '{' after context name")?;
        
        // Create a context node
        let location = self.current_location();
        let mut context = Node {
            kind: NodeKind::Context,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the name attribute
        context.attributes.insert("name".to_string(), name.value);
        
        // Parse declarations until we reach the end of the context
        while !self.check_delimiter("}") && !self.is_at_end() {
            if self.match_keyword("fn") {
                // Parse a function declaration
                let function = self.parse_function_declaration()?;
                context.children.push(Box::new(function));
            } else if self.match_keyword("var") {
                // Parse a variable declaration
                let variable = self.parse_variable_declaration()?;
                context.children.push(Box::new(variable));
            } else {
                // Unexpected token
                let token = self.peek().unwrap();
                return Err(ParserError::new(
                    &format!("Unexpected token in context: {}", token.value),
                    token.location.clone(),
                ));
            }
        }
        
        // Consume the closing brace
        self.consume_delimiter("}", "Expected '}' after context body")?;
        
        Ok(context)
    }
    
    /// Parse a function declaration
    fn parse_function_declaration(&mut self) -> ParserResult<Node> {
        // We've already consumed the "fn" keyword
        
        // Parse the function name
        let name = self.consume_identifier("Expected function name")?;
        
        // Parse the parameter list
        self.consume_delimiter("(", "Expected '(' after function name")?;
        
        let mut parameters = Vec::new();
        
        if !self.check_delimiter(")") {
            // Parse the first parameter
            parameters.push(self.parse_parameter()?);
            
            // Parse additional parameters
            while self.match_delimiter(",") {
                parameters.push(self.parse_parameter()?);
            }
        }
        
        self.consume_delimiter(")", "Expected ')' after parameter list")?;
        
        // Parse the return type (if any)
        let mut return_type = None;
        
        if self.match_operator("->") {
            return_type = Some(self.parse_type()?);
        }
        
        // Parse the function body
        let body = self.parse_block()?;
        
        // Create a function node
        let location = self.current_location();
        let mut function = Node {
            kind: NodeKind::Function,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the name attribute
        function.attributes.insert("name".to_string(), name.value);
        
        // Add the parameters
        for parameter in parameters {
            function.children.push(Box::new(parameter));
        }
        
        // Add the return type (if any)
        if let Some(typ) = return_type {
            function.attributes.insert("return_type".to_string(), typ.to_string());
        }
        
        // Add the body
        function.children.push(Box::new(body));
        
        Ok(function)
    }
    
    /// Parse a parameter
    fn parse_parameter(&mut self) -> ParserResult<Node> {
        // Parse the parameter name
        let name = self.consume_identifier("Expected parameter name")?;
        
        // Parse the parameter type
        self.consume_delimiter(":", "Expected ':' after parameter name")?;
        let typ = self.parse_type()?;
        
        // Create a parameter node
        let location = self.current_location();
        let mut parameter = Node {
            kind: NodeKind::Parameter,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the name attribute
        parameter.attributes.insert("name".to_string(), name.value);
        
        // Add the type attribute
        parameter.attributes.insert("type".to_string(), typ.to_string());
        
        Ok(parameter)
    }
    
    /// Parse a type
    fn parse_type(&mut self) -> ParserResult<Type> {
        if self.match_keyword("Int") {
            Ok(Type::Int)
        } else if self.match_keyword("Float") {
            Ok(Type::Float)
        } else if self.match_keyword("String") {
            Ok(Type::String)
        } else if self.match_keyword("Bool") {
            Ok(Type::Bool)
        } else if self.match_keyword("List") {
            Ok(Type::List)
        } else if self.match_keyword("Map") {
            Ok(Type::Map)
        } else if self.match_keyword("Vector") {
            Ok(Type::Vector)
        } else if self.match_keyword("Context") {
            Ok(Type::Context)
        } else if self.check_token(TokenKind::SemanticType) {
            let token = self.advance().unwrap();
            Ok(Type::Semantic(token.value.clone()))
        } else {
            let token = self.peek().unwrap();
            Err(ParserError::new(
                &format!("Expected type, got {}", token.value),
                token.location.clone(),
            ))
        }
    }
    
    /// Parse a block
    fn parse_block(&mut self) -> ParserResult<Node> {
        // Consume the opening brace
        self.consume_delimiter("{", "Expected '{' at start of block")?;
        
        // Create a block node
        let location = self.current_location();
        let mut block = Node {
            kind: NodeKind::Block,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Parse statements until we reach the end of the block
        while !self.check_delimiter("}") && !self.is_at_end() {
            let statement = self.parse_statement()?;
            block.children.push(Box::new(statement));
        }
        
        // Consume the closing brace
        self.consume_delimiter("}", "Expected '}' at end of block")?;
        
        Ok(block)
    }
    
    /// Parse a statement
    fn parse_statement(&mut self) -> ParserResult<Node> {
        if self.match_keyword("var") {
            // Parse a variable declaration
            self.parse_variable_declaration()
        } else if self.match_keyword("if") {
            // Parse an if statement
            self.parse_if_statement()
        } else if self.match_keyword("when") {
            // Parse a when statement
            self.parse_when_statement()
        } else if self.match_keyword("for") {
            // Parse a for statement
            self.parse_for_statement()
        } else if self.match_keyword("return") {
            // Parse a return statement
            self.parse_return_statement()
        } else if self.match_keyword("with") {
            // Parse a with statement
            self.parse_with_statement()
        } else if self.match_keyword("within") {
            // Parse a within statement
            self.parse_within_statement()
        } else if self.match_keyword("intent") {
            // Parse an intent statement
            self.parse_intent_statement()
        } else if self.match_keyword("parallel") {
            // Parse a parallel statement
            self.parse_parallel_statement()
        } else if self.match_keyword("apply") {
            // Parse an apply statement
            self.parse_apply_statement()
        } else if self.match_token(TokenKind::Semantic) {
            // Parse a semantic statement
            self.parse_semantic_statement()
        } else if self.check_delimiter("{") {
            // Parse a block statement
            let block = self.parse_block()?;
            
            // Create a statement node
            let location = self.current_location();
            let mut statement = Node {
                kind: NodeKind::Statement,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the block as a child
            statement.children.push(Box::new(block));
            
            Ok(statement)
        } else {
            // Parse an expression statement
            let expression = self.parse_expression()?;
            
            // Consume the semicolon
            self.consume_delimiter(";", "Expected ';' after expression")?;
            
            // Create a statement node
            let location = self.current_location();
            let mut statement = Node {
                kind: NodeKind::Statement,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the expression as a child
            statement.children.push(Box::new(expression));
            
            Ok(statement)
        }
    }
    
    /// Parse a variable declaration
    fn parse_variable_declaration(&mut self) -> ParserResult<Node> {
        // We've already consumed the "var" keyword
        
        // Parse the variable name
        let name = self.consume_identifier("Expected variable name")?;
        
        // Parse the variable type (if any)
        let mut typ = None;
        
        if self.match_delimiter(":") {
            typ = Some(self.parse_type()?);
        }
        
        // Parse the initializer
        self.consume_operator("=", "Expected '=' after variable name")?;
        let initializer = self.parse_expression()?;
        
        // Consume the semicolon
        self.consume_delimiter(";", "Expected ';' after variable declaration")?;
        
        // Create a variable declaration node
        let location = self.current_location();
        let mut variable = Node {
            kind: NodeKind::Variable,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the name attribute
        variable.attributes.insert("name".to_string(), name.value);
        
        // Add the type attribute (if any)
        if let Some(t) = typ {
            variable.attributes.insert("type".to_string(), t.to_string());
        }
        
        // Add the initializer as a child
        variable.children.push(Box::new(initializer));
        
        Ok(variable)
    }
    
    /// Parse an if statement
    fn parse_if_statement(&mut self) -> ParserResult<Node> {
        // We've already consumed the "if" keyword
        
        // Parse the condition
        self.consume_delimiter("(", "Expected '(' after 'if'")?;
        let condition = self.parse_expression()?;
        self.consume_delimiter(")", "Expected ')' after condition")?;
        
        // Parse the then branch
        let then_branch = self.parse_block()?;
        
        // Parse the else branch (if any)
        let mut else_branch = None;
        
        if self.match_keyword("else") {
            if self.check_keyword("if") {
                // Parse an else-if statement
                else_branch = Some(self.parse_if_statement()?);
            } else {
                // Parse an else block
                else_branch = Some(self.parse_block()?);
            }
        }
        
        // Create an if statement node
        let location = self.current_location();
        let mut if_statement = Node {
            kind: NodeKind::If,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the condition as a child
        if_statement.children.push(Box::new(condition));
        
        // Add the then branch as a child
        if_statement.children.push(Box::new(then_branch));
        
        // Add the else branch as a child (if any)
        if let Some(branch) = else_branch {
            if_statement.children.push(Box::new(branch));
        }
        
        Ok(if_statement)
    }
    
    /// Parse a when statement
    fn parse_when_statement(&mut self) -> ParserResult<Node> {
        // We've already consumed the "when" keyword
        
        // Parse the expression
        self.consume_delimiter("(", "Expected '(' after 'when'")?;
        let expression = self.parse_expression()?;
        self.consume_delimiter(")", "Expected ')' after expression")?;
        
        // Parse the cases
        self.consume_delimiter("{", "Expected '{' after expression")?;
        
        // Create a when statement node
        let location = self.current_location();
        let mut when_statement = Node {
            kind: NodeKind::When,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the expression as a child
        when_statement.children.push(Box::new(expression));
        
        // Parse cases until we reach the end of the when statement
        while !self.check_delimiter("}") && !self.is_at_end() {
            // Parse a case
            let case = self.parse_when_case()?;
            when_statement.children.push(Box::new(case));
        }
        
        // Consume the closing brace
        self.consume_delimiter("}", "Expected '}' after cases")?;
        
        Ok(when_statement)
    }
    
    /// Parse a when case
    fn parse_when_case(&mut self) -> ParserResult<Node> {
        // Parse the case expression
        let expression = if self.match_keyword("otherwise") {
            // Create an otherwise node
            let location = self.current_location();
            Node {
                kind: NodeKind::Otherwise,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            }
        } else {
            // Parse a regular expression
            self.parse_expression()?
        };
        
        // Parse the arrow
        self.consume_operator("=>", "Expected '=>' after case expression")?;
        
        // Parse the case body
        let body = self.parse_block()?;
        
        // Create a case node
        let location = self.current_location();
        let mut case = Node {
            kind: NodeKind::Case,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the expression as a child
        case.children.push(Box::new(expression));
        
        // Add the body as a child
        case.children.push(Box::new(body));
        
        Ok(case)
    }
    
    /// Parse a for statement
    fn parse_for_statement(&mut self) -> ParserResult<Node> {
        // We've already consumed the "for" keyword
        
        // Parse the loop variable
        self.consume_delimiter("(", "Expected '(' after 'for'")?;
        let variable = self.consume_identifier("Expected loop variable")?;
        
        // Parse the "in" keyword
        self.consume_keyword("in", "Expected 'in' after loop variable")?;
        
        // Parse the collection expression
        let collection = self.parse_expression()?;
        
        // Parse the closing parenthesis
        self.consume_delimiter(")", "Expected ')' after collection expression")?;
        
        // Parse the loop body
        let body = self.parse_block()?;
        
        // Create a for statement node
        let location = self.current_location();
        let mut for_statement = Node {
            kind: NodeKind::For,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the variable attribute
        for_statement.attributes.insert("variable".to_string(), variable.value);
        
        // Add the collection as a child
        for_statement.children.push(Box::new(collection));
        
        // Add the body as a child
        for_statement.children.push(Box::new(body));
        
        Ok(for_statement)
    }
    
    /// Parse a return statement
    fn parse_return_statement(&mut self) -> ParserResult<Node> {
        // We've already consumed the "return" keyword
        
        // Parse the return value (if any)
        let value = if !self.check_delimiter(";") {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        // Consume the semicolon
        self.consume_delimiter(";", "Expected ';' after return statement")?;
        
        // Create a return statement node
        let location = self.current_location();
        let mut return_statement = Node {
            kind: NodeKind::Return,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the value as a child (if any)
        if let Some(v) = value {
            return_statement.children.push(Box::new(v));
        }
        
        Ok(return_statement)
    }
    
    /// Parse a with statement
    fn parse_with_statement(&mut self) -> ParserResult<Node> {
        // We've already consumed the "with" keyword
        
        // Parse the "context" keyword
        self.consume_keyword("context", "Expected 'context' after 'with'")?;
        
        // Parse the context name
        let name = self.consume_string("Expected context name")?;
        
        // Parse the body
        let body = self.parse_block()?;
        
        // Create a with statement node
        let location = self.current_location();
        let mut with_statement = Node {
            kind: NodeKind::With,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the name attribute
        with_statement.attributes.insert("name".to_string(), name.value);
        
        // Add the body as a child
        with_statement.children.push(Box::new(body));
        
        Ok(with_statement)
    }
    
    /// Parse a within statement
    fn parse_within_statement(&mut self) -> ParserResult<Node> {
        // We've already consumed the "within" keyword
        
        // Parse the context name
        let name = self.consume_string("Expected context name")?;
        
        // Parse the body
        let body = self.parse_block()?;
        
        // Create a within statement node
        let location = self.current_location();
        let mut within_statement = Node {
            kind: NodeKind::Within,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the name attribute
        within_statement.attributes.insert("name".to_string(), name.value);
        
        // Add the body as a child
        within_statement.children.push(Box::new(body));
        
        Ok(within_statement)
    }
    
    /// Parse an intent statement
    fn parse_intent_statement(&mut self) -> ParserResult<Node> {
        // We've already consumed the "intent" keyword
        
        // Parse the colon
        self.consume_delimiter(":", "Expected ':' after 'intent'")?;
        
        // Parse the intent expression
        let expression = self.parse_expression()?;
        
        // Consume the semicolon
        self.consume_delimiter(";", "Expected ';' after intent expression")?;
        
        // Create an intent statement node
        let location = self.current_location();
        let mut intent_statement = Node {
            kind: NodeKind::Intent,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the expression as a child
        intent_statement.children.push(Box::new(expression));
        
        Ok(intent_statement)
    }
    
    /// Parse a parallel statement
    fn parse_parallel_statement(&mut self) -> ParserResult<Node> {
        // We've already consumed the "parallel" keyword
        
        // Parse the paths
        self.consume_delimiter("{", "Expected '{' after 'parallel'")?;
        
        // Create a parallel statement node
        let location = self.current_location();
        let mut parallel_statement = Node {
            kind: NodeKind::Parallel,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Parse paths until we reach the end of the parallel statement
        while !self.check_delimiter("}") && !self.is_at_end() {
            // Parse a path
            let path = self.parse_parallel_path()?;
            parallel_statement.children.push(Box::new(path));
        }
        
        // Consume the closing brace
        self.consume_delimiter("}", "Expected '}' after paths")?;
        
        // Parse the selection strategy
        self.consume_keyword("select", "Expected 'select' after parallel paths")?;
        
        let strategy = if self.match_keyword("fastest") {
            "fastest"
        } else if self.match_keyword("best") {
            "best"
        } else if self.match_keyword("all") {
            "all"
        } else {
            let token = self.peek().unwrap();
            return Err(ParserError::new(
                &format!("Expected selection strategy, got {}", token.value),
                token.location.clone(),
            ));
        };
        
        // Add the strategy attribute
        parallel_statement.attributes.insert("strategy".to_string(), strategy.to_string());
        
        // Consume the semicolon
        self.consume_delimiter(";", "Expected ';' after selection strategy")?;
        
        Ok(parallel_statement)
    }
    
    /// Parse a parallel path
    fn parse_parallel_path(&mut self) -> ParserResult<Node> {
        // Parse the path name
        let name = self.consume_identifier("Expected path name")?;
        
        // Parse the colon
        self.consume_delimiter(":", "Expected ':' after path name")?;
        
        // Parse the path body
        let body = self.parse_block()?;
        
        // Create a path node
        let location = self.current_location();
        let mut path = Node {
            kind: NodeKind::Path,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the name attribute
        path.attributes.insert("name".to_string(), name.value);
        
        // Add the body as a child
        path.children.push(Box::new(body));
        
        Ok(path)
    }
    
    /// Parse an apply statement
    fn parse_apply_statement(&mut self) -> ParserResult<Node> {
        // We've already consumed the "apply" keyword
        
        // Parse the vector expression
        let vector = self.parse_expression()?;
        
        // Parse the "to" keyword
        self.consume_keyword("to", "Expected 'to' after vector expression")?;
        
        // Parse the body
        let body = self.parse_block()?;
        
        // Create an apply statement node
        let location = self.current_location();
        let mut apply_statement = Node {
            kind: NodeKind::Apply,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the vector as a child
        apply_statement.children.push(Box::new(vector));
        
        // Add the body as a child
        apply_statement.children.push(Box::new(body));
        
        Ok(apply_statement)
    }
    
    /// Parse a semantic statement
    fn parse_semantic_statement(&mut self) -> ParserResult<Node> {
        // We've already consumed the semantic token
        let token = self.previous().unwrap();
        
        // Create a semantic statement node
        let location = self.current_location();
        let mut semantic_statement = Node {
            kind: NodeKind::Semantic,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the token attribute
        semantic_statement.attributes.insert("token".to_string(), token.value.clone());
        
        // Check if it's a remember statement
        if token.value == "@remember" {
            // Parse the variable name
            let name = self.consume_identifier("Expected variable name after '@remember'")?;
            
            // Parse the equals sign
            self.consume_operator("=", "Expected '=' after variable name")?;
            
            // Parse the value expression
            let value = self.parse_expression()?;
            
            // Consume the semicolon
            self.consume_delimiter(";", "Expected ';' after remember statement")?;
            
            // Add the name attribute
            semantic_statement.attributes.insert("name".to_string(), name.value);
            
            // Add the value as a child
            semantic_statement.children.push(Box::new(value));
        } else if token.value == "@recall" {
            // Check if there's a specific key
            if self.match_delimiter("(") {
                // Parse the key
                let key = self.consume_string("Expected key in '@recall(...)'")?;
                
                // Parse the closing parenthesis
                self.consume_delimiter(")", "Expected ')' after key")?;
                
                // Add the key attribute
                semantic_statement.attributes.insert("key".to_string(), key.value);
            }
            
            // Consume the semicolon
            self.consume_delimiter(";", "Expected ';' after recall statement")?;
        } else {
            // Unknown semantic token
            return Err(ParserError::new(
                &format!("Unknown semantic token: {}", token.value),
                token.location.clone(),
            ));
        }
        
        Ok(semantic_statement)
    }
    
    /// Parse an expression
    fn parse_expression(&mut self) -> ParserResult<Node> {
        self.parse_assignment()
    }
    
    /// Parse an assignment expression
    fn parse_assignment(&mut self) -> ParserResult<Node> {
        let expr = self.parse_logical_or()?;
        
        if self.match_operator("=") {
            // Parse the right-hand side
            let value = self.parse_assignment()?;
            
            // Create an assignment node
            let location = self.current_location();
            let mut assignment = Node {
                kind: NodeKind::Assignment,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the left-hand side as a child
            assignment.children.push(Box::new(expr));
            
            // Add the right-hand side as a child
            assignment.children.push(Box::new(value));
            
            Ok(assignment)
        } else if self.match_operator("+=") || self.match_operator("-=") || self.match_operator("*=") || self.match_operator("/=") || self.match_operator("%=") {
            // Get the operator
            let token = self.previous().unwrap();
            let operator = token.value[0..1].to_string(); // Extract the operator without the '='
            
            // Parse the right-hand side
            let value = self.parse_assignment()?;
            
            // Create a binary expression for the operation
            let location = self.current_location();
            let mut operation = Node {
                kind: NodeKind::Binary,
                location: location.clone(),
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the operator attribute
            operation.attributes.insert("operator".to_string(), operator.to_string());
            
            // Add the left-hand side as a child
            operation.children.push(Box::new(expr.clone()));
            
            // Add the right-hand side as a child
            operation.children.push(Box::new(value));
            
            // Create an assignment node
            let mut assignment = Node {
                kind: NodeKind::Assignment,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the left-hand side as a child
            assignment.children.push(Box::new(expr));
            
            // Add the operation as a child
            assignment.children.push(Box::new(operation));
            
            Ok(assignment)
        } else {
            Ok(expr)
        }
    }
    
    /// Parse a logical OR expression
    fn parse_logical_or(&mut self) -> ParserResult<Node> {
        let mut expr = self.parse_logical_and()?;
        
        while self.match_keyword("or") || self.match_operator("||") {
            // Get the operator
            let token = self.previous().unwrap();
            
            // Parse the right-hand side
            let right = self.parse_logical_and()?;
            
            // Create a binary expression
            let location = self.current_location();
            let mut binary = Node {
                kind: NodeKind::Binary,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the operator attribute
            binary.attributes.insert("operator".to_string(), "or".to_string());
            
            // Add the left-hand side as a child
            binary.children.push(Box::new(expr));
            
            // Add the right-hand side as a child
            binary.children.push(Box::new(right));
            
            // Update the expression
            expr = binary;
        }
        
        Ok(expr)
    }
    
    /// Parse a logical AND expression
    fn parse_logical_and(&mut self) -> ParserResult<Node> {
        let mut expr = self.parse_equality()?;
        
        while self.match_keyword("and") || self.match_operator("&&") {
            // Get the operator
            let token = self.previous().unwrap();
            
            // Parse the right-hand side
            let right = self.parse_equality()?;
            
            // Create a binary expression
            let location = self.current_location();
            let mut binary = Node {
                kind: NodeKind::Binary,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the operator attribute
            binary.attributes.insert("operator".to_string(), "and".to_string());
            
            // Add the left-hand side as a child
            binary.children.push(Box::new(expr));
            
            // Add the right-hand side as a child
            binary.children.push(Box::new(right));
            
            // Update the expression
            expr = binary;
        }
        
        Ok(expr)
    }
    
    /// Parse an equality expression
    fn parse_equality(&mut self) -> ParserResult<Node> {
        let mut expr = self.parse_comparison()?;
        
        while self.match_operator("==") || self.match_operator("!=") {
            // Get the operator
            let token_value = self.previous().unwrap().value.clone();
            
            // Parse the right-hand side
            let right = self.parse_comparison()?;
            
            // Create a binary expression
            let location = self.current_location();
            let mut binary = Node {
                kind: NodeKind::Binary,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the operator attribute
            binary.attributes.insert("operator".to_string(), token_value);
            
            // Add the left-hand side as a child
            binary.children.push(Box::new(expr));
            
            // Add the right-hand side as a child
            binary.children.push(Box::new(right));
            
            // Update the expression
            expr = binary;
        }
        
        Ok(expr)
    }
    
    /// Parse a comparison expression
    fn parse_comparison(&mut self) -> ParserResult<Node> {
        let mut expr = self.parse_term()?;
        
        while self.match_operator("<") || self.match_operator(">") || self.match_operator("<=") || self.match_operator(">=") {
            // Get the operator
            let token_value = self.previous().unwrap().value.clone();
            
            // Parse the right-hand side
            let right = self.parse_term()?;
            
            // Create a binary expression
            let location = self.current_location();
            let mut binary = Node {
                kind: NodeKind::Binary,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the operator attribute
            binary.attributes.insert("operator".to_string(), token_value);
            
            // Add the left-hand side as a child
            binary.children.push(Box::new(expr));
            
            // Add the right-hand side as a child
            binary.children.push(Box::new(right));
            
            // Update the expression
            expr = binary;
        }
        
        Ok(expr)
    }
    
    /// Parse a term expression
    fn parse_term(&mut self) -> ParserResult<Node> {
        let mut expr = self.parse_factor()?;
        
        while self.match_operator("+") || self.match_operator("-") {
            // Get the operator
            let token_value = self.previous().unwrap().value.clone();
            
            // Parse the right-hand side
            let right = self.parse_factor()?;
            
            // Create a binary expression
            let location = self.current_location();
            let mut binary = Node {
                kind: NodeKind::Binary,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the operator attribute
            binary.attributes.insert("operator".to_string(), token_value);
            
            // Add the left-hand side as a child
            binary.children.push(Box::new(expr));
            
            // Add the right-hand side as a child
            binary.children.push(Box::new(right));
            
            // Update the expression
            expr = binary;
        }
        
        Ok(expr)
    }
    
    /// Parse a factor expression
    fn parse_factor(&mut self) -> ParserResult<Node> {
        let mut expr = self.parse_unary()?;
        
        while self.match_operator("*") || self.match_operator("/") || self.match_operator("%") {
            // Get the operator
            let token_value = self.previous().unwrap().value.clone();
            
            // Parse the right-hand side
            let right = self.parse_unary()?;
            
            // Create a binary expression
            let location = self.current_location();
            let mut binary = Node {
                kind: NodeKind::Binary,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the operator attribute
            binary.attributes.insert("operator".to_string(), token_value);
            
            // Add the left-hand side as a child
            binary.children.push(Box::new(expr));
            
            // Add the right-hand side as a child
            binary.children.push(Box::new(right));
            
            // Update the expression
            expr = binary;
        }
        
        Ok(expr)
    }
    
    /// Parse a unary expression
    fn parse_unary(&mut self) -> ParserResult<Node> {
        if self.match_operator("-") || self.match_operator("!") || self.match_keyword("not") {
            // Get the operator
            let token = self.previous().unwrap();
            let operator_str = if token.value == "not" { "!".to_string() } else { token.value.clone() };
            
            // Parse the operand
            let operand = self.parse_unary()?;
            
            // Create a unary expression
            let location = self.current_location();
            let mut unary = Node {
                kind: NodeKind::Unary,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the operator attribute
            unary.attributes.insert("operator".to_string(), operator_str);
            
            // Add the operand as a child
            unary.children.push(Box::new(operand));
            
            Ok(unary)
        } else {
            self.parse_call()
        }
    }
    
    /// Parse a call expression
    fn parse_call(&mut self) -> ParserResult<Node> {
        let mut expr = self.parse_primary()?;
        
        loop {
            if self.match_delimiter("(") {
                // Parse a function call
                expr = self.finish_call(expr)?;
            } else if self.match_delimiter(".") {
                // Parse a property access
                let name = self.consume_identifier("Expected property name after '.'")?;
                
                // Create a property access node
                let location = self.current_location();
                let mut property = Node {
                    kind: NodeKind::Binary,
                    location,
                    children: Vec::new(),
                    attributes: std::collections::HashMap::new(),
                };
                
                // Add the operator attribute
                property.attributes.insert("operator".to_string(), ".".to_string());
                
                // Add the object as a child
                property.children.push(Box::new(expr));
                
                // Add the property name as a child
                let name_node = Node {
                    kind: NodeKind::Identifier,
                    location: name.location.clone(),
                    children: Vec::new(),
                    attributes: std::collections::HashMap::new(),
                };
                
                // Add the name attribute
                property.attributes.insert("name".to_string(), name.value);
                
                // Update the expression
                expr = property;
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    /// Finish parsing a call expression
    fn finish_call(&mut self, callee: Node) -> ParserResult<Node> {
        // Parse the arguments
        let mut arguments = Vec::new();
        
        if !self.check_delimiter(")") {
            // Parse the first argument
            arguments.push(self.parse_expression()?);
            
            // Parse additional arguments
            while self.match_delimiter(",") {
                arguments.push(self.parse_expression()?);
            }
        }
        
        // Parse the closing parenthesis
        self.consume_delimiter(")", "Expected ')' after arguments")?;
        
        // Create a call node
        let location = self.current_location();
        let mut call = Node {
            kind: NodeKind::Call,
            location,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        };
        
        // Add the callee as a child
        call.children.push(Box::new(callee));
        
        // Add the arguments as children
        for argument in arguments {
            call.children.push(Box::new(argument));
        }
        
        Ok(call)
    }
    
    /// Parse a primary expression
    fn parse_primary(&mut self) -> ParserResult<Node> {
        if self.match_keyword("true") || self.match_keyword("false") {
            // Parse a boolean literal
            let token = self.previous().unwrap();
            let value = token.value == "true";
            
            // Create a literal node
            let location = self.current_location();
            let mut literal = Node {
                kind: NodeKind::Literal,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the type attribute
            literal.attributes.insert("type".to_string(), "Bool".to_string());
            
            // Add the value attribute
            literal.attributes.insert("value".to_string(), value.to_string());
            
            Ok(literal)
        } else if self.match_keyword("null") {
            // Parse a null literal
            
            // Create a literal node
            let location = self.current_location();
            let mut literal = Node {
                kind: NodeKind::Literal,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the type attribute
            literal.attributes.insert("type".to_string(), "Null".to_string());
            
            Ok(literal)
        } else if self.match_token(TokenKind::IntLiteral) {
            // Parse an integer literal
            let token = self.previous().unwrap();
            
            // Create a literal node
            let location = self.current_location();
            let mut literal = Node {
                kind: NodeKind::Literal,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the type attribute
            literal.attributes.insert("type".to_string(), "Int".to_string());
            
            // Add the value attribute
            literal.attributes.insert("value".to_string(), token.value.clone());
            
            Ok(literal)
        } else if self.match_token(TokenKind::FloatLiteral) {
            // Parse a floating-point literal
            let token = self.previous().unwrap();
            
            // Create a literal node
            let location = self.current_location();
            let mut literal = Node {
                kind: NodeKind::Literal,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the type attribute
            literal.attributes.insert("type".to_string(), "Float".to_string());
            
            // Add the value attribute
            literal.attributes.insert("value".to_string(), token.value.clone());
            
            Ok(literal)
        } else if self.match_token(TokenKind::StringLiteral) {
            // Parse a string literal
            let token = self.previous().unwrap();
            
            // Create a literal node
            let location = self.current_location();
            let mut literal = Node {
                kind: NodeKind::Literal,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the type attribute
            literal.attributes.insert("type".to_string(), "String".to_string());
            
            // Add the value attribute
            literal.attributes.insert("value".to_string(), token.value.clone());
            
            Ok(literal)
        } else if self.match_token(TokenKind::NaturalLanguage) {
            // Parse a natural language expression
            let token = self.previous().unwrap();
            
            // Create a natural language node
            let location = self.current_location();
            let mut natural = Node {
                kind: NodeKind::NaturalLanguage,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the value attribute
            natural.attributes.insert("value".to_string(), token.value.clone());
            
            Ok(natural)
        } else if self.match_token(TokenKind::Identifier) {
            // Parse an identifier
            let token = self.previous().unwrap();
            
            // Create an identifier node
            let location = self.current_location();
            let mut identifier = Node {
                kind: NodeKind::Identifier,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the name attribute
            identifier.attributes.insert("name".to_string(), token.value.clone());
            
            Ok(identifier)
        } else if self.match_delimiter("(") {
            // Parse a grouping expression
            let expr = self.parse_expression()?;
            
            // Parse the closing parenthesis
            self.consume_delimiter(")", "Expected ')' after expression")?;
            
            // Create a grouping node
            let location = self.current_location();
            let mut grouping = Node {
                kind: NodeKind::Grouping,
                location,
                children: Vec::new(),
                attributes: std::collections::HashMap::new(),
            };
            
            // Add the expression as a child
            grouping.children.push(Box::new(expr));
            
            Ok(grouping)
        } else {
            // Unexpected token
            let token = self.peek().unwrap();
            Err(ParserError::new(
                &format!("Unexpected token: {}", token.value),
                token.location.clone(),
            ))
        }
    }
    
    /// Check if the current token is of the given type
    fn check_token(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().unwrap().kind == kind
        }
    }
    
    /// Check if the current token is a keyword with the given value
    fn check_keyword(&self, value: &str) -> bool {
        if self.is_at_end() {
            false
        } else {
            let token = self.peek().unwrap();
            token.kind == TokenKind::Keyword && token.value == value
        }
    }
    
    /// Check if the current token is a delimiter with the given value
    fn check_delimiter(&self, value: &str) -> bool {
        if self.is_at_end() {
            false
        } else {
            let token = self.peek().unwrap();
            token.kind == TokenKind::Delimiter && token.value == value
        }
    }
    
    /// Check if the current token is an operator with the given value
    fn check_operator(&self, value: &str) -> bool {
        if self.is_at_end() {
            false
        } else {
            let token = self.peek().unwrap();
            token.kind == TokenKind::Operator && token.value == value
        }
    }
    
    /// Consume the current token if it is of the given type
    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check_token(kind) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    /// Consume the current token if it is a keyword with the given value
    fn match_keyword(&mut self, value: &str) -> bool {
        if self.check_keyword(value) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    /// Consume the current token if it is a delimiter with the given value
    fn match_delimiter(&mut self, value: &str) -> bool {
        if self.check_delimiter(value) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    /// Consume the current token if it is an operator with the given value
    fn match_operator(&mut self, value: &str) -> bool {
        if self.check_operator(value) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    /// Consume the current token if it is an identifier
    fn consume_identifier(&mut self, error_message: &str) -> ParserResult<Token> {
        if self.check_token(TokenKind::Identifier) {
            Ok(self.advance().unwrap().clone())
        } else {
            let token = self.peek().unwrap();
            Err(ParserError::new(error_message, token.location.clone()))
        }
    }
    
    /// Consume the current token if it is a keyword with the given value
    fn consume_keyword(&mut self, value: &str, error_message: &str) -> ParserResult<Token> {
        if self.check_keyword(value) {
            Ok(self.advance().unwrap().clone())
        } else {
            let token = self.peek().unwrap();
            Err(ParserError::new(error_message, token.location.clone()))
        }
    }
    
    /// Consume the current token if it is a delimiter with the given value
    fn consume_delimiter(&mut self, value: &str, error_message: &str) -> ParserResult<Token> {
        if self.check_delimiter(value) {
            Ok(self.advance().unwrap().clone())
        } else {
            let token = self.peek().unwrap();
            Err(ParserError::new(error_message, token.location.clone()))
        }
    }
    
    /// Consume the current token if it is an operator with the given value
    fn consume_operator(&mut self, value: &str, error_message: &str) -> ParserResult<Token> {
        if self.check_operator(value) {
            Ok(self.advance().unwrap().clone())
        } else {
            let token = self.peek().unwrap();
            Err(ParserError::new(error_message, token.location.clone()))
        }
    }
    
    /// Consume the current token if it is a string literal
    fn consume_string(&mut self, error_message: &str) -> ParserResult<Token> {
        if self.check_token(TokenKind::StringLiteral) {
            Ok(self.advance().unwrap().clone())
        } else {
            let token = self.peek().unwrap();
            Err(ParserError::new(error_message, token.location.clone()))
        }
    }
    
    /// Advance to the next token
    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.position += 1;
        }
        self.previous()
    }
    
    /// Get the current token
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }
    
    /// Get the previous token
    fn previous(&self) -> Option<&Token> {
        if self.position == 0 {
            None
        } else {
            self.tokens.get(self.position - 1)
        }
    }
    
    /// Check if we've reached the end of the token stream
    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len() || self.peek().unwrap().kind == TokenKind::Eof
    }
    
    /// Get the current source location
    fn current_location(&self) -> SourceLocation {
        if let Some(token) = self.peek() {
            token.location.clone()
        } else if let Some(token) = self.previous() {
            token.location.clone()
        } else {
            SourceLocation::new(0, 0, 0, 0, "")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenKind};
    
    #[test]
    fn test_parse_empty_program() {
        let tokens = vec![
            Token {
                kind: TokenKind::Eof,
                value: "".to_string(),
                location: SourceLocation::new(1, 1, 1, 1, "test.llm"),
            },
        ];
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.root.kind, NodeKind::Program);
        assert_eq!(ast.root.children.len(), 0);
    }
    
    #[test]
    fn test_parse_context_declaration() {
        let tokens = vec![
            Token {
                kind: TokenKind::Keyword,
                value: "context".to_string(),
                location: SourceLocation::new(1, 1, 1, 8, "test.llm"),
            },
            Token {
                kind: TokenKind::Identifier,
                value: "MainProgram".to_string(),
                location: SourceLocation::new(1, 9, 1, 20, "test.llm"),
            },
            Token {
                kind: TokenKind::Delimiter,
                value: "{".to_string(),
                location: SourceLocation::new(1, 21, 1, 22, "test.llm"),
            },
            Token {
                kind: TokenKind::Delimiter,
                value: "}".to_string(),
                location: SourceLocation::new(1, 23, 1, 24, "test.llm"),
            },
            Token {
                kind: TokenKind::Eof,
                value: "".to_string(),
                location: SourceLocation::new(1, 25, 1, 25, "test.llm"),
            },
        ];
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.root.kind, NodeKind::Program);
        assert_eq!(ast.root.children.len(), 1);
        
        let context = ast.root.get_child(0).unwrap();
        assert_eq!(context.kind, NodeKind::Context);
        assert_eq!(context.get_attribute("name").unwrap(), "MainProgram");
    }
    
    #[test]
    fn test_parse_function_declaration() {
        let tokens = vec![
            Token {
                kind: TokenKind::Keyword,
                value: "fn".to_string(),
                location: SourceLocation::new(1, 1, 1, 3, "test.llm"),
            },
            Token {
                kind: TokenKind::Identifier,
                value: "main".to_string(),
                location: SourceLocation::new(1, 4, 1, 8, "test.llm"),
            },
            Token {
                kind: TokenKind::Delimiter,
                value: "(".to_string(),
                location: SourceLocation::new(1, 8, 1, 9, "test.llm"),
            },
            Token {
                kind: TokenKind::Delimiter,
                value: ")".to_string(),
                location: SourceLocation::new(1, 9, 1, 10, "test.llm"),
            },
            Token {
                kind: TokenKind::Delimiter,
                value: "{".to_string(),
                location: SourceLocation::new(1, 11, 1, 12, "test.llm"),
            },
            Token {
                kind: TokenKind::Delimiter,
                value: "}".to_string(),
                location: SourceLocation::new(1, 13, 1, 14, "test.llm"),
            },
            Token {
                kind: TokenKind::Eof,
                value: "".to_string(),
                location: SourceLocation::new(1, 15, 1, 15, "test.llm"),
            },
        ];
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.root.kind, NodeKind::Program);
        assert_eq!(ast.root.children.len(), 1);
        
        let function = ast.root.get_child(0).unwrap();
        assert_eq!(function.kind, NodeKind::Function);
        assert_eq!(function.get_attribute("name").unwrap(), "main");
        assert_eq!(function.child_count(), 1); // Just the body
        
        let body = function.get_child(0).unwrap();
        assert_eq!(body.kind, NodeKind::Block);
    }
    
    #[test]
    fn test_parse_variable_declaration() {
        let tokens = vec![
            Token {
                kind: TokenKind::Keyword,
                value: "var".to_string(),
                location: SourceLocation::new(1, 1, 1, 4, "test.llm"),
            },
            Token {
                kind: TokenKind::Identifier,
                value: "x".to_string(),
                location: SourceLocation::new(1, 5, 1, 6, "test.llm"),
            },
            Token {
                kind: TokenKind::Operator,
                value: "=".to_string(),
                location: SourceLocation::new(1, 7, 1, 8, "test.llm"),
            },
            Token {
                kind: TokenKind::IntLiteral,
                value: "42".to_string(),
                location: SourceLocation::new(1, 9, 1, 11, "test.llm"),
            },
            Token {
                kind: TokenKind::Delimiter,
                value: ";".to_string(),
                location: SourceLocation::new(1, 11, 1, 12, "test.llm"),
            },
            Token {
                kind: TokenKind::Eof,
                value: "".to_string(),
                location: SourceLocation::new(1, 13, 1, 13, "test.llm"),
            },
        ];
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.root.kind, NodeKind::Program);
        assert_eq!(ast.root.children.len(), 1);
        
        let variable = ast.root.get_child(0).unwrap();
        assert_eq!(variable.kind, NodeKind::Variable);
        assert_eq!(variable.get_attribute("name").unwrap(), "x");
        assert_eq!(variable.child_count(), 1);
        
        let initializer = variable.get_child(0).unwrap();
        assert_eq!(initializer.kind, NodeKind::Literal);
        assert_eq!(initializer.get_attribute("type").unwrap(), "Int");
        assert_eq!(initializer.get_attribute("value").unwrap(), "42");
    }
}
