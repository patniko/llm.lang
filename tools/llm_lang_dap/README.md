# LLM.lang Debug Adapter Protocol Implementation

This is a Debug Adapter Protocol (DAP) implementation for the LLM.lang programming language. It provides debugging features such as breakpoints, stepping, variable inspection, and more.

## Features

- Set breakpoints
- Step through code (step over, step into, step out)
- Inspect variables
- Evaluate expressions
- View call stack
- Conditional breakpoints
- Logpoints
- Hot code replacement (using LLM.lang's self-modifying capabilities)

## Installation

```bash
# Clone the repository
git clone https://github.com/llm-lang/llm.lang.git
cd llm.lang

# Build the DAP server
cargo build --release -p llm_lang_dap
```

## Usage

The DAP server can be used with any editor that supports the Debug Adapter Protocol, such as Visual Studio Code.

### Visual Studio Code

1. Install the [LLM.lang extension](https://marketplace.visualstudio.com/items?itemName=llm-lang.llm-lang) from the Visual Studio Code Marketplace.
2. Open a `.llm` file.
3. Set breakpoints by clicking in the gutter.
4. Press F5 to start debugging.
5. Use the debug toolbar to control execution (continue, step over, step into, step out).
6. Inspect variables in the Variables panel.
7. Evaluate expressions in the Debug Console.

## Configuration

The DAP server can be configured using a `launch.json` file in VS Code. Here's an example configuration:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "llm-lang",
      "request": "launch",
      "name": "Debug LLM.lang Program",
      "program": "${file}",
      "stopOnEntry": true
    }
  ]
}
```

## Development

To build and run the DAP server during development:

```bash
# Build the DAP server
cargo build -p llm_lang_dap

# Run the DAP server
cargo run -p llm_lang_dap
```

## Integration with Language Server Protocol

The DAP server works alongside the Language Server Protocol (LSP) implementation to provide a complete development experience. The LSP provides features like code completion and hover information, while the DAP provides debugging capabilities.

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
