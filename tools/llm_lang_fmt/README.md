# LLM.lang Code Formatter

This is a code formatter for the LLM.lang programming language. It automatically formats LLM.lang code according to a consistent style.

## Features

- Consistent indentation (spaces or tabs)
- Consistent spacing around operators
- Consistent spacing around punctuation
- Consistent line breaks
- Consistent brace style
- Consistent comment style

## Installation

```bash
# Clone the repository
git clone https://github.com/llm-lang/llm.lang.git
cd llm.lang

# Build the formatter
cargo build --release -p llm_lang_fmt
```

## Usage

```bash
# Format a file and print the result to stdout
llm_lang_fmt path/to/file.llm

# Format a file and write the result back to the file
llm_lang_fmt --write path/to/file.llm

# Check if a file is already formatted
llm_lang_fmt --check path/to/file.llm

# Use tabs for indentation
llm_lang_fmt --indent-style tabs path/to/file.llm

# Use 2 spaces for indentation
llm_lang_fmt --indent-style spaces --indent-size 2 path/to/file.llm
```

## Configuration

You can configure the formatter using command-line options or a configuration file.

### Command-Line Options

- `--write`: Write the formatted code back to the input file
- `--check`: Check if the file is already formatted
- `--indent-style`: The indentation style (spaces or tabs)
- `--indent-size`: The indentation size (number of spaces or tab width)
- `--verbose`: Print verbose output

### Configuration File

You can create a `.llm_fmt.json` file in your project directory to configure the formatter:

```json
{
  "indentStyle": "spaces",
  "indentSize": 4,
  "lineWidth": 100,
  "braceStyle": "same-line",
  "trailingComma": true
}
```

## Integration with Editors

### Visual Studio Code

1. Install the [LLM.lang extension](https://marketplace.visualstudio.com/items?itemName=llm-lang.llm-lang) from the Visual Studio Code Marketplace.
2. Enable the "Format on Save" option in the settings.
3. The formatter will automatically format your code when you save a `.llm` file.

### Vim/Neovim

1. Install a plugin that supports external formatters, such as [neoformat](https://github.com/sbdchd/neoformat) or [vim-autoformat](https://github.com/Chiel92/vim-autoformat).
2. Configure the plugin to use the LLM.lang formatter.

Example configuration for neoformat:

```vim
let g:neoformat_llm_llmfmt = {
    \ 'exe': 'llm_lang_fmt',
    \ 'args': ['--write'],
    \ 'replace': 1,
    \ 'stdin': 0,
    \ 'valid_exit_codes': [0],
    \ }

let g:neoformat_enabled_llm = ['llmfmt']
```

### Emacs

1. Install a package that supports external formatters, such as [format-all](https://github.com/lassik/emacs-format-all-the-code).
2. Configure the package to use the LLM.lang formatter.

Example configuration for format-all:

```elisp
(define-format-all-formatter llm-fmt
  (:executable "llm_lang_fmt")
  (:install "cargo install llm_lang_fmt")
  (:languages "LLM")
  (:format (format-all--buffer-easy executable)))
```

## Development

To build and run the formatter during development:

```bash
# Build the formatter
cargo build -p llm_lang_fmt

# Run the formatter
cargo run -p llm_lang_fmt -- path/to/file.llm
```

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
