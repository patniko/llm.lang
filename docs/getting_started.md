# Getting Started with LLM.lang

This guide will help you get started with LLM.lang, a programming language specifically designed for Large Language Models. It combines the best features of popular programming languages with novel constructs that leverage the unique capabilities of LLMs.

## Installation

### Prerequisites

- Rust (1.65 or later)
- Cargo (included with Rust)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/llm-lang/llm.lang.git
cd llm.lang

# Build the project
cargo build --release

# Add to your PATH
export PATH=$PATH:$(pwd)/target/release
```

## Running Your First LLM.lang Program

Create a file named `hello.llm`:

```llm
context MainProgram {
    fn main() {
        print("Hello, World from LLM.lang!");
    }
}
```

Run the program using the LLM.lang interpreter:

```bash
llmi hello.llm
```

You should see the following output:

```
Hello, World from LLM.lang!
```

## Key Features of LLM.lang

LLM.lang introduces several unique features that make it particularly well-suited for AI-driven development:

### 1. Contextual Awareness

Contexts allow organizing code and maintaining state:

```llm
// Create a context
with context "UserProfile" {
    var name = "Alice";
    var age = 30;
    
    // Define a function within this context
    fn formatProfile() -> String {
        return name + " (" + toString(age) + ")";
    }
    
    print(formatProfile());  // Outputs: "Alice (30)"
}

// Later, access the context by name
within "UserProfile" {
    // We can access variables and functions defined in this context
    print(formatProfile());  // Outputs: "Alice (30)"
    
    // Update a variable
    age = 31;
    print(formatProfile());  // Outputs: "Alice (31)"
}
```

### 2. Semantic Memory

Store and recall values across different parts of your program:

```llm
// Remember values in semantic memory
@remember currentUser = "Alice";
@remember userPreferences = ["reading", "hiking", "coding"];
@remember userStats = {
    "loginCount": 42,
    "lastLogin": "2025-04-26",
    "accountType": "premium"
};

// Do some other operations...

// Later, recall the values without explicit references
print("Current user: " + @recall);  // Retrieves the most relevant value (currentUser)
print("User preferences: " + toString(@recall("userPreferences")));
print("User login count: " + toString(@recall("userStats").loginCount));
```

### 3. Natural Language Integration

Express complex operations using natural language:

```llm
// Use natural language to express a query
var activeUsers = #"Find all users who logged in during the last week and have premium accounts"#;

// This would be processed by the NLP engine and converted to a query
print("Query result: " + toString(activeUsers));

// Use intent-based programming
intent: create a personalized recommendation system based on user preferences;

// This would generate code based on the intent
print("Generated recommendation system:");
print("  - Analyzing user preferences");
print("  - Finding similar users");
print("  - Generating recommendations based on collaborative filtering");
```

### 4. Example-Driven Programming

Define functions by examples rather than explicit algorithms:

```llm
// Define a function by examples
examples for pluralize {
    "book" -> "books";
    "box" -> "boxes";
    "child" -> "children";
    "person" -> "people";
    "sheep" -> "sheep";
}

// Use the function defined by examples
print(pluralize("book"));    // Outputs: "books"
print(pluralize("child"));   // Outputs: "children"
print(pluralize("sheep"));   // Outputs: "sheep"

// The function generalizes to new inputs
print(pluralize("computer")); // Outputs: "computers"
print(pluralize("city"));     // Outputs: "cities"
```

### 5. Parallel Thought Processes

Express multiple solution approaches simultaneously:

```llm
// Define a problem to solve
var numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
print("Finding the sum of squares for " + toString(numbers));

// Execute multiple approaches in parallel
var result = parallel {
    path1: {
        // First approach: direct calculation
        print("Path 1: Direct calculation");
        var sum = 0;
        for (num in numbers) {
            sum += num * num;
        }
        return sum;
    }
    
    path2: {
        // Second approach: functional style
        print("Path 2: Functional style");
        return numbers.map(n => n * n).reduce((a, b) => a + b, 0);
    }
    
    path3: {
        // Third approach: mathematical formula
        print("Path 3: Mathematical formula");
        var n = numbers.length;
        return (n * (n + 1) * (2 * n + 1)) / 6;
    }
} select best;

