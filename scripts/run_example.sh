#!/bin/bash
# Script to compile and run LLM.lang examples

set -e  # Exit on error

# Check if Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust and Cargo first."
    exit 1
fi

# Build the project if needed
#if [ ! -f "target/release/llmi" ] || [ ! -f "target/release/llmc" ]; then
echo "Building LLM.lang..."
cargo build --release
#fi

# Add the target/release directory to PATH temporarily
export PATH="$PATH:$(pwd)/target/release"

# Check if an example file was provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <example_file.llm>"
    echo "Available examples:"
    find examples -name "*.llm" | sort
    exit 1
fi

EXAMPLE_FILE="$1"

# Check if the example file exists
if [ ! -f "$EXAMPLE_FILE" ]; then
    echo "Error: Example file '$EXAMPLE_FILE' not found."
    exit 1
fi

# Run the example
echo "Running example: $EXAMPLE_FILE"
echo "----------------------------------------"
llmi "$EXAMPLE_FILE"
echo "----------------------------------------"
echo "Example completed successfully!"

# Optionally compile the example
if [ "$2" == "--compile" ]; then
    OUTPUT_FILE="${EXAMPLE_FILE%.llm}"
    echo "Compiling example to: $OUTPUT_FILE"
    llmc -o "$OUTPUT_FILE" "$EXAMPLE_FILE"
    
    echo "Running compiled example:"
    echo "----------------------------------------"
    "$OUTPUT_FILE"
    echo "----------------------------------------"
    echo "Compiled example completed successfully!"
fi
