# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`saba` is a declarative development framework written in Rust that generates code based on YAML specifications. It follows Domain-Driven Design (DDD) with onion architecture and supports multiple programming languages including Rust, Go, Python, TypeScript, Bash, and Lua.

## Architecture

The project follows DDD with onion architecture:

- **Domain Layer** (`src/domain/`): Core business models and entities
  - `model/manifest.rs`: Central manifest model defining project specifications
- **Infrastructure Layer** (`src/infrastructure/`): External concerns and data access
  - `filesystem/manifest.rs`: File system operations for manifest handling
- **Use Case Layer** (`src/usecase/`): Application business logic
  - `gen_file/`: Code generation logic for different languages (rust, golang, python, typescript, bash, lua)
  - Each language has its own module with `gen_file.rs`, `template.rs`, and utilities
- **Presentation Layer** (`src/presentation/`): CLI interface
  - `command/new.rs`: Interactive project initialization
  - `command/up.rs`: Code generation execution

## Key Commands

### Development
```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Build and copy to _test directory (for testing)
./build.sh

# Run the application
cargo run -- <subcommand>
```

### Application Usage
```bash
# Initialize new project with interactive prompts
saba new

# Generate code from saba.yml specification
saba up
```

### Testing
The project currently has no unit tests defined (`#[test]` or `#[cfg(test)]` not found in codebase).

## Configuration

- **saba.yml**: Project specification file that defines:
  - `lang`: Programming language (rust, go, python, typescript, bash, lua)
  - `arch`: Architecture type (ddd for Domain-Driven Design)
  - `spec`: Hierarchical structure defining modules, upstreams, and codefiles

## Code Generation Flow

1. `saba new` creates a `saba.yml` specification file through interactive prompts
2. `saba up` reads the specification and generates code using language-specific generators
3. Each language generator follows the same pattern: parse manifest → generate files using templates

## Template System

Uses Askama templating engine for code generation. Templates are embedded in the respective language modules under `usecase/gen_file/<lang>/template.rs`.

## Dependencies

Key dependencies include:
- `clap`: CLI argument parsing
- `anyhow`: Error handling
- `askama`: Template engine
- `inquire`: Interactive prompts
- `regex`: Pattern matching
- `yaml-rust`: YAML parsing