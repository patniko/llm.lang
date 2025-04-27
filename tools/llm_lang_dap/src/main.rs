//! LLM.lang Debug Adapter Protocol implementation
//!
//! This binary provides a Debug Adapter Protocol implementation for LLM.lang.

use std::sync::Arc;
use std::collections::HashMap;
use std::path::PathBuf;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};

use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use log::{debug, error, info, warn};
use dashmap::DashMap;
use async_trait::async_trait;

// DAP protocol types
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    seq: u64,
    #[serde(rename = "type")]
    message_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(flatten)]
    message: Message,
    command: String,
    arguments: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(flatten)]
    message_info: Message,
    request_seq: u64,
    success: bool,
    command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    #[serde(flatten)]
    message_info: Message,
    event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Value>,
}

// Debugger state
struct Debugger {
    /// The next sequence number for messages
    seq: u64,
    
    /// The current source file being debugged
    source_file: Option<PathBuf>,
    
    /// The current AST of the source file
    ast: Option<llm_lang::parser::ast::Ast>,
    
    /// The current execution state
    execution_state: ExecutionState,
    
    /// The breakpoints set by the client
    breakpoints: DashMap<PathBuf, Vec<Breakpoint>>,
    
    /// The variables in the current scope
    variables: DashMap<String, llm_lang::Value>,
    
    /// The call stack
    call_stack: Vec<StackFrame>,
}

#[derive(Debug, Clone)]
enum ExecutionState {
    /// The debugger is not running
    Stopped,
    
    /// The debugger is running
    Running,
    
    /// The debugger is paused at a breakpoint
    Paused {
        /// The source file
        source: PathBuf,
        
        /// The line number
        line: u64,
        
        /// The column number
        column: u64,
    },
}

#[derive(Debug, Clone)]
struct Breakpoint {
    /// The ID of the breakpoint
    id: u64,
    
    /// The line number
    line: u64,
    
    /// The column number
    column: Option<u64>,
    
    /// Whether the breakpoint is verified
    verified: bool,
    
    /// The condition for the breakpoint
    condition: Option<String>,
    
    /// The log message for the breakpoint
    log_message: Option<String>,
}

#[derive(Debug, Clone)]
struct StackFrame {
    /// The ID of the stack frame
    id: u64,
    
    /// The name of the function
    name: String,
    
    /// The source file
    source: PathBuf,
    
    /// The line number
    line: u64,
    
    /// The column number
    column: u64,
}

impl Debugger {
    /// Create a new debugger
    fn new() -> Self {
        Self {
            seq: 1,
            source_file: None,
            ast: None,
            execution_state: ExecutionState::Stopped,
            breakpoints: DashMap::new(),
            variables: DashMap::new(),
            call_stack: Vec::new(),
        }
    }
    
    /// Get the next sequence number
    fn next_seq(&mut self) -> u64 {
        let seq = self.seq;
        self.seq += 1;
        seq
    }
    
    /// Load a source file
    fn load_source(&mut self, path: PathBuf) -> Result<(), String> {
        // Read the source file
        let source = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read source file: {}", e))?;
        
        // Parse the source file
        let mut lexer = llm_lang::lexer::Lexer::new(&source);
        let tokens = lexer.tokenize()
            .map_err(|e| format!("Lexer error: {}", e))?;
        
        let mut parser = llm_lang::parser::Parser::new(tokens);
        let ast = parser.parse()
            .map_err(|e| format!("Parser error: {}", e))?;
        
        // Store the source file and AST
        self.source_file = Some(path);
        self.ast = Some(ast);
        
        Ok(())
    }
    
    /// Set breakpoints in a source file
    fn set_breakpoints(&self, path: PathBuf, breakpoints: Vec<Breakpoint>) -> Vec<Breakpoint> {
        // Store the breakpoints
        self.breakpoints.insert(path, breakpoints.clone());
        
        // Return the breakpoints (in a real implementation, we would verify them)
        breakpoints.into_iter().map(|mut bp| {
            bp.verified = true;
            bp
        }).collect()
    }
    
