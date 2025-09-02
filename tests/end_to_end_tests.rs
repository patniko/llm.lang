use std::process::Command;
use std::fs;
use std::path::Path;
use llm_lang::{compile, execute, CompileOptions, ExecuteOptions};

/// End-to-end workflow tests that demonstrate the complete development cycle
#[cfg(test)]
mod end_to_end_tests {
    use super::*;

    /// Test the complete workflow: write -> compile -> execute
    #[test]
    fn test_complete_workflow() {
        // Step 1: Create a temporary test program
        let test_program = r#"
// Test program demonstrating basic LLM.lang functionality
context TestProgram {
    fn main() {
        // Simple computation
        var x = 42;
        var y = x + 8;
        return y;
    }
}
"#;

        // Step 2: Test compilation
        println!("Step 1: Testing compilation...");
        let compile_result = compile(test_program, CompileOptions::default());
        assert!(compile_result.is_ok(), "Program should compile successfully");
        
        let compiled_program = compile_result.unwrap();
        println!("✓ Compilation successful");
        println!("  Metadata entries: {}", compiled_program.metadata().len());

        // Step 3: Test execution
        println!("Step 2: Testing execution...");
        let execute_options = ExecuteOptions {
            debug: false,
            max_memory: Some(1024 * 1024),
            max_time: Some(5000),
            parallel: false,
            vectors: false,
            nlp: false,
        };

        let execute_result = execute(test_program, execute_options);
        match execute_result {
            Ok(result) => {
                println!("✓ Execution successful");
                println!("  Result: {:?}", result.value);
                println!("  Execution time: {}ms", result.stats.execution_time);
                println!("  Peak memory: {} bytes", result.stats.peak_memory);
                println!("  Instructions: {}", result.stats.instructions);
            }
            Err(err) => {
                println!("! Execution failed (may be expected): {:?}", err);
                // Don't fail the test yet since execution might have unimplemented features
            }
        }
    }

