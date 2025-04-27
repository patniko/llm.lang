//! LLM.lang Interpreter
//!
//! This binary provides a command-line interface for executing LLM.lang source code.

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;
use std::time::Instant;

use clap::Parser;
use colored::Colorize;
use llm_lang::{execute, ExecuteOptions, Value};

/// LLM.lang Interpreter
#[derive(Parser, Debug)]
#[clap(
    name = "llmi",
    version = llm_lang::VERSION,
    author = "LLM.lang Contributors",
    about = "Interpreter for the LLM.lang programming language"
)]
struct Cli {
    /// The input file to execute
    #[clap(value_parser)]
    input: Option<PathBuf>,

    /// Whether to enable debug mode
    #[clap(short, long)]
    debug: bool,

    /// The maximum memory usage in megabytes
    #[clap(short, long)]
    memory_limit: Option<usize>,

    /// The maximum execution time in seconds
    #[clap(short, long)]
    time_limit: Option<u64>,

    /// Whether to disable parallel execution
    #[clap(long)]
    no_parallel: bool,

    /// Whether to disable vector operations
    #[clap(long)]
    no_vectors: bool,

    /// Whether to disable natural language processing
    #[clap(long)]
    no_nlp: bool,

    /// Whether to print execution statistics
    #[clap(short, long)]
    stats: bool,

    /// Whether to start an interactive REPL
    #[clap(short, long)]
    interactive: bool,
}

fn main() {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Create execution options
    let options = ExecuteOptions {
        debug: cli.debug,
        max_memory: cli.memory_limit.map(|mb| mb * 1024 * 1024),
        max_time: cli.time_limit.map(|s| s * 1000),
        parallel: !cli.no_parallel,
        vectors: !cli.no_vectors,
        nlp: !cli.no_nlp,
    };

    // Check if we should start an interactive REPL
    if cli.interactive || cli.input.is_none() {
        run_repl(options);
    } else {
        // Execute the input file
        let input = cli.input.unwrap();
        let source = match fs::read_to_string(&input) {
            Ok(source) => source,
            Err(err) => {
                eprintln!("{}: Failed to read input file: {}", "Error".red().bold(), err);
                process::exit(1);
            }
        };

        execute_source(&source, &input.to_string_lossy(), options, cli.stats);
    }
}

/// Execute LLM.lang source code
fn execute_source(source: &str, filename: &str, options: ExecuteOptions, show_stats: bool) {
    let start_time = Instant::now();

    match execute(source, options) {
        Ok(result) => {
            // Print the result
            match result.value {
                Value::Void => {}
                _ => println!("{}", format_value(&result.value)),
            }

            // Print execution statistics if requested
            if show_stats {
                let elapsed = start_time.elapsed();
                println!("\n{}: Execution statistics", "Info".blue().bold());
                println!("  Time: {:.2?}", elapsed);
                println!("  Peak memory: {} bytes", result.stats.peak_memory);
                println!("  Instructions executed: {}", result.stats.instructions);
            }
        }
        Err(err) => {
            eprintln!("{}: {}", "Error".red().bold(), err);
            process::exit(1);
        }
    }
}

/// Run an interactive REPL
fn run_repl(options: ExecuteOptions) {
    println!("{} v{}", "LLM.lang Interactive Interpreter".green().bold(), llm_lang::VERSION);
    println!("Type 'exit' or 'quit' to exit, 'help' for help.");

    let mut line_number = 1;
    let mut multiline_input = String::new();
    let mut in_multiline = false;

    loop {
        // Print prompt
        if in_multiline {
            print!("... ");
        } else {
            print!(">>> ");
        }
        io::stdout().flush().unwrap();

        // Read input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();

        // Check for special commands
        if !in_multiline {
            match input {
                "exit" | "quit" => break,
                "help" => {
                    print_help();
                    continue;
                }
                _ => {}
            }
        }

        // Check for multiline input
        if input.ends_with('{') {
            in_multiline = true;
            multiline_input.push_str(input);
            multiline_input.push('\n');
            continue;
        } else if in_multiline && input == "}" {
            in_multiline = false;
            multiline_input.push_str(input);
            
            // Execute the multiline input
            let source = multiline_input.clone();
            execute_source(&source, &format!("<repl-{}>", line_number), options.clone(), false);
            
            // Reset multiline input
            multiline_input.clear();
            line_number += 1;
            continue;
        } else if in_multiline {
            multiline_input.push_str(input);
            multiline_input.push('\n');
            continue;
        }

        // Execute the input
        if !input.is_empty() {
            let source = if !input.contains("context") && !input.contains("fn") {
                // Wrap simple expressions in a context and main function for convenience
                format!(
                    "context REPL {{ fn main() {{ {} }} }}",
                    input
                )
            } else {
                input.to_string()
            };

            execute_source(&source, &format!("<repl-{}>", line_number), options.clone(), false);
            line_number += 1;
        }
    }

    println!("Goodbye!");
}

/// Print help information for the REPL
fn print_help() {
    println!("\n{}", "LLM.lang Interactive Interpreter Help".green().bold());
    println!("Commands:");
    println!("  exit, quit - Exit the interpreter");
    println!("  help       - Show this help message");
    println!("\nMultiline Input:");
    println!("  Start a line with a '{{' to begin multiline input");
    println!("  Enter a single '}}' on a line to end multiline input and execute it");
    println!("\nExamples:");
    println!("  >>> print(\"Hello, World!\")");
    println!("  >>> var x = 42");
    println!("  >>> context Example {{ fn main() {{ print(\"Hello from a context!\") }} }}");
    println!("");
}

/// Format a value for display
fn format_value(value: &Value) -> String {
    match value {
        Value::Void => "void".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => format!("\"{}\"", s),
        Value::List(items) => {
            let items_str: Vec<String> = items.iter().map(format_value).collect();
            format!("[{}]", items_str.join(", "))
        }
        Value::Map(map) => {
            let items_str: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, format_value(v)))
                .collect();
            format!("{{{}}}", items_str.join(", "))
        }
        Value::Function(name) => format!("<function {}>", name),
        Value::Vector(v) => format!("<vector with {} dimensions>", v.len()),
        Value::Context(name) => format!("<context {}>", name),
    }
}
