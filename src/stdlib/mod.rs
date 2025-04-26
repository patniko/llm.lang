//! Standard library module for LLM.lang
//!
//! This module provides the standard library for the LLM.lang programming language,
//! including core functions, collection functions, string functions, vector functions,
//! and context functions.

use std::collections::HashMap;
use crate::Value;
use crate::runtime::error::RuntimeError;
use crate::utils::SourceLocation;

/// The standard library for the LLM.lang language
pub struct StdLib {
    /// The functions in the standard library
    functions: HashMap<String, StdLibFunction>,
}

/// A standard library function
type StdLibFunction = fn(Vec<Value>) -> Result<Value, RuntimeError>;

impl StdLib {
    /// Create a new standard library
    pub fn new() -> Self {
        let mut stdlib = Self {
            functions: HashMap::new(),
        };
        
        // Register core functions
        stdlib.register_function("print", Self::print);
        stdlib.register_function("toString", Self::to_string);
        stdlib.register_function("parseInt", Self::parse_int);
        stdlib.register_function("parseFloat", Self::parse_float);
        
        // Register collection functions
        stdlib.register_function("length", Self::length);
        stdlib.register_function("isEmpty", Self::is_empty);
        stdlib.register_function("contains", Self::contains);
        stdlib.register_function("map", Self::map);
        stdlib.register_function("filter", Self::filter);
        stdlib.register_function("reduce", Self::reduce);
        
        // Register string functions
        stdlib.register_function("substring", Self::substring);
        stdlib.register_function("indexOf", Self::index_of);
        stdlib.register_function("toLowerCase", Self::to_lower_case);
        stdlib.register_function("toUpperCase", Self::to_upper_case);
        stdlib.register_function("trim", Self::trim);
        
        // Register vector functions
        stdlib.register_function("embed", Self::embed);
        stdlib.register_function("similarity", Self::similarity);
        stdlib.register_function("nearest", Self::nearest);
        
        // Register context functions
        stdlib.register_function("currentContext", Self::current_context);
        stdlib.register_function("switchContext", Self::switch_context);
        stdlib.register_function("mergeContexts", Self::merge_contexts);
        
        stdlib
    }
    
    /// Register a function in the standard library
    pub fn register_function(&mut self, name: &str, function: StdLibFunction) {
        self.functions.insert(name.to_string(), function);
    }
    
    /// Get a function from the standard library
    pub fn get_function(&self, name: &str) -> Option<&StdLibFunction> {
        self.functions.get(name)
    }
    