    /// Start debugging
    fn start(&mut self, stop_on_entry: bool) -> Result<(), String> {
        // Check if we have a source file
        let source_file = self.source_file.clone()
            .ok_or_else(|| "No source file loaded".to_string())?;
        
        // Check if we have an AST
        let ast = self.ast.clone()
            .ok_or_else(|| "No AST loaded".to_string())?;
        
        // Create a runtime engine
        let options = llm_lang::runtime::engine::EngineOptions {
            debug: true,
            max_memory: None,
            max_time: None,
            parallel: true,
            vectors: true,
            nlp: true,
            self_modifying: true,
        };
        
        let mut engine = llm_lang::runtime::engine::Engine::new(options);
        
        // Set the execution state
        if stop_on_entry {
            self.execution_state = ExecutionState::Paused {
                source: source_file,
                line: 1,
                column: 1,
            };
        } else {
            self.execution_state = ExecutionState::Running;
            
            // Execute the AST
            // In a real implementation, we would execute the AST step by step
            // and check for breakpoints
            match engine.execute(ast) {
                Ok(value) => {
                    // Store the result
                    self.variables.insert("result".to_string(), value);
                    
                    // Set the execution state to stopped
                    self.execution_state = ExecutionState::Stopped;
                }
                Err(e) => {
                    return Err(format!("Runtime error: {}", e));
                }
            }
        }
        
        Ok(())
    }
    
    /// Continue execution
    fn continue_execution(&mut self) -> Result<(), String> {
        // Check if we're paused
        match self.execution_state {
            ExecutionState::Paused { .. } => {
                // In a real implementation, we would continue execution
                // until the next breakpoint or the end of the program
                self.execution_state = ExecutionState::Stopped;
                Ok(())
            }
            _ => Err("Not paused".to_string()),
        }
    }
    
    /// Step over
    fn step_over(&mut self) -> Result<(), String> {
        // Check if we're paused
        match self.execution_state {
            ExecutionState::Paused { .. } => {
                // In a real implementation, we would step over the current line
                // and pause at the next line
                self.execution_state = ExecutionState::Stopped;
                Ok(())
            }
            _ => Err("Not paused".to_string()),
        }
    }
    
    /// Step into
    fn step_into(&mut self) -> Result<(), String> {
        // Check if we're paused
        match self.execution_state {
            ExecutionState::Paused { .. } => {
                // In a real implementation, we would step into the current function
                // and pause at the first line
                self.execution_state = ExecutionState::Stopped;
                Ok(())
            }
            _ => Err("Not paused".to_string()),
        }
    }
    
    /// Step out
    fn step_out(&mut self) -> Result<(), String> {
        // Check if we're paused
        match self.execution_state {
            ExecutionState::Paused { .. } => {
                // In a real implementation, we would step out of the current function
                // and pause at the next line in the calling function
                self.execution_state = ExecutionState::Stopped;
                Ok(())
            }
            _ => Err("Not paused".to_string()),
        }
    }
    
    /// Get the variables in the current scope
    fn get_variables(&self) -> HashMap<String, llm_lang::Value> {
        let mut variables = HashMap::new();
        
        for entry in self.variables.iter() {
            variables.insert(entry.key().clone(), entry.value().clone());
        }
        
        variables
    }
    
    /// Get the call stack
    fn get_call_stack(&self) -> Vec<StackFrame> {
        self.call_stack.clone()
    }
    
    /// Evaluate an expression
    fn evaluate(&self, expression: &str) -> Result<llm_lang::Value, String> {
        // In a real implementation, we would parse and evaluate the expression
        // using the current scope
        Ok(llm_lang::Value::String(format!("Evaluated: {}", expression)))
    }
}

// DAP server
struct DebugServer {
    /// The debugger
    debugger: Arc<Mutex<Debugger>>,
}

impl DebugServer {
    /// Create a new debug server
    fn new() -> Self {
        Self {
            debugger: Arc::new(Mutex::new(Debugger::new())),
        }
    }
    
