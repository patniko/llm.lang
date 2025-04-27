#!/bin/bash

# Get the absolute path to the project directory
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Get the absolute paths to the executables
DAP_PATH="$PROJECT_DIR/target/release/llm_lang_dap"
LSP_PATH="$PROJECT_DIR/target/release/llm_lang_lsp"

# Check if the executables exist
if [ ! -f "$DAP_PATH" ]; then
    echo "Error: Debug adapter executable not found at $DAP_PATH"
    echo "Please build the debug adapter first with: cargo build --release -p llm_lang_dap"
    exit 1
fi

if [ ! -f "$LSP_PATH" ]; then
    echo "Error: Language server executable not found at $LSP_PATH"
    echo "Please build the language server first with: cargo build --release -p llm_lang_lsp"
    exit 1
fi

# Make sure the executables are executable
chmod +x "$DAP_PATH"
chmod +x "$LSP_PATH"

# Create a settings.json file with the correct paths
mkdir -p "$PROJECT_DIR/.vscode-data/User"
cat > "$PROJECT_DIR/.vscode-data/User/settings.json" << EOF
{
    "llm-lang.dapPath": "$DAP_PATH",
    "llm-lang.lspPath": "$LSP_PATH"
}
EOF

# Launch VSCode with the correct configuration
code --user-data-dir="$PROJECT_DIR/.vscode-data" \
     --extensions-dir="$PROJECT_DIR/.vscode-extensions" \
     --install-extension "$PROJECT_DIR/tools/llm_lang_vscode/llm-lang-0.1.0.vsix" \
     "$PROJECT_DIR"

echo "VSCode launched with LLM.lang extension and debug adapter configured."
