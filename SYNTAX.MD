# Neit Programming Language - Syntax Guide
---
> [!NOTE]
> This documentation is a work in progress and may be updated as the language evolves.
---
---
## Command Line Interface

Neit provides a robust command-line interface for compiling and testing your code. While project directory compilation is planned for future releases, the current CLI offers comprehensive options for file operations.

### Basic Command Structure
```bash
neit <command> <file> [options]
```

### Available Commands
- `build` - Compiles source files into executables
- `help`  - Displays command usage and help information

### Build Options

#### Compiler Selection
`--cc=<compiler>`
- Supported compilers: `zig`, `clang`, `gcc`
- Default: Tries compilers in order (zig → clang → gcc)
- Note: Zig binary is searched in `./zig/zig` (or `./zig/zig.exe` on Windows) after it is not found on PATH

#### Output Control
`--out=<filename>`
- Specifies output binary name
- Default: `output`

#### Build Mode
`--static`
- Enables static compilation
- Effectiveness depends on chosen C compiler

#### Target Platform
`--target=<platform>`
- Supported platforms:
    - Windows x86-64: `windows-x86-64` or `winx8664`
    - Linux x86-64: `linux-x86-64` or `linx8664`
- Note: Cross-compilation requires appropriate toolchains (except when using Zig)
    - Windows builds on Linux require MinGW
---