    /// Handle a request
    async fn handle_request(&self, request: Request) -> Response {
        let mut debugger = self.debugger.lock().await;
        
        let seq = debugger.next_seq();
        let request_seq = request.message.seq;
        let command = request.command.clone();
        
        match request.command.as_str() {
            "initialize" => {
                // Initialize the debugger
                let body = serde_json::json!({
                    "supportsConfigurationDoneRequest": true,
                    "supportsEvaluateForHovers": true,
                    "supportsStepBack": false,
                    "supportsSetVariable": true,
                    "supportsRestartFrame": false,
                    "supportsGotoTargetsRequest": false,
                    "supportsStepInTargetsRequest": false,
                    "supportsCompletionsRequest": true,
                    "supportsModulesRequest": false,
                    "supportsValueFormattingOptions": true,
                    "supportsExceptionInfoRequest": true,
                    "supportTerminateDebuggee": true,
                    "supportsDelayedStackTraceLoading": true,
                    "supportsLogPoints": true,
                    "supportsConditionalBreakpoints": true,
                    "supportsHitConditionalBreakpoints": false,
                    "supportsSetExpression": true,
                    "supportsTerminateRequest": true,
                    "supportsDataBreakpoints": false,
                    "supportsReadMemoryRequest": false,
                    "supportsDisassembleRequest": false,
                    "supportsCancelRequest": true,
                    "supportsBreakpointLocationsRequest": true,
                    "supportsClipboardContext": false,
                    "supportsSteppingGranularity": false,
                    "supportsInstructionBreakpoints": false,
                    "supportsExceptionFilterOptions": false,
                });
                
                Response {
                    message_info: Message {
                        seq,
                        message_type: "response".to_string(),
                    },
                    request_seq,
                    success: true,
                    command,
                    body: Some(body),
                    message_text: None,
                }
            }
            "launch" => {
                // Launch the debugger
                let arguments = request.arguments.unwrap_or_default();
                let program = arguments.get("program").and_then(|v| v.as_str()).unwrap_or("");
                let stop_on_entry = arguments.get("stopOnEntry").and_then(|v| v.as_bool()).unwrap_or(false);
                
                // Load the source file
                let path = PathBuf::from(program);
                match debugger.load_source(path) {
                    Ok(()) => {
                        // Start debugging
                        match debugger.start(stop_on_entry) {
                            Ok(()) => {
                                Response {
                                message_info: Message {
                                    seq,
                                    message_type: "response".to_string(),
                                },
                                request_seq,
                                success: true,
                                command,
                                body: None,
                                message_text: None,
                                }
                            }
                            Err(e) => {
                                Response {
                                message_info: Message {
                                    seq,
                                    message_type: "response".to_string(),
                                },
                                request_seq,
                                success: false,
                                command,
                                body: None,
                                message_text: Some(e),
                                }
                            }
                        }
                    }
                    Err(e) => {
                        Response {
                            message_info: Message {
                                seq,
                                message_type: "response".to_string(),
                            },
                            request_seq,
                            success: false,
                            command,
                            body: None,
                            message_text: Some(e),
                        }
                    }
                }
            }
            "setBreakpoints" => {
                // Set breakpoints
                let arguments = request.arguments.unwrap_or_default();
                let empty_map = serde_json::Map::new();
                let source = arguments.get("source").and_then(|v| v.as_object()).unwrap_or(&empty_map);
                let path = source.get("path").and_then(|v| v.as_str()).unwrap_or("");
                let empty_vec = Vec::new();
                let breakpoints = arguments.get("breakpoints").and_then(|v| v.as_array()).unwrap_or(&empty_vec);
                
                // Convert the breakpoints
                let breakpoints: Vec<Breakpoint> = breakpoints.iter().enumerate().map(|(i, bp)| {
                    let line = bp.get("line").and_then(|v| v.as_u64()).unwrap_or(0);
                    let column = bp.get("column").and_then(|v| v.as_u64());
                    let condition = bp.get("condition").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let log_message = bp.get("logMessage").and_then(|v| v.as_str()).map(|s| s.to_string());
                    
                    Breakpoint {
                        id: i as u64 + 1,
                        line,
                        column,
                        verified: false,
                        condition,
                        log_message,
                    }
                }).collect();
                
                // Set the breakpoints
                let path = PathBuf::from(path);
                let breakpoints = debugger.set_breakpoints(path, breakpoints);
                
                // Convert the breakpoints to JSON
                let breakpoints: Vec<serde_json::Value> = breakpoints.iter().map(|bp| {
                    serde_json::json!({
                        "id": bp.id,
                        "verified": bp.verified,
                        "line": bp.line,
                        "column": bp.column,
                    })
                }).collect();
                
                Response {
                    message_info: Message {
                        seq,
                        message_type: "response".to_string(),
                    },
                    request_seq,
                    success: true,
                    command,
                    body: Some(serde_json::json!({
                        "breakpoints": breakpoints,
                    })),
                    message_text: None,
                }
            }
            "configurationDone" => {
                // Configuration is done
                Response {
                    message_info: Message {
                        seq,
                        message_type: "response".to_string(),
                    },
                    request_seq,
                    success: true,
                    command,
                    body: None,
                    message_text: None,
                }
            }
            "continue" => {
                // Continue execution
                match debugger.continue_execution() {
                    Ok(()) => {
                        Response {
                            message_info: Message {
                                seq,
                                message_type: "response".to_string(),
                            },
                            request_seq,
                            success: true,
                            command,
                            body: Some(serde_json::json!({
                                "allThreadsContinued": true,
                            })),
                            message_text: None,
                        }
                    }
                    Err(e) => {
                        Response {
                            message_info: Message {
                                seq,
                                message_type: "response".to_string(),
                            },
                            request_seq,
                            success: false,
                            command,
                            body: None,
                            message_text: Some(e),
                        }
                    }
                }
            }
            "next" => {
                // Step over
                match debugger.step_over() {
                    Ok(()) => {
                        Response {
                            message_info: Message {
                                seq,
                                message_type: "response".to_string(),
                            },
                            request_seq,
                            success: true,
                            command,
                            body: None,
                            message_text: None,
                        }
                    }
                    Err(e) => {
                        Response {
                            message_info: Message {
                                seq,
                                message_type: "response".to_string(),
                            },
                            request_seq,
                            success: false,
                            command,
                            body: None,
                            message_text: Some(e),
                        }
                    }
                }
            }
            "stepIn" => {
                // Step into
                match debugger.step_into() {
                    Ok(()) => {
                        Response {
                            message_info: Message {
                                seq,
                                message_type: "response".to_string(),
                            },
                            request_seq,
                            success: true,
                            command,
                            body: None,
                            message_text: None,
                        }
                    }
                    Err(e) => {
                        Response {
                            message_info: Message {
                                seq,
                                message_type: "response".to_string(),
                            },
                            request_seq,
                            success: false,
                            command,
                            body: None,
                            message_text: Some(e),
                        }
                    }
                }
            }
            "stepOut" => {
                // Step out
                match debugger.step_out() {
                    Ok(()) => {
                        Response {
                            message_info: Message {
                                seq,
                                message_type: "response".to_string(),
                            },
                            request_seq,
                            success: true,
                            command,
                            body: None,
                            message_text: None,
                        }
                    }
                    Err(e) => {
                        Response {
                            message_info: Message {
                                seq,
                                message_type: "response".to_string(),
                            },
                            request_seq,
                            success: false,
                            command,
                            body: None,
                            message_text: Some(e),
                        }
                    }
                }
            }
            "threads" => {
                // Get threads
                Response {
                    message_info: Message {
                        seq,
                        message_type: "response".to_string(),
                    },
                    request_seq,
                    success: true,
                    command,
                    body: Some(serde_json::json!({
                        "threads": [
                            {
                                "id": 1,
                                "name": "main",
                            }
                        ],
                    })),
                    message_text: None,
                }
            }
            "stackTrace" => {
                // Get stack trace
                let stack_frames = debugger.get_call_stack();
                
                // Convert the stack frames to JSON
                let frames: Vec<serde_json::Value> = stack_frames.iter().map(|frame| {
                    serde_json::json!({
                        "id": frame.id,
                        "name": frame.name,
                        "source": {
                            "name": frame.source.file_name().unwrap_or_default().to_string_lossy(),
                            "path": frame.source.to_string_lossy(),
                        },
                        "line": frame.line,
                        "column": frame.column,
                    })
                }).collect();
                
                Response {
                    message_info: Message {
                        seq,
                        message_type: "response".to_string(),
                    },
                    request_seq,
                    success: true,
                    command,
                    body: Some(serde_json::json!({
                        "stackFrames": frames,
                        "totalFrames": frames.len(),
                    })),
                    message_text: None,
                }
            }
            "scopes" => {
                // Get scopes
                Response {
                    message_info: Message {
                        seq,
                        message_type: "response".to_string(),
                    },
                    request_seq,
                    success: true,
                    command,
                    body: Some(serde_json::json!({
                        "scopes": [
                            {
                                "name": "Local",
                                "variablesReference": 1,
                                "expensive": false,
                            }
                        ],
                    })),
                    message_text: None,
                }
            }
            "variables" => {
                // Get variables
                let variables = debugger.get_variables();
                
                // Convert the variables to JSON
                let variables: Vec<serde_json::Value> = variables.iter().map(|(name, value)| {
                    serde_json::json!({
                        "name": name,
                        "value": format!("{:?}", value),
                        "type": match value {
                            llm_lang::Value::Void => "Void",
                            llm_lang::Value::Bool(_) => "Bool",
                            llm_lang::Value::Int(_) => "Int",
                            llm_lang::Value::Float(_) => "Float",
                            llm_lang::Value::String(_) => "String",
                            llm_lang::Value::List(_) => "List",
                            llm_lang::Value::Map(_) => "Map",
                            llm_lang::Value::Vector(_) => "Vector",
                            llm_lang::Value::Function(_) => "Function",
                            llm_lang::Value::Context(_) => "Context",
                        },
                        "variablesReference": 0,
                    })
                }).collect();
                
                Response {
                    message_info: Message {
                        seq,
                        message_type: "response".to_string(),
                    },
                    request_seq,
                    success: true,
                    command,
                    body: Some(serde_json::json!({
                        "variables": variables,
                    })),
                    message_text: None,
                }
            }
            "evaluate" => {
                // Evaluate an expression
                let arguments = request.arguments.unwrap_or_default();
                let expression = arguments.get("expression").and_then(|v| v.as_str()).unwrap_or("");
                
                match debugger.evaluate(expression) {
                    Ok(value) => {
                        Response {
                            message_info: Message {
                                seq,
                                message_type: "response".to_string(),
                            },
                            request_seq,
                            success: true,
                            command,
                            body: Some(serde_json::json!({
                                "result": format!("{:?}", value),
                                "type": match value {
                                    llm_lang::Value::Void => "Void",
                                    llm_lang::Value::Bool(_) => "Bool",
                                    llm_lang::Value::Int(_) => "Int",
                                    llm_lang::Value::Float(_) => "Float",
                                    llm_lang::Value::String(_) => "String",
                                    llm_lang::Value::List(_) => "List",
                                    llm_lang::Value::Map(_) => "Map",
                                    llm_lang::Value::Vector(_) => "Vector",
                                    llm_lang::Value::Function(_) => "Function",
                                    llm_lang::Value::Context(_) => "Context",
                                },
                                "variablesReference": 0,
                            })),
                            message_text: None,
                        }
                    }
                    Err(e) => {
                        Response {
                            message_info: Message {
                                seq,
                                message_type: "response".to_string(),
                            },
                            request_seq,
                            success: false,
                            command,
                            body: None,
                            message_text: Some(e),
                        }
                    }
                }
            }
            "disconnect" => {
                // Disconnect the debugger
                Response {
                    message_info: Message {
                        seq,
                        message_type: "response".to_string(),
                    },
                    request_seq,
                    success: true,
                    command,
                    body: None,
                    message_text: None,
                }
            }
            _ => {
                // Unknown command
                Response {
                    message_info: Message {
                        seq,
                        message_type: "response".to_string(),
                    },
                    request_seq,
                    success: false,
                    command,
                    body: None,
                    message_text: Some(format!("Unknown command: {}", request.command)),
                }
            }
        }
    }
    
