# LLM.lang Language Specification

This document provides a comprehensive specification of the LLM.lang programming language, including its syntax, semantics, and unique features.

## 1. Introduction

LLM.lang is a programming language specifically designed for Large Language Models (LLMs). It combines the best features of popular programming languages with novel constructs that leverage the unique capabilities of LLMs, creating a language that is both powerful and intuitive for AI-driven development.

### 1.1 Design Goals

The primary design goals of LLM.lang are:

1. **Contextual Awareness**: Enable programs to maintain and switch between different contexts naturally.
2. **Semantic Understanding**: Allow working with meaning rather than just syntax.
3. **Natural Language Integration**: Support expressing complex operations in natural language.
4. **Example-Driven Development**: Enable defining behavior through examples.
5. **Parallel Reasoning**: Support expressing multiple solution approaches simultaneously.
6. **Concept Manipulation**: Facilitate working with semantic embeddings and thought vectors.

### 1.2 Language Paradigms

LLM.lang incorporates multiple programming paradigms:

- **Imperative Programming**: For direct, step-by-step instructions.
- **Functional Programming**: For composable, side-effect-free operations.
- **Declarative Programming**: For specifying what should be accomplished rather than how.
- **Context-Oriented Programming**: For organizing code around contexts.
- **Example-Driven Programming**: For defining behavior through examples.
- **Vector-Based Programming**: For manipulating semantic concepts as vectors.

## 2. Lexical Structure

### 2.1 Character Set

LLM.lang programs are written in Unicode. Source code is interpreted as a sequence of Unicode code points.

### 2.2 Comments

LLM.lang supports two types of comments:

- **Line Comments**: Begin with `//` and continue until the end of the line.
  ```llm
  // This is a line comment
  ```

- **Block Comments**: Begin with `/*` and end with `*/`. They can span multiple lines.
  ```llm
  /* This is a
     block comment */
  ```

### 2.3 Identifiers

Identifiers are used to name variables, functions, contexts, and other program elements. They must start with a letter or underscore, followed by any number of letters, digits, or underscores.

```
identifier = [a-zA-Z_][a-zA-Z0-9_]*
```

### 2.4 Keywords

The following keywords are reserved and cannot be used as identifiers:

```
context  with     within   fn       return   var      const
if       else     when     otherwise for      in       parallel
path     select   fastest  best     all      examples transform
into     apply    to       vector   embed    intent   true
false    null     and      or       not      Int      Float
String   Bool     List     Map      Vector
```

### 2.5 Literals

#### 2.5.1 Integer Literals

Integer literals represent whole numbers. They can be written in decimal, hexadecimal, octal, or binary notation.

```
decimal_integer = [0-9]+
hex_integer = 0x[0-9a-fA-F]+
octal_integer = 0o[0-7]+
binary_integer = 0b[01]+
```

#### 2.5.2 Floating-Point Literals

Floating-point literals represent real numbers. They can include a decimal point and an optional exponent.

```
float = [0-9]+\.[0-9]+([eE][+-]?[0-9]+)?
```

#### 2.5.3 String Literals

String literals represent sequences of characters. They are enclosed in double quotes.

```
string = "[^"]*"
```

Escape sequences can be used to include special characters:

- `\"`: Double quote
- `\\`: Backslash
- `\n`: Newline
- `\r`: Carriage return
- `\t`: Tab
- `\u{XXXX}`: Unicode code point (where XXXX is a hexadecimal number)

#### 2.5.4 Boolean Literals

Boolean literals represent truth values.

```
boolean = true | false
```

#### 2.5.5 Null Literal

The null literal represents the absence of a value.

```
null_literal = null
```

### 2.6 Natural Language Expressions

Natural language expressions allow embedding natural language queries or statements within code. They are enclosed in `#"` and `"#`.

```
natural_language = #"[^"]*"#
```

### 2.7 Semantic Tokens

Semantic tokens are special tokens that have semantic meaning in the language. They start with `@`.

```
semantic_token = @[a-zA-Z_][a-zA-Z0-9_]*
```

Examples include `@remember` and `@recall`.

### 2.8 Semantic Types

Semantic types allow defining types based on semantic meaning rather than structure. They are enclosed in `~`.

```
semantic_type = ~[^~]*~
```

## 3. Types

### 3.1 Basic Types

LLM.lang includes the following basic types:

- **Int**: Integer numbers
- **Float**: Floating-point numbers
- **String**: Text strings
- **Bool**: Boolean values (true or false)