print("Result: " + toString(result));
```

### 6. Vector-Based Concept Manipulation

Work with semantic embeddings as first-class citizens:

```llm
// Create vector embeddings from text
vector programming = embed("computer programming software development coding");
vector database = embed("database SQL queries tables records");
vector ai = embed("artificial intelligence machine learning neural networks");

// Combine vectors with different weights
vector combined = programming * 0.5 + database * 0.3 + ai * 0.2;

// Apply the vector to guide content generation
apply combined to {
    // This block is influenced by the combined vector
    print("Developing a database-backed application that incorporates machine learning");
    print("requires understanding how to efficiently store and query data while");
    print("integrating AI models into your software architecture.");
}

// Calculate similarity between vectors
var sim = similarity(programming, ai);
print("Similarity between programming and AI: " + toString(sim));
```

## Complete Example

Here's a complete example that demonstrates all the key features of LLM.lang:

```llm
// llm_features.llm - A demonstration of LLM.lang's unique features

context MainProgram {
    // Main function - entry point of the program
    fn main() {
        print("=== LLM.lang Features Demonstration ===\n");
        
        // Demonstrate contextual awareness
        demonstrateContexts();
        
        // Demonstrate semantic memory
        demonstrateSemanticMemory();
        
        // Demonstrate natural language integration
        demonstrateNaturalLanguage();
        
        // Demonstrate example-driven programming
        demonstrateExampleDriven();
        
        // Demonstrate parallel execution
        demonstrateParallelExecution();
        
        // Demonstrate vector operations
        demonstrateVectorOperations();
        
        print("\nAll features demonstrated successfully!");
    }
    
    // Demonstrate contextual awareness
    fn demonstrateContexts() {
        print("\n=== Contextual Awareness ===");
        
        // Create a new context
        with context "UserProfile" {
            var name = "Alice";
            var age = 30;
            var preferences = ["reading", "hiking", "coding"];
            
            print("Created user profile for " + name);
            
            // Define a function within this context
            fn formatProfile() -> String {
                return name + " (" + toString(age) + ") likes " + toString(preferences);
            }
            
            // Call the function
            print("Profile: " + formatProfile());
        }
        
        // Later, access the context by name
        within "UserProfile" {
            // We can access variables and functions defined in this context
            print("Accessing user profile again: " + formatProfile());
            
            // Update a variable
            age = 31;
            print("Updated profile: " + formatProfile());
        }
    }
    
    // Demonstrate semantic memory
    fn demonstrateSemanticMemory() {
        print("\n=== Semantic Memory ===");
        
        // Remember values in semantic memory
        @remember currentUser = "Alice";
        @remember userPreferences = ["reading", "hiking", "coding"];
        @remember userStats = {
            "loginCount": 42,
            "lastLogin": "2025-04-26",
            "accountType": "premium"
        };
        
        print("Stored user information in semantic memory");
        
        // Do some other operations...
        print("Performing other operations...");
        
        // Later, recall the values without explicit references
        print("Current user: " + @recall);  // Retrieves the most relevant value (currentUser)
        print("User preferences: " + toString(@recall("userPreferences")));
        print("User login count: " + toString(@recall("userStats").loginCount));
    }
    
    // Demonstrate natural language integration
    fn demonstrateNaturalLanguage() {
        print("\n=== Natural Language Integration ===");
        
        // Use natural language to express a query
        var activeUsers = #"Find all users who logged in during the last week and have premium accounts"#;
        
        // This would be processed by the NLP engine and converted to a query
        print("Query result: " + toString(activeUsers));
        
        // Use intent-based programming
        intent: create a personalized recommendation system based on user preferences;
        
        // This would generate code based on the intent
        print("Generated recommendation system:");
        print("  - Analyzing user preferences");
        print("  - Finding similar users");
        print("  - Generating recommendations based on collaborative filtering");
    }
    
    // Demonstrate example-driven programming
    fn demonstrateExampleDriven() {
        print("\n=== Example-Driven Programming ===");
        
        // Define a function by examples
        examples for pluralize {
            "book" -> "books";
            "box" -> "boxes";
            "child" -> "children";
            "person" -> "people";
            "sheep" -> "sheep";
        }
        
        // Use the function defined by examples
        print("Pluralize 'book': " + pluralize("book"));
        print("Pluralize 'child': " + pluralize("child"));
        print("Pluralize 'sheep': " + pluralize("sheep"));
        
        // The function generalizes to new inputs
        print("Pluralize 'computer': " + pluralize("computer"));
        print("Pluralize 'city': " + pluralize("city"));
        
        // Define a transformation by examples
        examples for titleCase {
            "hello world" -> "Hello World";
            "llm.lang features" -> "Llm.lang Features";
            "example-driven programming" -> "Example-Driven Programming";
        }
        
        print("Title case 'hello world': " + titleCase("hello world"));
        print("Title case 'natural language processing': " + titleCase("natural language processing"));
    }
    
    // Demonstrate parallel execution
    fn demonstrateParallelExecution() {
        print("\n=== Parallel Execution ===");
        
        // Define a problem to solve
        var numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        print("Finding the sum of squares for " + toString(numbers));
        
        // Execute multiple approaches in parallel
        var result = parallel {
            path1: {
                // First approach: direct calculation
                print("Path 1: Direct calculation");
                var sum = 0;
                for (num in numbers) {
                    sum += num * num;
                }
                return sum;
            }
            
            path2: {
                // Second approach: functional style
                print("Path 2: Functional style");
                return numbers.map(n => n * n).reduce((a, b) => a + b, 0);
            }
            
            path3: {
                // Third approach: mathematical formula
                print("Path 3: Mathematical formula");
                var n = numbers.length;
                return (n * (n + 1) * (2 * n + 1)) / 6;
            }
        } select best;
        
        print("Result: " + toString(result));
    }
    
    // Demonstrate vector operations
    fn demonstrateVectorOperations() {
        print("\n=== Vector Operations ===");
        
        // Create vector embeddings from text
        vector programming = embed("computer programming software development coding");
        vector database = embed("database SQL queries tables records");
        vector ai = embed("artificial intelligence machine learning neural networks");
        
        print("Created vector embeddings for different concepts");
        
        // Combine vectors with different weights
        vector combined = programming * 0.5 + database * 0.3 + ai * 0.2;
        
        print("Combined vector: programming (50%) + database (30%) + AI (20%)");
        
        // Apply the vector to guide content generation
        print("\nGenerated content based on the combined vector:");
        
        apply combined to {
            // This block is influenced by the combined vector
            print("Developing a database-backed application that incorporates machine learning");
            print("requires understanding how to efficiently store and query data while");
            print("integrating AI models into your software architecture. Consider using");
            print("an ORM framework to bridge the gap between your application code and");
            print("the database, while leveraging ML libraries for the AI components.");
        }
        
        // Calculate similarity between vectors
        var sim1 = similarity(programming, database);
        var sim2 = similarity(programming, ai);
        var sim3 = similarity(database, ai);
        
        print("\nSimilarity between concepts:");
        print("Programming <-> Database: " + toString(sim1));
        print("Programming <-> AI: " + toString(sim2));
        print("Database <-> AI: " + toString(sim3));
    }
}
```

To run this example:

```bash
llmi examples/llm_features.llm
```

## Interactive Mode

LLM.lang also provides an interactive REPL (Read-Eval-Print Loop) mode, which allows you to experiment with the language interactively:

```bash
llmi -i
```

This will start an interactive session where you can enter LLM.lang code and see the results immediately:

```
LLM.lang Interactive Interpreter v0.1.0
Type 'exit' or 'quit' to exit, 'help' for help.
>>> print("Hello, World!")
Hello, World!
>>> var x = 42
>>> print("The answer is " + toString(x))
The answer is 42
>>> exit
Goodbye!
```

## Compiling LLM.lang Programs

LLM.lang includes a compiler that can compile your programs to native executables:

```bash
llmc -o hello examples/hello_world.llm
```

This will compile `hello_world.llm` to an executable named `hello`.

## Next Steps

Now that you've learned the basics of LLM.lang, you can:

1. Explore the [examples](../examples/) directory for more sample programs
2. Read the [language specification](language_specification.md) for a comprehensive reference
3. Check out the [standard library](standard_library.md) documentation
4. Learn about [advanced features](advanced_features.md)

Happy coding with LLM.lang!