    /// Send an event
    async fn send_event(&self, event: &str, body: Option<serde_json::Value>) -> Event {
        let mut debugger = self.debugger.lock().await;
        
        let seq = debugger.next_seq();
        
        Event {
            message_info: Message {
                seq,
                message_type: "event".to_string(),
            },
            event: event.to_string(),
            body,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    // Create a debug server
    let debug_server = Arc::new(DebugServer::new());
    
    // Create a TCP listener
    let listener = TcpListener::bind("127.0.0.1:4711")?;
    println!("Listening on {}", listener.local_addr()?);
    
    // Accept connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let debug_server = debug_server.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, debug_server).await {
                        error!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Error accepting connection: {}", e);
            }
        }
    }
    
    Ok(())
}

async fn handle_connection(stream: TcpStream, debug_server: Arc<DebugServer>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Use standard IO for communication
    let reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));
    let writer = BufWriter::new(stream);
    
    // Create a JSON-RPC server
    let mut server = jsonrpc::Server::new(reader, writer);
    
    // Send the initialized event
    let initialized_event = debug_server.send_event("initialized", None).await;
    server.send_event(&initialized_event).await?;
    
    // Handle requests
    loop {
        match server.receive_request().await {
            Ok(request) => {
                let response = debug_server.handle_request(request).await;
                server.send_response(&response).await?;
                
                // Check if we need to send any events
                match response.command.as_str() {
                    "launch" => {
                        if response.success {
                            // Send the stopped event if we're stopping on entry
                            let debugger = debug_server.debugger.lock().await;
                            match debugger.execution_state {
                                ExecutionState::Paused { .. } => {
                                    let stopped_event = debug_server.send_event("stopped", Some(serde_json::json!({
                                        "reason": "entry",
                                        "threadId": 1,
                                        "allThreadsStopped": true,
                                    }))).await;
                                    server.send_event(&stopped_event).await?;
                                }
                                _ => {}
                            }
                        }
                    }
                    "continue" | "next" | "stepIn" | "stepOut" => {
                        if response.success {
                            // Send the stopped event if we're paused
                            let debugger = debug_server.debugger.lock().await;
                            match debugger.execution_state {
                                ExecutionState::Paused { .. } => {
                                    let stopped_event = debug_server.send_event("stopped", Some(serde_json::json!({
                                        "reason": "step",
                                        "threadId": 1,
                                        "allThreadsStopped": true,
                                    }))).await;
                                    server.send_event(&stopped_event).await?;
                                }
                                ExecutionState::Stopped => {
                                    let terminated_event = debug_server.send_event("terminated", None).await;
                                    server.send_event(&terminated_event).await?;
                                }
                                _ => {}
                            }
                        }
                    }
                    "disconnect" => {
                        // Send the terminated event
                        let terminated_event = debug_server.send_event("terminated", None).await;
                        server.send_event(&terminated_event).await?;
                        
                        // Exit the loop
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => {
                error!("Error receiving request: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

// JSON-RPC server
mod jsonrpc {
    use std::io::{BufRead, BufReader, BufWriter, Read, Write};
    use std::marker::Unpin;
    
    use super::{Request, Response, Event};
    
    pub struct Server<R, W>
    where
        R: Read,
        W: Write,
    {
        reader: BufReader<R>,
        writer: BufWriter<W>,
    }
    
    impl<R, W> Server<R, W>
    where
        R: Read,
        W: Write,
    {
        /// Create a new JSON-RPC server
        pub fn new(reader: R, writer: W) -> Self {
            Self {
                reader: BufReader::new(reader),
                writer: BufWriter::new(writer),
            }
        }
        
        /// Receive a request
        pub async fn receive_request(&mut self) -> Result<Request, Box<dyn std::error::Error + Send + Sync>> {
            // Read the Content-Length header
            let mut content_length = None;
            
            loop {
                let mut line = String::new();
                let bytes_read = self.reader.read_line(&mut line)?;
                
                if bytes_read == 0 {
                    return Err("Connection closed".into());
                }
                
                let line = line.trim();
                
                if line.is_empty() {
                    break;
                }
                
                if line.starts_with("Content-Length:") {
                    let length = line.trim_start_matches("Content-Length:").trim();
                    content_length = Some(length.parse::<usize>()?);
                }
            }
            
            // Read the content
            let content_length = content_length.ok_or("Missing Content-Length header")?;
            let mut content = vec![0; content_length];
            self.reader.read_exact(&mut content)?;
            
            // Parse the content
            let request: Request = serde_json::from_slice(&content)?;
            
            Ok(request)
        }
        
        /// Send a response
        pub async fn send_response(&mut self, response: &Response) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            // Serialize the response
            let content = serde_json::to_vec(response)?;
            
            // Write the headers
            self.writer.write_all(format!("Content-Length: {}\r\n\r\n", content.len()).as_bytes())?;
            
            // Write the content
            self.writer.write_all(&content)?;
            self.writer.flush()?;
            
            Ok(())
        }
        
        /// Send an event
        pub async fn send_event(&mut self, event: &Event) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            // Serialize the event
            let content = serde_json::to_vec(event)?;
            
            // Write the headers
            self.writer.write_all(format!("Content-Length: {}\r\n\r\n", content.len()).as_bytes())?;
            
            // Write the content
            self.writer.write_all(&content)?;
            self.writer.flush()?;
            
            Ok(())
        }
    }
}
