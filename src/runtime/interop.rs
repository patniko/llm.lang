//! Interop module for the LLM.lang runtime
//!
//! This module provides the interoperability manager for the LLM.lang runtime,
//! which allows interoperability with other languages and systems.

use std::collections::HashMap;
use std::process::Command;
use crate::Value;
use super::error::RuntimeError;

/// A foreign function
type ForeignFunction = fn(Vec<Value>) -> Result<Value, RuntimeError>;

/// An interoperability manager
pub struct Interop {
    /// The registered foreign functions
    functions: HashMap<String, ForeignFunction>,
}

impl Interop {
    /// Create a new interoperability manager
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }
    
    /// Register a foreign function
    pub fn register_function(&mut self, name: &str, function: ForeignFunction) {
        self.functions.insert(name.to_string(), function);
    }
    
    /// Get a foreign function
    pub fn get_function(&self, name: &str) -> Option<&ForeignFunction> {
        self.functions.get(name)
    }
    
    /// Call a foreign function
    pub fn call_function(&self, name: &str, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if let Some(function) = self.functions.get(name) {
            function(arguments)
        } else {
            Err(RuntimeError::new(
                &format!("Unknown foreign function: '{}'", name),
                crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Execute a shell command
    pub fn execute_command(&self, command: &str, arguments: Vec<&str>) -> Result<Value, RuntimeError> {
        // Execute the command
        let output = Command::new(command)
            .args(arguments)
            .output()
            .map_err(|e| {
                RuntimeError::new(
                    &format!("Failed to execute command: {}", e),
                    crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
                )
            })?;
        
        // Check if the command was successful
        if output.status.success() {
            // Convert the output to a string
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            
            Ok(Value::String(stdout))
        } else {
            // Convert the error output to a string
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            
            Err(RuntimeError::new(
                &format!("Command failed: {}", stderr),
                crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Call a Python function
    pub fn call_python_function(&self, module: &str, function: &str, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        // Convert the arguments to Python code
        let args = arguments
            .iter()
            .map(|arg| self.value_to_python(arg))
            .collect::<Result<Vec<String>, RuntimeError>>()?
            .join(", ");
        
        // Create the Python code
        let code = format!(
            "import {} as module; print(repr(module.{}({})))",
            module, function, args
        );
        
        // Execute the Python code
        let output = self.execute_command("python", vec!["-c", &code])?;
        
        // Parse the output as a Python value
        self.python_to_value(&output.to_string())
    }
    
    /// Call a JavaScript function
    pub fn call_javascript_function(&self, module: &str, function: &str, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        // Convert the arguments to JavaScript code
        let args = arguments
            .iter()
            .map(|arg| self.value_to_javascript(arg))
            .collect::<Result<Vec<String>, RuntimeError>>()?
            .join(", ");
        
        // Create the JavaScript code
        let code = format!(
            "const module = require('{}'); console.log(JSON.stringify(module.{}({})))",
            module, function, args
        );
        
        // Execute the JavaScript code
        let output = self.execute_command("node", vec!["-e", &code])?;
        
        // Parse the output as a JSON value
        self.json_to_value(&output.to_string())
    }
    
    /// Convert a value to Python code
    fn value_to_python(&self, value: &Value) -> Result<String, RuntimeError> {
        match value {
            Value::Void => Ok("None".to_string()),
            Value::Bool(b) => Ok(if *b { "True".to_string() } else { "False".to_string() }),
            Value::Int(i) => Ok(i.to_string()),
            Value::Float(f) => Ok(f.to_string()),
            Value::String(s) => Ok(format!("\"{}\"", s.replace("\"", "\\\""))),
            Value::List(items) => {
                let items = items
                    .iter()
                    .map(|item| self.value_to_python(item))
                    .collect::<Result<Vec<String>, RuntimeError>>()?
                    .join(", ");
                
                Ok(format!("[{}]", items))
            }
            Value::Map(map) => {
                let items = map
                    .iter()
                    .map(|(key, value)| {
                        let value = self.value_to_python(value)?;
                        Ok(format!("\"{}\": {}", key.replace("\"", "\\\""), value))
                    })
                    .collect::<Result<Vec<String>, RuntimeError>>()?
                    .join(", ");
                
                Ok(format!("{{{}}}", items))
            }
            _ => Err(RuntimeError::new(
                &format!("Cannot convert value to Python: {:?}", value),
                crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
            )),
        }
    }
    
    /// Convert a value to JavaScript code
    fn value_to_javascript(&self, value: &Value) -> Result<String, RuntimeError> {
        match value {
            Value::Void => Ok("null".to_string()),
            Value::Bool(b) => Ok(if *b { "true".to_string() } else { "false".to_string() }),
            Value::Int(i) => Ok(i.to_string()),
            Value::Float(f) => Ok(f.to_string()),
            Value::String(s) => Ok(format!("\"{}\"", s.replace("\"", "\\\""))),
            Value::List(items) => {
                let items = items
                    .iter()
                    .map(|item| self.value_to_javascript(item))
                    .collect::<Result<Vec<String>, RuntimeError>>()?
                    .join(", ");
                
                Ok(format!("[{}]", items))
            }
            Value::Map(map) => {
                let items = map
                    .iter()
                    .map(|(key, value)| {
                        let value = self.value_to_javascript(value)?;
                        Ok(format!("\"{}\": {}", key.replace("\"", "\\\""), value))
                    })
                    .collect::<Result<Vec<String>, RuntimeError>>()?
                    .join(", ");
                
                Ok(format!("{{{}}}", items))
            }
            _ => Err(RuntimeError::new(
                &format!("Cannot convert value to JavaScript: {:?}", value),
                crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
            )),
        }
    }
    
    /// Convert Python output to a value
    fn python_to_value(&self, output: &str) -> Result<Value, RuntimeError> {
        // Parse the Python output
        let output = output.trim();
        
        if output == "None" {
            Ok(Value::Void)
        } else if output == "True" {
            Ok(Value::Bool(true))
        } else if output == "False" {
            Ok(Value::Bool(false))
        } else if let Ok(i) = output.parse::<i64>() {
            Ok(Value::Int(i))
        } else if let Ok(f) = output.parse::<f64>() {
            Ok(Value::Float(f))
        } else if output.starts_with("'") && output.ends_with("'") {
            Ok(Value::String(output[1..output.len() - 1].to_string()))
        } else if output.starts_with("\"") && output.ends_with("\"") {
            Ok(Value::String(output[1..output.len() - 1].to_string()))
        } else if output.starts_with("[") && output.ends_with("]") {
            // Parse a list
            let items = output[1..output.len() - 1].split(",").collect::<Vec<&str>>();
            
            let mut values = Vec::new();
            
            for item in items {
                values.push(self.python_to_value(item.trim())?);
            }
            
            Ok(Value::List(values))
        } else if output.starts_with("{") && output.ends_with("}") {
            // Parse a dictionary
            let items = output[1..output.len() - 1].split(",").collect::<Vec<&str>>();
            
            let mut map = HashMap::new();
            
            for item in items {
                let parts = item.split(":").collect::<Vec<&str>>();
                
                if parts.len() != 2 {
                    return Err(RuntimeError::new(
                        &format!("Invalid Python dictionary: {}", output),
                        crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
                    ));
                }
                
                let key = self.python_to_value(parts[0].trim())?;
                let value = self.python_to_value(parts[1].trim())?;
                
                if let Value::String(key) = key {
                    map.insert(key, value);
                } else {
                    return Err(RuntimeError::new(
                        &format!("Invalid Python dictionary key: {:?}", key),
                        crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
                    ));
                }
            }
            
            Ok(Value::Map(map))
        } else {
            Err(RuntimeError::new(
                &format!("Cannot parse Python output: {}", output),
                crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
    
    /// Convert JSON output to a value
    fn json_to_value(&self, output: &str) -> Result<Value, RuntimeError> {
        // Parse the JSON output
        let output = output.trim();
        
        if output == "null" {
            Ok(Value::Void)
        } else if output == "true" {
            Ok(Value::Bool(true))
        } else if output == "false" {
            Ok(Value::Bool(false))
        } else if let Ok(i) = output.parse::<i64>() {
            Ok(Value::Int(i))
        } else if let Ok(f) = output.parse::<f64>() {
            Ok(Value::Float(f))
        } else if output.starts_with("\"") && output.ends_with("\"") {
            Ok(Value::String(output[1..output.len() - 1].to_string()))
        } else if output.starts_with("[") && output.ends_with("]") {
            // Parse a list
            let items = output[1..output.len() - 1].split(",").collect::<Vec<&str>>();
            
            let mut values = Vec::new();
            
            for item in items {
                values.push(self.json_to_value(item.trim())?);
            }
            
            Ok(Value::List(values))
        } else if output.starts_with("{") && output.ends_with("}") {
            // Parse an object
            let items = output[1..output.len() - 1].split(",").collect::<Vec<&str>>();
            
            let mut map = HashMap::new();
            
            for item in items {
                let parts = item.split(":").collect::<Vec<&str>>();
                
                if parts.len() != 2 {
                    return Err(RuntimeError::new(
                        &format!("Invalid JSON object: {}", output),
                        crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
                    ));
                }
                
                let key = self.json_to_value(parts[0].trim())?;
                let value = self.json_to_value(parts[1].trim())?;
                
                if let Value::String(key) = key {
                    map.insert(key, value);
                } else {
                    return Err(RuntimeError::new(
                        &format!("Invalid JSON object key: {:?}", key),
                        crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
                    ));
                }
            }
            
            Ok(Value::Map(map))
        } else {
            Err(RuntimeError::new(
                &format!("Cannot parse JSON output: {}", output),
                crate::utils::SourceLocation::new(0, 0, 0, 0, ""),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_interop_new() {
        let interop = Interop::new();
        
        assert_eq!(interop.functions.len(), 0);
    }
    
    #[test]
    fn test_interop_register_function() {
        let mut interop = Interop::new();
        
        interop.register_function("test", |_| Ok(Value::Int(42)));
        
        assert_eq!(interop.functions.len(), 1);
        assert!(interop.get_function("test").is_some());
    }
    
    #[test]
    fn test_interop_call_function() {
        let mut interop = Interop::new();
        
        interop.register_function("test", |_| Ok(Value::Int(42)));
        
        let result = interop.call_function("test", Vec::new()).unwrap();
        
        assert_eq!(result, Value::Int(42));
    }
    
    #[test]
    fn test_interop_call_function_unknown() {
        let interop = Interop::new();
        
        let result = interop.call_function("test", Vec::new());
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Unknown foreign function: 'test'");
    }
    
    #[test]
    fn test_interop_value_to_python() {
        let interop = Interop::new();
        
        assert_eq!(interop.value_to_python(&Value::Void).unwrap(), "None");
        assert_eq!(interop.value_to_python(&Value::Bool(true)).unwrap(), "True");
        assert_eq!(interop.value_to_python(&Value::Bool(false)).unwrap(), "False");
        assert_eq!(interop.value_to_python(&Value::Int(42)).unwrap(), "42");
        assert_eq!(interop.value_to_python(&Value::Float(3.14)).unwrap(), "3.14");
        assert_eq!(interop.value_to_python(&Value::String("hello".to_string())).unwrap(), "\"hello\"");
        assert_eq!(interop.value_to_python(&Value::List(vec![Value::Int(1), Value::Int(2)])).unwrap(), "[1, 2]");
        assert_eq!(
            interop.value_to_python(&Value::Map(HashMap::from([
                ("a".to_string(), Value::Int(1)),
                ("b".to_string(), Value::Int(2)),
            ]))).unwrap(),
            "{\"a\": 1, \"b\": 2}"
        );
    }
    
    #[test]
    fn test_interop_value_to_javascript() {
        let interop = Interop::new();
        
        assert_eq!(interop.value_to_javascript(&Value::Void).unwrap(), "null");
        assert_eq!(interop.value_to_javascript(&Value::Bool(true)).unwrap(), "true");
        assert_eq!(interop.value_to_javascript(&Value::Bool(false)).unwrap(), "false");
        assert_eq!(interop.value_to_javascript(&Value::Int(42)).unwrap(), "42");
        assert_eq!(interop.value_to_javascript(&Value::Float(3.14)).unwrap(), "3.14");
        assert_eq!(interop.value_to_javascript(&Value::String("hello".to_string())).unwrap(), "\"hello\"");
        assert_eq!(interop.value_to_javascript(&Value::List(vec![Value::Int(1), Value::Int(2)])).unwrap(), "[1, 2]");
        assert_eq!(
            interop.value_to_javascript(&Value::Map(HashMap::from([
                ("a".to_string(), Value::Int(1)),
                ("b".to_string(), Value::Int(2)),
            ]))).unwrap(),
            "{\"a\": 1, \"b\": 2}"
        );
    }
    
    #[test]
    fn test_interop_python_to_value() {
        let interop = Interop::new();
        
        assert_eq!(interop.python_to_value("None").unwrap(), Value::Void);
        assert_eq!(interop.python_to_value("True").unwrap(), Value::Bool(true));
        assert_eq!(interop.python_to_value("False").unwrap(), Value::Bool(false));
        assert_eq!(interop.python_to_value("42").unwrap(), Value::Int(42));
        assert_eq!(interop.python_to_value("3.14").unwrap(), Value::Float(3.14));
        assert_eq!(interop.python_to_value("'hello'").unwrap(), Value::String("hello".to_string()));
        assert_eq!(interop.python_to_value("\"hello\"").unwrap(), Value::String("hello".to_string()));
    }
    
    #[test]
    fn test_interop_json_to_value() {
        let interop = Interop::new();
        
        assert_eq!(interop.json_to_value("null").unwrap(), Value::Void);
        assert_eq!(interop.json_to_value("true").unwrap(), Value::Bool(true));
        assert_eq!(interop.json_to_value("false").unwrap(), Value::Bool(false));
        assert_eq!(interop.json_to_value("42").unwrap(), Value::Int(42));
        assert_eq!(interop.json_to_value("3.14").unwrap(), Value::Float(3.14));
        assert_eq!(interop.json_to_value("\"hello\"").unwrap(), Value::String("hello".to_string()));
    }
}
