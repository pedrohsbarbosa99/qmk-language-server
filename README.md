# QMK Language Server (qmk-lsp)

A Language Server Protocol (LSP) implementation for QMK Firmware development, written in Rust.

## Features

- **Autocompletion**: Intelligent suggestions for QMK keycodes, macros, and custom keycodes.
- **Hover Documentation**: Detailed information and documentation for keycodes when hovering.
- **Diagnostics**: Validation and error checking for your keymaps.
- **Parsing**: Supports parsing of `keymap.c` and `info.json` files to understand your keyboard layout.

## Editor Support

### Visual Studio Code

Install the official VSCode extension from the marketplace:

[![VS Code Marketplace](https://img.shields.io/visual-studio-marketplace/v/PedroHenriqueBarbosa.qmk-lsp-client?style=flat-square&label=VS%20Code%20Marketplace&logo=visual-studio-code)](https://marketplace.visualstudio.com/items?itemName=PedroHenriqueBarbosa.qmk-lsp-client)

**Installation:**
1. Open VS Code
2. Go to Extensions (`Ctrl+Shift+X`)
3. Search for "QMK Language Server Client"
4. Click Install

See the [extension README](extensions/vscode/client/README.md) for detailed configuration and usage instructions.

### Other Editors

The language server follows the LSP specification and can be integrated with any LSP-compatible editor. Configure your editor to use the `qmk-lsp` binary.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (and Cargo)

### Build from Source

Clone the repository and build the project using Cargo:

```bash
cd qmk-lsp
cargo build --release
```

The binary will be available at `target/release/qmk-lsp`.

## Usage

Configure your LSP client to use the `qmk-lsp` binary.

### Generic Client Configuration

Point your LSP client to the executable: `path/to/qmk-lsp/target/release/qmk-lsp`.

## Contributing

Contributions are welcome! Here's how you can help improve the QMK Language Server:

### Reporting Issues

If you encounter a bug or have a feature request:
1. Check if the issue already exists in the [issue tracker](https://github.com/pedrohsbarbosa99/qmk-language-server/issues)
2. If not, create a new issue with:
   - A clear description of the problem or feature
   - Steps to reproduce (for bugs)
   - Your environment details (OS, editor, QMK version)
   - Relevant code samples or error messages

### Development Setup

1. **Fork and clone the repository:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/qmk-language-server.git
   cd qmk-language-server
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Run tests:**
   ```bash
   cargo test
   ```

4. **Build the VSCode extension (optional):**
   ```bash
   cd extensions/vscode/client
   npm install
   npm run compile
   ```

### Submitting Pull Requests

1. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and ensure:
   - Code follows Rust best practices
   - All tests pass (`cargo test`)
   - New features include tests
   - Code is properly formatted (`cargo fmt`)
   - No clippy warnings (`cargo clippy`)

3. Commit your changes with clear, descriptive messages:
   ```bash
   git commit -m "Add feature: description of your changes"
   ```

4. Push to your fork and submit a pull request:
   ```bash
   git push origin feature/your-feature-name
   ```

5. In your pull request description:
   - Explain what changes you made and why
   - Reference any related issues
   - Include screenshots/examples if applicable

### Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Write clear, self-documenting code
- Add comments for complex logic
- Keep functions focused and modular

### Areas for Contribution

- **Language Features**: Implement additional LSP features (go-to-definition, find references, etc.)
- **QMK Support**: Expand keycode coverage and QMK feature support
- **Editor Extensions**: Create extensions for other editors (Neovim, Emacs, etc.)
- **Documentation**: Improve documentation and examples
- **Testing**: Add more test coverage

## License

See the [LICENSE](LICENSE) file for details.
