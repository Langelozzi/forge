# Forge ⚒️

A lightweight CLI tool for building and managing C projects. Think `cargo` for C — Forge simplifies project initialization, dependency management, and building with a simple configuration file.

## Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/langelozzi/forge/main/install.sh | sh
```

This will download the latest release for your OS and architecture (macOS or Linux) and install it to your PATH.

## What is Forge?

Forge is a project manager for C that provides:

- **Project Initialization** - Scaffold a new C project with a standard directory structure
- **Build Management** - Compile C projects with a simple `forge.toml` configuration
- **Module Management** - Add and organize source files easily
- **Simple Configuration** - Define your project with a straightforward TOML config file

## Installation

### Option 1: One-liner (Recommended)
```bash
curl -fsSL https://raw.githubusercontent.com/langelozzi/forge/main/install.sh | sh
```

### Option 2: Download from Releases
Visit the [GitHub Releases](https://github.com/langelozzi/forge/releases) page and download the binary for your platform.

### Supported Platforms
- macOS (x86_64, ARM64)
- Linux (x86_64)

## Quick Start

### Initialize a new project
```bash
forge init my-project
cd my-project
```

This creates:
- `src/` - source files directory
- `include/` - header files directory
- `forge.toml` - project configuration

### Build your project
```bash
forge build
```

### Run your project
```bash
forge run
```

### Add a new module
```bash
forge add my_module
```

## Project Configuration

Forge uses `forge.toml` to configure your project:

```toml
[package]
name = "my-project"
version = "0.1.0"
compiler = "gcc"

[build]
flags = ["-Wall", "-Wextra"]
```

## Commands

### `forge init <name>`
Initialize a new C project with the given name.

**Usage:**
```bash
forge init my-app
```

### `forge build`
Compile your project based on the configuration in `forge.toml`.

**Usage:**
```bash
forge build
```

### `forge run`
Build and run your project.

**Usage:**
```bash
forge run
```

### `forge add <module>`
Add a new module/source file to your project.

**Usage:**
```bash
forge add math_utils
```

## Uninstall

Simply delete the binary from your system:
```bash
rm $(which forge)
```

Or if installed via sudo:
```bash
sudo rm $(which forge)
```

## Contributing

Contributions are welcome! Feel free to open issues and pull requests.

## License

MIT