### 3.2 Collection Types

LLM.lang includes the following collection types:

- **List**: Ordered collections of values
- **Map**: Key-value mappings

### 3.3 Special Types

LLM.lang includes the following special types:

- **Vector**: Semantic vector embeddings
- **Context**: Named execution contexts

### 3.4 Semantic Types

Semantic types allow defining types based on semantic meaning rather than structure. They are enclosed in `~`.

```llm
~EmailAddress~ email = "user@example.com";
~PhoneNumber~ phone = "123-456-7890";
```

The type checker ensures that values assigned to semantic types are semantically compatible with the type, even if they have different structures.

### 3.5 Type Inference

LLM.lang supports type inference. The type of a variable can be inferred from its initialization value.

```llm
var x = 42;  // x is inferred to be of type Int
var y = "hello";  // y is inferred to be of type String
```

## 4. Variables and Constants

### 4.1 Variable Declarations

Variables are declared using the `var` keyword, optionally followed by a type annotation.

```llm
var x: Int = 42;
var y = "hello";  // Type is inferred
```

### 4.2 Constant Declarations

Constants are declared using the `const` keyword. They cannot be reassigned after initialization.

```llm
const PI: Float = 3.14159;
const NAME = "LLM.lang";  // Type is inferred
```

### 4.3 Assignment

Variables can be assigned new values using the assignment operator `=`.

```llm
var x = 42;
x = 43;  // Reassignment
```

### 4.4 Compound Assignment

Compound assignment operators combine an operation with assignment.

```llm
var x = 42;
x += 1;  // Equivalent to x = x + 1
x -= 2;  // Equivalent to x = x - 2
x *= 3;  // Equivalent to x = x * 3
x /= 4;  // Equivalent to x = x / 4
x %= 5;  // Equivalent to x = x % 5
```

## 5. Expressions

### 5.1 Arithmetic Expressions

Arithmetic expressions combine numeric values using arithmetic operators.

```llm
var a = 5 + 3;  // Addition
var b = 5 - 3;  // Subtraction
var c = 5 * 3;  // Multiplication
var d = 5 / 3;  // Division
var e = 5 % 3;  // Modulo
```

### 5.2 Comparison Expressions

Comparison expressions compare values and produce boolean results.

```llm
var a = 5 == 3;  // Equal to
var b = 5 != 3;  // Not equal to
var c = 5 < 3;   // Less than
var d = 5 > 3;   // Greater than
var e = 5 <= 3;  // Less than or equal to
var f = 5 >= 3;  // Greater than or equal to
```

### 5.3 Logical Expressions

Logical expressions combine boolean values using logical operators.

```llm
var a = true and false;  // Logical AND
var b = true or false;   // Logical OR
var c = not true;        // Logical NOT
```

### 5.4 Function Calls

Function calls invoke functions with arguments.

```llm
var result = add(5, 3);
```

### 5.5 Natural Language Expressions

Natural language expressions allow embedding natural language queries or statements within code.

```llm
var users = #"Find all users who signed up in the last 7 days"#;
```

### 5.6 Vector Expressions

Vector expressions create and manipulate semantic vector embeddings.

```llm
vector concept = embed("database optimization");
vector combined = concept * 0.7 + embed("query performance") * 0.3;
```

## 6. Statements

### 6.1 Expression Statements

Expression statements evaluate an expression and discard the result.

```llm
print("Hello, World!");
```

### 6.2 Variable Declaration Statements

Variable declaration statements declare variables and optionally initialize them.

```llm
var x: Int = 42;
```

### 6.3 Assignment Statements

Assignment statements assign values to variables.

```llm
x = 43;
```

### 6.4 Block Statements

Block statements group multiple statements together.

```llm
{
    var x = 42;
    print(x);
}
```

### 6.5 If Statements

If statements conditionally execute code based on a boolean expression.

```llm
if (x > 0) {
    print("Positive");
} else if (x < 0) {
    print("Negative");
} else {
    print("Zero");
}
```

### 6.6 When Statements

When statements are similar to if statements but are more concise for pattern matching.

```llm
when (x) {
    > 0 => print("Positive");
    < 0 => print("Negative");
    otherwise => print("Zero");
}
```

### 6.7 For Statements

For statements iterate over collections.

```llm
for (item in items) {
    print(item);
}
```

### 6.8 Return Statements

Return statements return values from functions.

```llm
return x + y;
```

