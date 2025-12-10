# QMK Language Server (qmk-lsp)

A Language Server Protocol (LSP) implementation for QMK Firmware development, written in Rust.

## Features

- **Autocompletion**: Intelligent suggestions for QMK keycodes, macros, and custom keycodes.
- **Hover Documentation**: Detailed information and documentation for keycodes when hovering.
- **Diagnostics**: Validation and error checking for your keymaps.
- **Parsing**: Supports parsing of `keymap.c` and `info.json` files to understand your keyboard layout.

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

## License

See the [LICENSE](LICENSE) file for details.