    /// Call a function from the standard library
    pub fn call_function(&self, name: &str, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if let Some(function) = self.functions.get(name) {
            function(arguments)
        } else {
            Err(RuntimeError::new(
                &format!("Unknown function: '{}'", name),
                SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Print a value to the console
    fn print(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.is_empty() {
            return Err(RuntimeError::new(
                "print() requires at least one argument",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        for arg in arguments {
            println!("{}", Self::value_to_string(&arg));
        }
        
        Ok(Value::Void)
    }
    
    /// Convert a value to a string
    fn to_string(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != 1 {
            return Err(RuntimeError::new(
                "toString() requires exactly one argument",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        Ok(Value::String(Self::value_to_string(&arguments[0])))
    }
    
    /// Parse a string as an integer
    fn parse_int(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != 1 {
            return Err(RuntimeError::new(
                "parseInt() requires exactly one argument",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let arg = &arguments[0];
        
        if let Value::String(s) = arg {
            match s.parse::<i64>() {
                Ok(i) => Ok(Value::Int(i)),
                Err(_) => Err(RuntimeError::new(
                    &format!("Cannot parse '{}' as an integer", s),
                    SourceLocation::new(0, 0, 0, 0, ""),
                )),
            }
        } else {
            Err(RuntimeError::new(
                &format!("Cannot parse {:?} as an integer", arg),
                SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Parse a string as a floating-point number
    fn parse_float(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != 1 {
            return Err(RuntimeError::new(
                "parseFloat() requires exactly one argument",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let arg = &arguments[0];
        
        if let Value::String(s) = arg {
            match s.parse::<f64>() {
                Ok(f) => Ok(Value::Float(f)),
                Err(_) => Err(RuntimeError::new(
                    &format!("Cannot parse '{}' as a floating-point number", s),
                    SourceLocation::new(0, 0, 0, 0, ""),
                )),
            }
        } else {
            Err(RuntimeError::new(
                &format!("Cannot parse {:?} as a floating-point number", arg),
                SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Get the length of a collection
    fn length(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != 1 {
            return Err(RuntimeError::new(
                "length() requires exactly one argument",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let arg = &arguments[0];
        
        match arg {
            Value::String(s) => Ok(Value::Int(s.len() as i64)),
            Value::List(list) => Ok(Value::Int(list.len() as i64)),
            Value::Map(map) => Ok(Value::Int(map.len() as i64)),
            _ => Err(RuntimeError::new(
                &format!("Cannot get length of {:?}", arg),
                SourceLocation::new(0, 0, 0, 0, ""),
            )),
        }
    }
    
    /// Check if a collection is empty
    fn is_empty(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != 1 {
            return Err(RuntimeError::new(
                "isEmpty() requires exactly one argument",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let arg = &arguments[0];
        
        match arg {
            Value::String(s) => Ok(Value::Bool(s.is_empty())),
            Value::List(list) => Ok(Value::Bool(list.is_empty())),
            Value::Map(map) => Ok(Value::Bool(map.is_empty())),
            _ => Err(RuntimeError::new(
                &format!("Cannot check if {:?} is empty", arg),
                SourceLocation::new(0, 0, 0, 0, ""),
            )),
        }
    }
    
    /// Check if a collection contains a value
    fn contains(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != 2 {
            return Err(RuntimeError::new(
                "contains() requires exactly two arguments",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let collection = &arguments[0];
        let value = &arguments[1];
        
        match collection {
            Value::String(s) => {
                if let Value::String(v) = value {
                    Ok(Value::Bool(s.contains(v)))
                } else {
                    Err(RuntimeError::new(
                        &format!("Cannot check if string contains {:?}", value),
                        SourceLocation::new(0, 0, 0, 0, ""),
                    ))
                }
            }
            Value::List(list) => {
                Ok(Value::Bool(list.contains(value)))
            }
            Value::Map(map) => {
                if let Value::String(key) = value {
                    Ok(Value::Bool(map.contains_key(key)))
                } else {
                    Err(RuntimeError::new(
                        &format!("Cannot check if map contains key {:?}", value),
                        SourceLocation::new(0, 0, 0, 0, ""),
                    ))
                }
            }
            _ => Err(RuntimeError::new(
                &format!("Cannot check if {:?} contains a value", collection),
                SourceLocation::new(0, 0, 0, 0, ""),
            )),
        }
    }
    
    /// Apply a function to each element of a collection
    fn map(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        // This is a placeholder implementation
        // In a real implementation, this would apply a function to each element
        Err(RuntimeError::new(
            "map() is not implemented yet",
            SourceLocation::new(0, 0, 0, 0, ""),
        ))
    }
    
    /// Filter a collection based on a predicate
    fn filter(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        // This is a placeholder implementation
        // In a real implementation, this would filter a collection
        Err(RuntimeError::new(
            "filter() is not implemented yet",
            SourceLocation::new(0, 0, 0, 0, ""),
        ))
    }
    
    /// Reduce a collection to a single value
    fn reduce(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        // This is a placeholder implementation
        // In a real implementation, this would reduce a collection
        Err(RuntimeError::new(
            "reduce() is not implemented yet",
            SourceLocation::new(0, 0, 0, 0, ""),
        ))
    }
    
    /// Get a substring of a string
    fn substring(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() < 2 || arguments.len() > 3 {
            return Err(RuntimeError::new(
                "substring() requires two or three arguments",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let string = &arguments[0];
        let start = &arguments[1];
        let end = if arguments.len() == 3 { Some(&arguments[2]) } else { None };
        
        if let Value::String(s) = string {
            if let Value::Int(start_idx) = start {
                let start_idx = *start_idx as usize;
                
                if start_idx > s.len() {
                    return Err(RuntimeError::new(
                        &format!("Start index {} is out of bounds for string of length {}", start_idx, s.len()),
                        SourceLocation::new(0, 0, 0, 0, ""),
                    ));
                }
                
                if let Some(Value::Int(end_idx)) = end {
                    let end_idx = *end_idx as usize;
                    
                    if end_idx > s.len() {
                        return Err(RuntimeError::new(
                            &format!("End index {} is out of bounds for string of length {}", end_idx, s.len()),
                            SourceLocation::new(0, 0, 0, 0, ""),
                        ));
                    }
                    
                    if end_idx < start_idx {
                        return Err(RuntimeError::new(
                            &format!("End index {} is less than start index {}", end_idx, start_idx),
                            SourceLocation::new(0, 0, 0, 0, ""),
                        ));
                    }
                    
                    Ok(Value::String(s[start_idx..end_idx].to_string()))
                } else {
                    Ok(Value::String(s[start_idx..].to_string()))
                }
            } else {
                Err(RuntimeError::new(
                    &format!("Start index must be an integer, got {:?}", start),
                    SourceLocation::new(0, 0, 0, 0, ""),
                ))
            }
        } else {
            Err(RuntimeError::new(
                &format!("Cannot get substring of {:?}", string),
                SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Find the index of a substring
    fn index_of(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != 2 {
            return Err(RuntimeError::new(
                "indexOf() requires exactly two arguments",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let string = &arguments[0];
        let substring = &arguments[1];
        
        if let Value::String(s) = string {
            if let Value::String(sub) = substring {
                match s.find(sub) {
                    Some(idx) => Ok(Value::Int(idx as i64)),
                    None => Ok(Value::Int(-1)),
                }
            } else {
                Err(RuntimeError::new(
                    &format!("Substring must be a string, got {:?}", substring),
                    SourceLocation::new(0, 0, 0, 0, ""),
                ))
            }
        } else {
            Err(RuntimeError::new(
                &format!("Cannot find index in {:?}", string),
                SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Convert a string to lowercase
    fn to_lower_case(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != 1 {
            return Err(RuntimeError::new(
                "toLowerCase() requires exactly one argument",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let string = &arguments[0];
        
        if let Value::String(s) = string {
            Ok(Value::String(s.to_lowercase()))
        } else {
            Err(RuntimeError::new(
                &format!("Cannot convert {:?} to lowercase", string),
                SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Convert a string to uppercase
    fn to_upper_case(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != 1 {
            return Err(RuntimeError::new(
                "toUpperCase() requires exactly one argument",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let string = &arguments[0];
        
        if let Value::String(s) = string {
            Ok(Value::String(s.to_uppercase()))
        } else {
            Err(RuntimeError::new(
                &format!("Cannot convert {:?} to uppercase", string),
                SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Remove whitespace from the beginning and end of a string
    fn trim(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != 1 {
            return Err(RuntimeError::new(
                "trim() requires exactly one argument",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let string = &arguments[0];
        
        if let Value::String(s) = string {
            Ok(Value::String(s.trim().to_string()))
        } else {
            Err(RuntimeError::new(
                &format!("Cannot trim {:?}", string),
                SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Convert text to a semantic vector embedding
    fn embed(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != 1 {
            return Err(RuntimeError::new(
                "embed() requires exactly one argument",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let text = &arguments[0];
        
        if let Value::String(s) = text {
            // This is a placeholder implementation
            // In a real implementation, this would use a vector embedding model
            let vector = vec![0.1, 0.2, 0.3, 0.4, 0.5];
            Ok(Value::Vector(vector))
        } else {
            Err(RuntimeError::new(
                &format!("Cannot embed {:?}", text),
                SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Calculate the similarity between two vectors
    fn similarity(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != 2 {
            return Err(RuntimeError::new(
                "similarity() requires exactly two arguments",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let vector1 = &arguments[0];
        let vector2 = &arguments[1];
        
        if let (Value::Vector(v1), Value::Vector(v2)) = (vector1, vector2) {
            // This is a placeholder implementation
            // In a real implementation, this would calculate the cosine similarity
            Ok(Value::Float(0.85))
        } else {
            Err(RuntimeError::new(
                &format!("Cannot calculate similarity between {:?} and {:?}", vector1, vector2),
                SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Find the nearest vectors to a given vector
    fn nearest(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        // This is a placeholder implementation
        // In a real implementation, this would find the nearest vectors
        Err(RuntimeError::new(
            "nearest() is not implemented yet",
            SourceLocation::new(0, 0, 0, 0, ""),
        ))
    }
    
    /// Get the current context
    fn current_context(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if !arguments.is_empty() {
            return Err(RuntimeError::new(
                "currentContext() takes no arguments",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        // This is a placeholder implementation
        // In a real implementation, this would get the current context
        Ok(Value::Context("MainProgram".to_string()))
    }
    
    /// Switch to a different context
    fn switch_context(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != 1 {
            return Err(RuntimeError::new(
                "switchContext() requires exactly one argument",
                SourceLocation::new(0, 0, 0, 0, ""),
            ));
        }
        
        let context = &arguments[0];
        
        if let Value::String(name) = context {
            // This is a placeholder implementation
            // In a real implementation, this would switch to the specified context
            Ok(Value::Context(name.clone()))
        } else {
            Err(RuntimeError::new(
                &format!("Context name must be a string, got {:?}", context),
                SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Merge two contexts
    fn merge_contexts(arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        // This is a placeholder implementation
        // In a real implementation, this would merge two contexts
        Err(RuntimeError::new(
            "mergeContexts() is not implemented yet",
            SourceLocation::new(0, 0, 0, 0, ""),
        ))
    }
    
    /// Convert a value to a string
    fn value_to_string(value: &Value) -> String {
        match value {
            Value::Void => "void".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::List(items) => {
                let items_str: Vec<String> = items.iter().map(Self::value_to_string).collect();
                format!("[{}]", items_str.join(", "))
            }
            Value::Map(map) => {
                let items_str: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, Self::value_to_string(v)))
                    .collect();
                format!("{{{}}}", items_str.join(", "))
            }
            Value::Function(name) => format!("<function {}>", name),
            Value::Vector(v) => format!("<vector with {} dimensions>", v.len()),
            Value::Context(name) => format!("<context {}>", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stdlib_new() {
        let stdlib = StdLib::new();
        
        assert!(stdlib.get_function("print").is_some());
        assert!(stdlib.get_function("toString").is_some());
        assert!(stdlib.get_function("parseInt").is_some());
        assert!(stdlib.get_function("parseFloat").is_some());
        assert!(stdlib.get_function("length").is_some());
        assert!(stdlib.get_function("isEmpty").is_some());
        assert!(stdlib.get_function("contains").is_some());
        assert!(stdlib.get_function("map").is_some());
        assert!(stdlib.get_function("filter").is_some());
        assert!(stdlib.get_function("reduce").is_some());
        assert!(stdlib.get_function("substring").is_some());
        assert!(stdlib.get_function("indexOf").is_some());
        assert!(stdlib.get_function("toLowerCase").is_some());
        assert!(stdlib.get_function("toUpperCase").is_some());
        assert!(stdlib.get_function("trim").is_some());
        assert!(stdlib.get_function("embed").is_some());
        assert!(stdlib.get_function("similarity").is_some());
        assert!(stdlib.get_function("nearest").is_some());
        assert!(stdlib.get_function("currentContext").is_some());
        assert!(stdlib.get_function("switchContext").is_some());
        assert!(stdlib.get_function("mergeContexts").is_some());
    }
    
    #[test]
    fn test_stdlib_register_function() {
        let mut stdlib = StdLib::new();
        
        stdlib.register_function("test", |_| Ok(Value::Int(42)));
        
        assert!(stdlib.get_function("test").is_some());
    }
    
    #[test]
    fn test_stdlib_call_function() {
        let mut stdlib = StdLib::new();
        
        stdlib.register_function("test", |_| Ok(Value::Int(42)));
        
        let result = stdlib.call_function("test", Vec::new()).unwrap();
        
        assert_eq!(result, Value::Int(42));
    }
    
    #[test]
    fn test_stdlib_call_function_unknown() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function("unknown", Vec::new());
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Unknown function: 'unknown'");
    }
    
    #[test]
    fn test_stdlib_to_string() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function("toString", vec![Value::Int(42)]).unwrap();
        
        assert_eq!(result, Value::String("42".to_string()));
    }
    
    #[test]
    fn test_stdlib_parse_int() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function("parseInt", vec![Value::String("42".to_string())]).unwrap();
        
        assert_eq!(result, Value::Int(42));
    }
    
    #[test]
    fn test_stdlib_parse_float() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function("parseFloat", vec![Value::String("3.14".to_string())]).unwrap();
        
        assert_eq!(result, Value::Float(3.14));
    }
    
    #[test]
    fn test_stdlib_length() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function("length", vec![Value::String("hello".to_string())]).unwrap();
        
        assert_eq!(result, Value::Int(5));
    }
    
    #[test]
    fn test_stdlib_is_empty() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function("isEmpty", vec![Value::String("".to_string())]).unwrap();
        
        assert_eq!(result, Value::Bool(true));
    }
    
    #[test]
    fn test_stdlib_contains() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function(
            "contains",
            vec![Value::String("hello".to_string()), Value::String("ell".to_string())]
        ).unwrap();
        
        assert_eq!(result, Value::Bool(true));
    }
    
    #[test]
    fn test_stdlib_substring() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function(
            "substring",
            vec![Value::String("hello".to_string()), Value::Int(1), Value::Int(4)]
        ).unwrap();
        
        assert_eq!(result, Value::String("ell".to_string()));
    }
    
    #[test]
    fn test_stdlib_index_of() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function(
            "indexOf",
            vec![Value::String("hello".to_string()), Value::String("ell".to_string())]
        ).unwrap();
        
        assert_eq!(result, Value::Int(1));
    }
    
    #[test]
    fn test_stdlib_to_lower_case() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function(
            "toLowerCase",
            vec![Value::String("HELLO".to_string())]
        ).unwrap();
        
        assert_eq!(result, Value::String("hello".to_string()));
    }
    
    #[test]
    fn test_stdlib_to_upper_case() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function(
            "toUpperCase",
            vec![Value::String("hello".to_string())]
        ).unwrap();
        
        assert_eq!(result, Value::String("HELLO".to_string()));
    }
    
    #[test]
    fn test_stdlib_trim() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function(
            "trim",
            vec![Value::String("  hello  ".to_string())]
        ).unwrap();
        
        assert_eq!(result, Value::String("hello".to_string()));
    }
    
    #[test]
    fn test_stdlib_embed() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function(
            "embed",
            vec![Value::String("hello".to_string())]
        ).unwrap();
        
        if let Value::Vector(v) = result {
            assert_eq!(v.len(), 5);
        } else {
            panic!("Expected Vector, got {:?}", result);
        }
    }
    
    #[test]
    fn test_stdlib_similarity() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function(
            "similarity",
            vec![
                Value::Vector(vec![0.1, 0.2, 0.3]),
                Value::Vector(vec![0.4, 0.5, 0.6])
            ]
        ).unwrap();
        
        if let Value::Float(f) = result {
            assert!(f > 0.0 && f <= 1.0);
        } else {
            panic!("Expected Float, got {:?}", result);
        }
    }
    
    #[test]
    fn test_stdlib_current_context() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function("currentContext", vec![]).unwrap();
        
        if let Value::Context(name) = result {
            assert_eq!(name, "MainProgram");
        } else {
            panic!("Expected Context, got {:?}", result);
        }
    }
    
    #[test]
    fn test_stdlib_switch_context() {
        let stdlib = StdLib::new();
        
        let result = stdlib.call_function(
            "switchContext",
            vec![Value::String("TestContext".to_string())]
        ).unwrap();
        
        if let Value::Context(name) = result {
            assert_eq!(name, "TestContext");
        } else {
            panic!("Expected Context, got {:?}", result);
        }
    }
    
    #[test]
    fn test_stdlib_value_to_string() {
        assert_eq!(StdLib::value_to_string(&Value::Void), "void");
        assert_eq!(StdLib::value_to_string(&Value::Bool(true)), "true");
        assert_eq!(StdLib::value_to_string(&Value::Int(42)), "42");
        assert_eq!(StdLib::value_to_string(&Value::Float(3.14)), "3.14");
        assert_eq!(StdLib::value_to_string(&Value::String("hello".to_string())), "hello");
        assert_eq!(StdLib::value_to_string(&Value::List(vec![Value::Int(1), Value::Int(2)])), "[1, 2]");
        
        let mut map = HashMap::new();