### 6.9 Intent Statements

Intent statements express an intent to perform an action, which is interpreted by the language.

```llm
intent: create a user authentication system;
```

## 7. Functions

### 7.1 Function Declarations

Functions are declared using the `fn` keyword, followed by a name, parameter list, optional return type, and body.

```llm
fn add(a: Int, b: Int) -> Int {
    return a + b;
}
```

### 7.2 Function Calls

Functions are called by name, followed by a comma-separated list of arguments in parentheses.

```llm
var result = add(5, 3);
```

### 7.3 Anonymous Functions

Anonymous functions (lambdas) are functions without names.

```llm
var add = fn(a: Int, b: Int) -> Int {
    return a + b;
};
```

### 7.4 Example-Driven Functions

Functions can be defined by examples, which specify input-output pairs.

```llm
examples for capitalize {
    "hello" -> "Hello";
    "world" -> "World";
}
```

## 8. Contexts

### 8.1 Context Declarations

Contexts are declared using the `context` keyword, followed by a name and body.

```llm
context UserAuthentication {
    var user: User;
    var token: String;
    
    fn authenticate() -> Bool {
        // Authentication logic
    }
}
```

### 8.2 With Statements

With statements create a new context and execute code within it.

```llm
with context "UserAuthentication" {
    var user = authenticateUser();
    var token = generateToken();
}
```

### 8.3 Within Statements

Within statements execute code within an existing context.

```llm
within "UserAuthentication" {
    validateToken(token);
}
```

## 9. Semantic Memory

### 9.1 Remember Statements

Remember statements store values in semantic memory.

```llm
@remember currentUser = getUserProfile();
```

### 9.2 Recall Expressions

Recall expressions retrieve values from semantic memory.

```llm
var profile = @recall;  // Retrieves the most relevant remembered value
var user = @recall("currentUser");  // Retrieves a specific remembered value
```

## 10. Parallel Execution

### 10.1 Parallel Blocks

Parallel blocks execute multiple code paths in parallel.

```llm
var result = parallel {
    path1: {
        return approach1();
    }
    
    path2: {
        return approach2();
    }
} select best;
```

### 10.2 Selection Strategies

Selection strategies determine how to combine the results of parallel execution paths.

- **fastest**: Select the result of the fastest path.
- **best**: Select the best result based on a quality metric.
- **all**: Return all results.

```llm
var result = parallel {
    // ...
} select fastest;
```

## 11. Vector Operations

### 11.1 Vector Creation

Vectors are created using the `embed` function, which converts text to a semantic vector embedding.

```llm
vector concept = embed("database optimization");
```

### 11.2 Vector Arithmetic

Vectors can be combined using arithmetic operations.

```llm
vector combined = concept1 * 0.7 + concept2 * 0.3;
```

### 11.3 Vector Application

Vectors can be applied to code blocks to guide their execution.

```llm
apply combined to {
    // This block is influenced by the combined vector
    optimizeDatabaseQueries();
}
```

## 12. Example-Driven Programming

### 12.1 Example Definitions

Examples define input-output pairs for functions or transformations.

```llm
examples for capitalize {
    "hello" -> "Hello";
    "world" -> "World";
}
```

### 12.2 Transformation Definitions

Transformations define how to convert one value to another.

```llm
transform "hello" into "Hello";
transform "world" into "World";
```

## 13. Natural Language Integration

### 13.1 Natural Language Queries

Natural language queries allow expressing complex operations in natural language.

```llm
var users = #"Find all users who signed up in the last 7 days"#;
```

### 13.2 Intent Statements

Intent statements express an intent to perform an action, which is interpreted by the language.

```llm
intent: create a user authentication system;
```

## 14. Standard Library

### 14.1 Core Functions

- **print**: Print a value to the console.
- **toString**: Convert a value to a string.
- **parseInt**: Parse a string as an integer.
- **parseFloat**: Parse a string as a floating-point number.

### 14.2 Collection Functions

- **length**: Get the length of a collection.
- **isEmpty**: Check if a collection is empty.
- **contains**: Check if a collection contains a value.
- **map**: Apply a function to each element of a collection.
- **filter**: Filter a collection based on a predicate.
- **reduce**: Reduce a collection to a single value.

### 14.3 String Functions

- **substring**: Get a substring of a string.
- **indexOf**: Find the index of a substring.
- **toLowerCase**: Convert a string to lowercase.
- **toUpperCase**: Convert a string to uppercase.
- **trim**: Remove whitespace from the beginning and end of a string.

