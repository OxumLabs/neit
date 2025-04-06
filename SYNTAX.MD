# Neit Programming Language - Syntax Guide

---

> [!NOTE]
> This documentation is a work in progress and may be updated as the language evolves.

---

## Command Line Interface

Neit provides a robust command-line interface (CLI) for compiling and testing your code. While project directory compilation is planned for future releases, the current CLI offers comprehensive options for file-based operations. This section provides a detailed overview of the CLI commands and their associated options.

### Basic Command Structure

The general structure for using the Neit CLI is as follows:

```bash
neit <command> <file> [options]
```

- `<command>`: Specifies the action to perform (e.g., `build`, `help`).
- `<file>`: The source file to operate on.
- `[options]`: Optional flags or parameters to customize the behavior of the command.

### Available Commands

#### `build`
Compiles the specified source file into an executable binary.

#### `help`
Displays detailed usage information and examples for the available commands.

### Build Options

The `build` command supports several options to customize the compilation process:

#### Compiler Selection
`--cc=<compiler>`

- **Description**: Specifies the C compiler to use for building the executable.
- **Supported Compilers**: `zig`, `clang`, `gcc`
- **Default Behavior**: The compiler is selected in the following order: `zig` → `clang` → `gcc`.
- **Additional Notes**: 
    - If `zig` is not found in the system PATH, the CLI searches for it in `./zig/zig` (or `./zig/zig.exe` on Windows).

#### Output Control
`--out=<filename>`

- **Description**: Sets the name of the output binary file.
- **Default Value**: `output`

#### Build Mode
`--static`

- **Description**: Enables static compilation, which embeds all dependencies into the binary.
- **Effectiveness**: The success of static compilation depends on the selected C compiler and its configuration.

#### Target Platform
`--target=<platform>`

- **Description**: Specifies the target platform for the build.
- **Supported Platforms**:
    - Windows x86-64: `windows-x86-64` or `winx8664`
    - Linux x86-64: `linux-x86-64` or `linx8664`
- **Cross-Compilation Notes**:
    - Cross-compilation requires appropriate toolchains.
    - For example, building Windows binaries on Linux requires MinGW, except when using Zig, which handles cross-compilation seamlessly.

---
## Syntax

The Neit programming language features a concise and expressive syntax. This section provides an overview of its key constructs.

### Variable Declaration

Variables in Neit can be declared using the `may` keyword for mutable variables or the `const` keyword for immutable variables.

```neit
may age = 0
const tst = "1"(i32)
```

- `may`: Declares a mutable variable that can be reassigned.
- `const`: Declares an immutable variable whose value cannot be changed after initialization.
- **Type Casting**: The syntax `"value"(type)` is used for explicit type casting.

### Loops and Conditionals

Neit supports `while` loops and `if` conditionals for control flow.

```neit
while age != 1000 {
    if age == 50 {
        println half way there!
    }
}
```

- `while`: Executes the block as long as the condition evaluates to `true`.
- `if`: Executes the block if the condition evaluates to `true`.

### Comments

Comments in Neit can be single-line or multi-line.

```neit
##
This is a
multiline comment
##
# This is a single-line comment
```

- `## ... ##`: Denotes a multi-line comment.
- `# ...`: Denotes a single-line comment.

### Printing

The `print` statement outputs text to the console without appending a newline.

```neit
print Hello, without a new line!
```

- **Note**: For printing with a newline, use `println`.

---

This guide provides a foundational understanding of the Neit programming language and its CLI. As the language evolves, additional features and enhancements will be documented here.
