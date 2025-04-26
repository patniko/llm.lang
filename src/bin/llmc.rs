//! LLM.lang Compiler
//!
//! This binary provides a command-line interface for compiling LLM.lang source code.

use std::fs;
use std::path::PathBuf;
use std::process;

use clap::{Parser, ValueEnum};
use colored::Colorize;
use llm_lang::{compile, CompileOptions};

/// LLM.lang Compiler
#[derive(Parser, Debug)]
#[clap(
    name = "llmc",
    version = llm_lang::VERSION,
    author = "LLM.lang Contributors",
    about = "Compiler for the LLM.lang programming language"
)]
struct Cli {
    /// The input file to compile
    #[clap(value_parser)]
    input: PathBuf,

    /// The output file
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>,

    /// Whether to optimize the compiled code
    #[clap(short, long)]
    optimize: bool,

    /// The optimization level (0-3)
    #[clap(short = 'O', long, default_value = "2")]
    optimization_level: u8,

    /// Whether to include debug information
    #[clap(short, long)]
    debug: bool,

    /// The target platform
    #[clap(short, long, default_value = "native")]
    target: String,

    /// The output format
    #[clap(short, long, value_enum, default_value_t = OutputFormat::Binary)]
    format: OutputFormat,

    /// Whether to print verbose output
    #[clap(short, long)]
    verbose: bool,
}

/// Output formats for the compiler
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    /// Binary executable
    Binary,
    /// Assembly code
    Assembly,
    /// LLVM IR
    Llvm,
    /// WebAssembly
    Wasm,
}

fn main() {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Read the input file
    let source = match fs::read_to_string(&cli.input) {
        Ok(source) => source,
        Err(err) => {
            eprintln!("{}: Failed to read input file: {}", "Error".red().bold(), err);
            process::exit(1);
        }
    };

    // Determine the output file
    let output = cli.output.unwrap_or_else(|| {
        let mut output = cli.input.clone();
        output.set_extension(match cli.format {
            OutputFormat::Binary => {
                if cfg!(windows) {
                    "exe"
                } else {
                    "out"
                }
            }
            OutputFormat::Assembly => "s",
            OutputFormat::Llvm => "ll",
            OutputFormat::Wasm => "wasm",
        });
        output
    });

    // Create compilation options
    let options = CompileOptions {
        optimize: cli.optimize,
        optimization_level: cli.optimization_level,
        debug_info: cli.debug,
        target: cli.target,
    };

    // Compile the source code
    if cli.verbose {
        println!("{}: Compiling {} to {}", "Info".blue().bold(), cli.input.display(), output.display());
    }

    match compile(&source, options) {
        Ok(program) => {
            // In a real implementation, this would write the compiled program to the output file
            // For now, we'll just print a success message
            println!("{}: Successfully compiled {} to {}", "Success".green().bold(), cli.input.display(), output.display());

            if cli.verbose {
                println!("{}: Compilation metadata:", "Info".blue().bold());
                for (key, value) in program.metadata() {
                    println!("  {}: {}", key, value);
                }
            }
        }
        Err(err) => {
            eprintln!("{}: {}", "Error".red().bold(), err);
            process::exit(1);
        }
    }
}