### 14.4 Vector Functions

- **embed**: Convert text to a semantic vector embedding.
- **similarity**: Calculate the similarity between two vectors.
- **nearest**: Find the nearest vectors to a given vector.

### 14.5 Context Functions

- **currentContext**: Get the current context.
- **switchContext**: Switch to a different context.
- **mergeContexts**: Merge two contexts.

## 15. Grammar

The following is a simplified grammar for LLM.lang in Extended Backus-Naur Form (EBNF):

```ebnf
Program = { ContextDeclaration | FunctionDeclaration | Statement } ;

ContextDeclaration = "context" Identifier "{" { FunctionDeclaration | VariableDeclaration } "}" ;

FunctionDeclaration = "fn" Identifier "(" [ ParameterList ] ")" [ "->" Type ] Block ;

ParameterList = Parameter { "," Parameter } ;

Parameter = Identifier ":" Type ;

Type = "Int" | "Float" | "String" | "Bool" | "List" | "Map" | "Vector" | "Context" | SemanticType ;

SemanticType = "~" Identifier "~" ;

Block = "{" { Statement } "}" ;

Statement = ExpressionStatement
          | VariableDeclaration
          | AssignmentStatement
          | BlockStatement
          | IfStatement
          | WhenStatement
          | ForStatement
          | ReturnStatement
          | WithStatement
          | WithinStatement
          | RememberStatement
          | IntentStatement
          | ParallelStatement
          | ApplyStatement ;

ExpressionStatement = Expression ";" ;

VariableDeclaration = "var" Identifier [ ":" Type ] "=" Expression ";" ;

AssignmentStatement = Identifier "=" Expression ";" ;

BlockStatement = Block ;

IfStatement = "if" "(" Expression ")" Block [ "else" ( IfStatement | Block ) ] ;

WhenStatement = "when" "(" Expression ")" "{" { WhenCase } "}" ;

WhenCase = Expression "=>" Block | "otherwise" "=>" Block ;

ForStatement = "for" "(" Identifier "in" Expression ")" Block ;

ReturnStatement = "return" Expression ";" ;

WithStatement = "with" "context" StringLiteral Block ;

WithinStatement = "within" StringLiteral Block ;

RememberStatement = "@remember" Identifier "=" Expression ";" ;

IntentStatement = "intent" ":" Expression ";" ;

ParallelStatement = "parallel" "{" { "path" Identifier ":" Block } "}" "select" SelectionStrategy ;

SelectionStrategy = "fastest" | "best" | "all" ;

ApplyStatement = "apply" Expression "to" Block ;

Expression = Literal
           | Identifier
           | FunctionCall
           | BinaryExpression
           | UnaryExpression
           | ParenthesizedExpression
           | NaturalLanguageExpression
           | RecallExpression
           | VectorExpression ;

Literal = IntegerLiteral
        | FloatLiteral
        | StringLiteral
        | BooleanLiteral
        | NullLiteral ;

IntegerLiteral = DecimalInteger | HexInteger | OctalInteger | BinaryInteger ;

DecimalInteger = Digit { Digit } ;

HexInteger = "0x" HexDigit { HexDigit } ;

OctalInteger = "0o" OctalDigit { OctalDigit } ;

BinaryInteger = "0b" BinaryDigit { BinaryDigit } ;

FloatLiteral = Digit { Digit } "." Digit { Digit } [ ExponentPart ] ;

ExponentPart = ( "e" | "E" ) [ "+" | "-" ] Digit { Digit } ;

StringLiteral = "\"" { Character } "\"" ;

BooleanLiteral = "true" | "false" ;

NullLiteral = "null" ;

Identifier = Letter { Letter | Digit | "_" } ;

FunctionCall = Identifier "(" [ ArgumentList ] ")" ;

ArgumentList = Expression { "," Expression } ;

BinaryExpression = Expression Operator Expression ;

Operator = "+" | "-" | "*" | "/" | "%" | "==" | "!=" | "<" | ">" | "<=" | ">=" | "and" | "or" ;

UnaryExpression = UnaryOperator Expression ;

UnaryOperator = "-" | "not" ;

ParenthesizedExpression = "(" Expression ")" ;

NaturalLanguageExpression = "#\"" { Character } "\"#" ;

RecallExpression = "@recall" [ "(" StringLiteral ")" ] ;

VectorExpression = "vector" Identifier "=" "embed" "(" StringLiteral ")" ;

Digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;

HexDigit = Digit | "a" | "b" | "c" | "d" | "e" | "f" | "A" | "B" | "C" | "D" | "E" | "F" ;

OctalDigit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" ;

BinaryDigit = "0" | "1" ;

Letter = "a" | "b" | ... | "z" | "A" | "B" | ... | "Z" ;

Character = Letter | Digit | Symbol | Whitespace ;

Symbol = "!" | "@" | "#" | "$" | "%" | "^" | "&" | "*" | "(" | ")" | ... ;

Whitespace = " " | "\t" | "\n" | "\r" ;
```

