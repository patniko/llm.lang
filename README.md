# LLM.lang: A Programming Language Optimized for Large Language Models

LLM.lang is a programming language specifically designed for Large Language Models (LLMs). It combines the best features of popular programming languages with novel constructs that leverage the unique capabilities of LLMs, creating a language that is both powerful and intuitive for AI-driven development.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/llm-lang/llm.lang.git
cd llm.lang

# Build the project
cargo build --release

# Run an example
./scripts/run_example.sh examples/hello_world.llm

# Try the features demonstration
./scripts/run_example.sh examples/llm_features.llm

# Start the interactive interpreter
./target/release/llmi -i
```

For more detailed instructions, see the [Getting Started Guide](docs/getting_started.md).

## Features

### Contextual Awareness

LLM.lang inherently understands and maintains context across the program:

```llm
// Create a context
with context "UserAuthentication" {
    var user = authenticateUser();
    var token = generateToken();
}

// Later, access the context by name
within "UserAuthentication" {
    validateToken(token);  // token is accessible here
}
```

### Semantic Memory

Store and recall values across different parts of your program:

```llm
// Remember a value
@remember currentUser = getUserProfile();

// Later, recall the value without explicit reference
var profile = @recall;  // Retrieves currentUser
```

### Natural Language Integration

Express complex operations using natural language:

```llm
// Natural language query
var users = #"Find all users who signed up in the last 7 days"#;

// Intent-based programming
intent: create a user authentication system;
```

### Example-Driven Programming

Define functions and transformations by examples:

```llm
// Define a function by examples
examples for capitalize {
    "hello" -> "Hello";
    "world" -> "World";
}

// Use the function
print(capitalize("llm.lang"));  // Outputs: "Llm.lang"
```

### Parallel Thought Processes

Express multiple solution approaches simultaneously:

```llm
// Execute multiple approaches in parallel
var result = parallel {
    path1: {
        // First approach
        return approach1();
    }
    path2: {
        // Second approach
        return approach2();
    }
} select best;  // Choose the best result
```

### Vector-Based Concept Manipulation

Work with semantic embeddings and thought vectors:

```llm
// Create vector embeddings
vector concept1 = embed("database optimization");
vector concept2 = embed("query performance");

// Combine vectors
vector combined = concept1 * 0.7 + concept2 * 0.3;

// Apply the vector to guide content generation
apply combined to {
    // This block is influenced by the combined vector
    optimizeDatabaseQueries();
}
```

## Getting Started

### Installation

```bash
# Clone the repository
git clone https://github.com/llm-lang/llm.lang.git
cd llm.lang

# Build the project
cargo build --release

# Add to your PATH
export PATH=$PATH:$(pwd)/target/release
```

### Hello World

Create a file named `hello.llm`:

```llm
context MainProgram {
    fn main() {
        print("Hello, World from LLM.lang!");
    }
}
```

Run the program:

```bash
llmi hello.llm
```

### More Examples

Check out the `examples/` directory for more sample programs:

- `examples/hello_world.llm`: Basic language features
- `examples/applications/data_analysis.llm`: A data analysis application
- `examples/applications/web_server.llm`: A simple web server
- `examples/applications/nlp_processor.llm`: Natural language processing

## Documentation

- [Language Specification](docs/language_specification.md)
- [Implementation Roadmap](docs/implementation_roadmap.md)
- [Core Components](docs/core_components.md)
- [Code Examples](docs/code_examples.md)
- [Standard Library](docs/standard_library.md)
- [Prototype Implementation Plan](docs/prototype_implementation_plan.md)
- [Project Structure](docs/project_structure.md)

## Why LLM.lang?

Traditional programming languages were designed for human programmers, with syntax and semantics optimized for human cognition. LLM.lang takes a different approach, designing a language that leverages the unique capabilities of Large Language Models while remaining readable and usable by humans.

Key advantages include:

1. **Contextual Programming**: Maintain and switch between different contexts naturally
2. **Semantic Understanding**: Work with meaning rather than just syntax
3. **Natural Language Integration**: Express complex operations in natural language
4. **Example-Driven Development**: Define behavior through examples
5. **Parallel Reasoning**: Express multiple solution approaches simultaneously
6. **Concept Manipulation**: Work with semantic embeddings and thought vectors

## Contributing

We welcome contributions to LLM.lang! See [CONTRIBUTING.md](.github/CONTRIBUTING.md) for details on how to get started.

## License

LLM.lang is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments

LLM.lang draws inspiration from many programming languages and paradigms, including:

- Python's readability and simplicity
- JavaScript's flexibility and functional features
- Rust's type system and safety
- Prolog's logic programming
- Lisp's metaprogramming
- Natural language processing techniques
- Vector-based semantic representations

## Contact

For questions, suggestions, or discussions about LLM.lang, please open an issue on GitHub or join our community Discord server.