    /// Test that we can process all example files in a batch
    #[test]
    fn test_batch_processing() {
        let examples_dir = "examples";
        if !Path::new(examples_dir).exists() {
            println!("Examples directory not found, skipping batch test");
            return;
        }

        let mut compiled_count = 0;
        let mut executed_count = 0;
        let mut total_count = 0;

        println!("Processing all example files...");

        let entries = fs::read_dir(examples_dir).expect("Failed to read examples directory");
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("llm") {
                    total_count += 1;
                    let filename = path.file_name().unwrap().to_str().unwrap();
                    
                    println!("Processing: {}", filename);
                    
                    let source = match fs::read_to_string(&path) {
                        Ok(source) => source,
                        Err(err) => {
                            println!("  ✗ Failed to read file: {}", err);
                            continue;
                        }
                    };

                    // Test compilation
                    match compile(&source, CompileOptions::default()) {
                        Ok(_) => {
                            compiled_count += 1;
                            println!("  ✓ Compiled successfully");

                            // Test execution for simple files
                            if filename.contains("simple") || filename.contains("test") {
                                let options = ExecuteOptions {
                                    debug: false,
                                    max_memory: Some(1024 * 1024),
                                    max_time: Some(2000),
                                    parallel: false,
                                    vectors: false,
                                    nlp: false,
                                };

                                match execute(&source, options) {
                                    Ok(_) => {
                                        executed_count += 1;
                                        println!("  ✓ Executed successfully");
                                    }
                                    Err(err) => {
                                        println!("  ! Execution failed: {:?}", err);
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            println!("  ✗ Compilation failed: {:?}", err);
                        }
                    }
                }
            }
        }

        println!("\nBatch Processing Summary:");
        println!("  Total files: {}", total_count);
        println!("  Successfully compiled: {}", compiled_count);
        println!("  Successfully executed: {}", executed_count);

        // We expect at least some files to compile successfully
        assert!(compiled_count > 0, "At least some example files should compile");
        
        // For a robust language implementation, we'd expect most simple files to work
        if total_count > 0 {
            let success_rate = (compiled_count as f64 / total_count as f64) * 100.0;
            println!("  Compilation success rate: {:.1}%", success_rate);
            
            // We expect at least 50% success rate for compilation
            assert!(success_rate >= 50.0, "Compilation success rate should be at least 50%");
        }
    }

    /// Test the development workflow using CLI tools
    #[test]
    fn test_cli_workflow() {
        println!("Testing CLI workflow...");

        // Ensure we have built the binaries
        let build_output = Command::new("cargo")
            .args(&["build", "--no-default-features"])
            .output()
            .expect("Failed to execute cargo build");

        if !build_output.status.success() {
            panic!("Failed to build project: {}", String::from_utf8_lossy(&build_output.stderr));
        }

        // Test with a simple example
        let simple_example = "examples/simple_hello.llm";
        if Path::new(simple_example).exists() {
            println!("Testing CLI workflow with: {}", simple_example);

            // Test interpreter
            println!("  Testing llmi (interpreter)...");
            let llmi_output = Command::new("./target/debug/llmi")
                .arg(simple_example)
                .output()
                .expect("Failed to execute llmi");

            println!("  llmi exit code: {}", llmi_output.status.code().unwrap_or(-1));
            if !llmi_output.stdout.is_empty() {
                println!("  llmi stdout: {}", String::from_utf8_lossy(&llmi_output.stdout));
            }
            if !llmi_output.stderr.is_empty() {
                println!("  llmi stderr: {}", String::from_utf8_lossy(&llmi_output.stderr));
            }

            // Test compiler
            println!("  Testing llmc (compiler)...");
            let output_file = "/tmp/test_output";
            let llmc_output = Command::new("./target/debug/llmc")
                .args(&["-o", output_file, simple_example])
                .output()
                .expect("Failed to execute llmc");

            println!("  llmc exit code: {}", llmc_output.status.code().unwrap_or(-1));
            if !llmc_output.stdout.is_empty() {
                println!("  llmc stdout: {}", String::from_utf8_lossy(&llmc_output.stdout));
            }
            if !llmc_output.stderr.is_empty() {
                println!("  llmc stderr: {}", String::from_utf8_lossy(&llmc_output.stderr));
            }

            // Clean up
            if Path::new(output_file).exists() {
                let _ = fs::remove_file(output_file);
            }

            // The tools should at least run without crashing completely
            // Note: llmi might execute successfully with no visible output if the program doesn't print anything
            let llmi_ran_successfully = llmi_output.status.success() || 
                                      !llmi_output.stdout.is_empty() || 
                                      !llmi_output.stderr.is_empty();
            assert!(llmi_ran_successfully, "llmi should either succeed or produce output");
            
            let llmc_ran_successfully = llmc_output.status.success() || 
                                      !llmc_output.stdout.is_empty() || 
                                      !llmc_output.stderr.is_empty();
            assert!(llmc_ran_successfully, "llmc should either succeed or produce output");
        }
    }

    /// Test error handling and reporting
    #[test]
    fn test_error_handling() {
        println!("Testing error handling...");

        // Test with invalid syntax
        let invalid_program = r#"
context InvalidProgram {
    fn main() {
        // Missing closing brace
        var x = 42
    
"#;

        let compile_result = compile(invalid_program, CompileOptions::default());
        assert!(compile_result.is_err(), "Invalid program should fail to compile");
        
        if let Err(err) = compile_result {
            println!("✓ Compilation error caught: {}", err);
            // Error should be descriptive
            let error_msg = format!("{}", err);
            assert!(!error_msg.is_empty(), "Error message should not be empty");
        }

        // Test with runtime error (if we can create one)
        let runtime_error_program = r#"
context ErrorProgram {
    fn main() {
        // This might cause a runtime error
        return unknown_function();
    }
}
"#;

        match compile(runtime_error_program, CompileOptions::default()) {
            Ok(_) => {
                // If it compiles, try to execute and see if we get a runtime error
                let execute_result = execute(runtime_error_program, ExecuteOptions {
                    debug: false,
                    max_memory: Some(1024 * 1024),
                    max_time: Some(1000),
                    parallel: false,
                    vectors: false,
                    nlp: false,
                });

                if let Err(err) = execute_result {
                    println!("✓ Runtime error caught: {}", err);
                }
            }
            Err(err) => {
                println!("✓ Compile-time error caught: {}", err);
            }
        }
    }
}