## 16. Examples

### 16.1 Hello World

```llm
context MainProgram {
    fn main() {
        print("Hello, World!");
    }
}
```

### 16.2 Context Awareness

```llm
context UserAuthentication {
    fn authenticate(username: String, password: String) -> Bool {
        // Authentication logic
        return true;
    }
    
    fn generateToken() -> String {
        return "token123";
    }
}

context MainProgram {
    fn main() {
        with context "UserAuthentication" {
            var authenticated = authenticate("user", "password");
            
            if (authenticated) {
                var token = generateToken();
                @remember userToken = token;
                print("Authentication successful. Token: " + token);
            } else {
                print("Authentication failed.");
            }
        }
        
        // Later, access the remembered token
        var token = @recall("userToken");
        print("Remembered token: " + token);
    }
}
```

### 16.3 Example-Driven Programming

```llm
context MainProgram {
    fn main() {
        // Define a function by examples
        examples for capitalize {
            "hello" -> "Hello";
            "world" -> "World";
            "llm.lang" -> "Llm.lang";
        }
        
        // Use the function
        print(capitalize("hello"));  // Outputs: "Hello"
        print(capitalize("llm.lang"));  // Outputs: "Llm.lang"
    }
}
```

### 16.4 Parallel Execution

```llm
context MainProgram {
    fn main() {
        var result = parallel {
            path1: {
                // First approach: direct calculation
                return 42;
            }
            
            path2: {
                // Second approach: step-by-step calculation
                var step1 = 10 + 10;
                var step2 = step1 * 2;
                return step2 + 2;
            }
        } select fastest;
        
        print("Result: " + toString(result));
    }
}
```

### 16.5 Vector Operations

```llm
context MainProgram {
    fn main() {
        // Create vector embeddings
        vector concept1 = embed("database optimization");
        vector concept2 = embed("query performance");
        
        // Combine vectors
        vector combined = concept1 * 0.7 + concept2 * 0.3;
        
        // Apply the vector to guide content generation
        apply combined to {
            // This block is influenced by the combined vector
            print("To optimize database performance, consider indexing frequently queried columns and analyzing query execution plans.");
        }
    }
}
```

### 16.6 Natural Language Integration

```llm
context MainProgram {
    fn main() {
        // Use natural language to express a query
        var users = #"Find all users who signed up in the last 7 days"#;
        
        // This would be processed by the NLP engine and converted to a query
        print("Query result: " + toString(users));
        
        // Use intent-based programming
        intent: create a greeting message for the user;
        
        // This would generate code based on the intent
        print("Generated greeting: Hello, user! Welcome to LLM.lang!");
    }
}
```

## 17. Future Directions

The LLM.lang language is designed to evolve as LLM capabilities advance. Future directions include:

1. **Enhanced Natural Language Integration**: Deeper integration with natural language processing capabilities.
2. **Advanced Vector Operations**: More sophisticated operations on semantic vectors.
3. **Improved Example-Driven Programming**: Better inference from examples.
4. **Context Hierarchies**: Support for hierarchical contexts.
5. **Interoperability**: Better integration with existing programming languages and ecosystems.
6. **Optimization**: Performance improvements for vector operations and parallel execution.
7. **Tooling**: Development of IDEs, debuggers, and other tools specifically designed for LLM.lang.

## 18. Conclusion

LLM.lang represents a new approach to programming that leverages the unique capabilities of Large Language Models. By combining traditional programming constructs with novel features like contextual awareness, semantic memory, natural language integration, example-driven programming, parallel thought processes, and vector-based concept manipulation, LLM.lang enables developers to create more intuitive, flexible, and powerful programs.

This specification provides a foundation for the LLM.lang language, but the language is expected to evolve as LLM capabilities advance and as developers provide feedback on their experiences with the language.
