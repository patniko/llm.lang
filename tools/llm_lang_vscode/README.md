# LLM.lang VS Code Extension

This extension provides language support for the LLM.lang programming language in Visual Studio Code, including syntax highlighting, code completion, debugging, and more.

## Features

### Language Support

- Syntax highlighting for LLM.lang files
- Code completion for LLM.lang keywords, functions, and variables
- Hover information for symbols
- Go to definition
- Find references
- Rename symbols
- Document formatting
- Error reporting

### Debugging

- Set breakpoints
- Step through code (step over, step into, step out)
- Inspect variables
- Evaluate expressions
- View call stack
- Conditional breakpoints
- Logpoints
- Hot code replacement (using LLM.lang's self-modifying capabilities)

## Requirements

- Visual Studio Code 1.60.0 or newer
- LLM.lang Language Server (`llm_lang_lsp`)
- LLM.lang Debug Adapter (`llm_lang_dap`)

## Installation

1. Install the LLM.lang Language Server and Debug Adapter:

```bash
# Clone the repository
git clone https://github.com/llm-lang/llm.lang.git
cd llm.lang

# Build the Language Server and Debug Adapter
cargo build --release -p llm_lang_lsp
cargo build --release -p llm_lang_dap

# Add the binaries to your PATH
export PATH="$PATH:$(pwd)/target/release"
```

2. Install the VS Code extension:

```bash
# Navigate to the extension directory
cd tools/llm_lang_vscode

# Install dependencies
npm install

# Package the extension
npm run vscode:prepublish

# Install the extension
code --install-extension llm-lang-0.1.0.vsix
```

## Usage

### Language Support

1. Open a `.llm` file in VS Code.
2. The language server will automatically start and provide language features.

### Debugging

1. Open a `.llm` file in VS Code.
2. Set breakpoints by clicking in the gutter.
3. Press F5 to start debugging.
4. Use the debug toolbar to control execution (continue, step over, step into, step out).
5. Inspect variables in the Variables panel.
6. Evaluate expressions in the Debug Console.

## Configuration

The extension can be configured using the following settings:

- `llm-lang.lspPath`: Path to the LLM.lang Language Server executable (default: `llm_lang_lsp`)
- `llm-lang.dapPath`: Path to the LLM.lang Debug Adapter executable (default: `llm_lang_dap`)
- `llm-lang.trace.server`: Traces the communication between VS Code and the language server (default: `off`)

## Commands

- `LLM.lang: Restart Language Server`: Restart the language server

## Known Issues

- The debugger is still in development and may not work correctly in all cases.
- Some language features may not be fully implemented yet.

## Release Notes

### 0.1.0

- Initial release
- Basic language support
- Debugging support

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This extension is licensed under the MIT License. See the [LICENSE](../../LICENSE) file for details.
