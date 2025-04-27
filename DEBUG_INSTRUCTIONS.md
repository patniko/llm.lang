# LLM.lang Debugger Instructions

This document provides instructions on how to use the LLM.lang debugger with VSCode.

## Prerequisites

- VSCode installed
- Rust and Cargo installed
- The LLM.lang project cloned and built

## Building the Required Components

Both the debug adapter and the language server need to be built before they can be used:

```bash
# Build the debug adapter
cargo build --release -p llm_lang_dap

# Build the language server
cargo build --release -p llm_lang_lsp
```

This will create the executables at:
- Debug adapter: `target/release/llm_lang_dap`
- Language server: `target/release/llm_lang_lsp`

## Launching VSCode with the LLM.lang Extension

We've provided a script that will launch VSCode with the LLM.lang extension installed and configured to use the debug adapter. To use it, run:

```bash
./launch_vscode.sh
```

This script:
1. Checks if both the debug adapter and language server executables exist
2. Makes sure they're executable
3. Launches VSCode with the LLM.lang extension installed
4. Configures the extension to use both executables

## Debugging an LLM.lang Program

Once VSCode is launched with the LLM.lang extension, you can debug an LLM.lang program by:

1. Opening an LLM.lang file (e.g., `examples/debug_test.llm`)
2. Setting breakpoints by clicking in the gutter to the left of the line numbers
3. Pressing F5 or selecting "Run > Start Debugging" from the menu
4. Selecting "LLM.lang Debug" from the debug configuration dropdown
5. Using the debug controls to step through the program

### Debug Controls

- Continue (F5): Continue execution until the next breakpoint
- Step Over (F10): Execute the current line and stop at the next line
- Step Into (F11): Step into a function call
- Step Out (Shift+F11): Step out of the current function
- Restart (Ctrl+Shift+F5): Restart the debugging session
- Stop (Shift+F5): Stop the debugging session

### Debug Views

- Variables: Shows the current variables in scope
- Watch: Allows you to watch expressions
- Call Stack: Shows the current call stack
- Breakpoints: Shows all breakpoints

## Example Program

We've provided an example program at `examples/debug_test.llm` that you can use to test the debugger. It includes:

- Variable declarations
- Function calls
- Conditional statements
- Loops
- Return statements

## Troubleshooting

If you encounter issues with the debugger:

1. Check the Debug Console for error messages
2. Make sure the debug adapter executable is built and accessible
3. Check the VSCode extension configuration to ensure it's pointing to the correct debug adapter executable
4. Try restarting VSCode

If problems persist, please file an issue on the LLM.lang GitHub repository.
