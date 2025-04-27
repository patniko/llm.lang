# LLM.lang Language Server Protocol Implementation

This is a Language Server Protocol (LSP) implementation for the LLM.lang programming language. It provides IDE features such as code completion, hover information, and more.

## Features

- Syntax highlighting
- Code completion
- Hover information
- Error reporting
- Go to definition
- Find references
- Rename symbols
- Document formatting

## Installation

```bash
# Clone the repository
git clone https://github.com/llm-lang/llm.lang.git
cd llm.lang

# Build the LSP server
cargo build --release -p llm_lang_lsp
```

## Usage

The LSP server can be used with any editor that supports the Language Server Protocol, such as Visual Studio Code, Vim, Emacs, and more.

### Visual Studio Code

1. Install the [LLM.lang extension](https://marketplace.visualstudio.com/items?itemName=llm-lang.llm-lang) from the Visual Studio Code Marketplace.
2. Open a `.llm` file.
3. The LSP server will automatically start and provide IDE features.

### Vim/Neovim

1. Install a Language Server Protocol client for Vim/Neovim, such as [coc.nvim](https://github.com/neoclide/coc.nvim) or [vim-lsp](https://github.com/prabirshrestha/vim-lsp).
2. Configure the client to use the LLM.lang LSP server.

Example configuration for coc.nvim:

```json
{
  "languageserver": {
    "llm-lang": {
      "command": "llm_lang_lsp",
      "filetypes": ["llm"]
    }
  }
}
```

### Emacs

1. Install a Language Server Protocol client for Emacs, such as [lsp-mode](https://github.com/emacs-lsp/lsp-mode).
2. Configure the client to use the LLM.lang LSP server.

Example configuration for lsp-mode:

```elisp
(lsp-register-client
 (make-lsp-client :new-connection (lsp-stdio-connection "llm_lang_lsp")
                  :major-modes '(llm-mode)
                  :server-id 'llm-lang))
```

## Development

To build and run the LSP server during development:

```bash
# Build the LSP server
cargo build -p llm_lang_lsp

# Run the LSP server
cargo run -p llm_lang_lsp
```

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
