# QMK Language Server Client

A Visual Studio Code extension that provides intelligent language support for QMK Firmware development through the QMK Language Server Protocol (LSP) implementation.

## Features

### 🎯 Autocompletion

Get intelligent suggestions while writing your keymaps:
- **QMK Keycodes**: Complete suggestions for all standard QMK keycodes (e.g., `KC_A`, `KC_ESC`, `KC_LCTL`)
- **Macros**: Autocomplete for QMK macro definitions
- **Custom Keycodes**: Support for your custom-defined keycodes

### 📚 Hover Documentation

Hover over any keycode to see detailed information:
- Keycode descriptions and usage
- Documentation directly from QMK documentation
- Quick reference without leaving your editor

### 🔍 Diagnostics

Real-time validation and error checking:
- Syntax validation for keymap files
- Error detection for invalid keycodes
- Warnings for potential issues in your configuration

### 📄 File Support

- **`keymap.c`**: Full parsing and analysis of keymap C files
- **`info.json`**: Support for keyboard metadata and layout definitions

## Installation

### From VS Code Marketplace

1. Open Visual Studio Code
2. Go to Extensions (`Ctrl+Shift+X` or `Cmd+Shift+X`)
3. Search for "QMK Language Server Client"
4. Click **Install**

### From VSIX

Download the `.vsix` file from the [releases page](https://github.com/pedrohsbarbosa99/qmk-language-server/releases) and install it manually:

```bash
code --install-extension qmk-lsp-client-*.vsix
```

## Requirements

The extension requires the `qmk-lsp` language server to be installed and available in your system.

### Installing the Language Server

1. Clone the repository:
   ```bash
   git clone https://github.com/pedrohsbarbosa99/qmk-language-server.git
   cd qmk-language-server
   ```

2. Build the language server:
   ```bash
   cargo build --release
   ```

3. The binary will be available at `target/release/qmk-lsp`

## Configuration

### Setting the Server Path

If the `qmk-lsp` binary is not in your system PATH, you can configure its location:

1. Open VS Code Settings (`Ctrl+,` or `Cmd+,`)
2. Search for "QMK LSP"
3. Set **QMK LSP: Server Path** to the full path of your `qmk-lsp` executable

Alternatively, add this to your `settings.json`:

```json
{
  "qmkLsp.serverPath": "/path/to/qmk-lsp/target/release/qmk-lsp"
}
```

## Usage

Once installed and configured, the extension will automatically activate when you open C or C++ files in your QMK keyboard directory. You'll get:

- Autocompletion as you type
- Hover documentation when you hover over keycodes
- Real-time diagnostics in the Problems panel

## Supported Languages

- C (`.c` files)
- C++ (`.cpp` files)

## Contributing

Contributions are welcome! Please see the [main repository](https://github.com/pedrohsbarbosa99/qmk-language-server) for contribution guidelines.

## Issues

Found a bug or have a feature request? Please open an issue on the [GitHub repository](https://github.com/pedrohsbarbosa99/qmk-language-server/issues).

## License

See the [LICENSE](https://github.com/pedrohsbarbosa99/qmk-language-server/blob/main/LICENSE) file for details.

## Links

- [GitHub Repository](https://github.com/pedrohsbarbosa99/qmk-language-server)
- [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=PedroHenriqueBarbosa.qmk-lsp-client)
- [QMK Firmware Documentation](https://docs.qmk.fm/)
