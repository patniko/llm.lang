# Test Infrastructure for LLM.lang

This document describes the test infrastructure that ensures sample code compiles and runs correctly.

## Overview

The test suite verifies that:
1. Example .llm files can be compiled successfully
2. Example .llm files can be executed without crashes
3. CLI tools (llmc and llmi) work correctly with example files
4. The entire toolchain works end-to-end

## Important Note

**To run the tests successfully, you need to use the `--no-default-features` flag to avoid heavy ML dependencies (PyTorch, etc.) that require external system libraries.**

```bash
cargo test --no-default-features
```

This disables features like `rust-bert`, `faiss`, and other ML libraries that require system dependencies to be installed.

## Test Structure

### Unit Tests (`src/`)
- Located within the source files using `#[cfg(test)]`
- Test individual components like lexer, parser, runtime, etc.
- Run with: `cargo test --no-default-features --lib`

### Integration Tests (`tests/`)

#### `sample_code_tests.rs`
Tests the core compilation and execution functionality:
- **`test_compile_simple_examples`**: Verifies simple examples compile
- **`test_execute_simple_examples`**: Verifies simple examples execute
- **`test_compile_debug_example`**: Tests more complex example compilation
- **`test_execute_debug_example`**: Tests complex example execution
- **`test_all_examples_parse`**: Ensures all examples are syntactically valid
- **`test_minimal_example`**: Tests a basic inline example

#### `cli_integration_tests.rs`
Tests the CLI tools:
- **`test_llmi_simple_examples`**: Tests the interpreter with examples
- **`test_llmc_simple_examples`**: Tests the compiler with examples
- **`test_run_example_script`**: Tests the convenience script

## Example Files Tested

### Simple Examples (Must Pass)
- `examples/simple_hello.llm` - Basic hello world
- `examples/simple_print.llm` - Simple print statement
- `examples/test.llm` - Minimal test
- `examples/test_lexer.llm` - Lexer test

### Complex Examples (May Fail Due to Unimplemented Features)
- `examples/debug_test.llm` - Variables, conditionals, loops
- `examples/hello_world.llm` - Full feature demonstration
- `examples/llm_features.llm` - Advanced language features

## Running Tests

### All Tests
```bash
cargo test --no-default-features
```

### Sample Code Tests Only
```bash
cargo test --no-default-features sample_code_tests
```

### CLI Integration Tests Only
```bash
cargo test --no-default-features cli_integration_tests
```

### With Output
```bash
cargo test --no-default-features -- --nocapture
```

## Test Configuration

### Dependencies
The tests run with minimal dependencies by using `--no-default-features` to avoid heavy ML libraries that require external system dependencies.

### Timeouts and Limits
- Memory limit: 1MB for simple tests, 2MB for complex tests
- Time limit: 5-10 seconds for execution
- Compilation should complete quickly

### Error Handling
- Simple examples are expected to compile and run successfully
- Complex examples may fail due to unimplemented features (acceptable for now)
- CLI tools should produce meaningful output even on failure

## Adding New Tests

### For New Example Files
1. Add the file path to the appropriate test array
2. Specify expected behavior (compile, execute, or both)
3. Consider adding it to CLI integration tests

### For New Features
1. Create unit tests in the relevant source files
2. Add integration tests if the feature affects end-to-end functionality
3. Update this documentation

## Test Results Interpretation

### Success Indicators
- ✓ Test passes completely
- Output shows meaningful execution results
- No crashes or panics

### Expected Failures
- Complex features not yet implemented
- Advanced language constructs (parallel, vectors, NLP)
- File I/O and system interactions

### Actual Failures
- Simple examples failing to compile
- Crashes during execution
- CLI tools not producing any output

## Future Improvements

1. **Output Verification**: Capture and verify actual program output
2. **Performance Tests**: Add benchmarks for compilation and execution speed
3. **Error Message Quality**: Verify error messages are helpful
4. **Cross-platform Testing**: Test on different operating systems
5. **Continuous Integration**: Set up automated testing