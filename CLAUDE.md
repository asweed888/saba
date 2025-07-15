# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`saba` is a declarative development framework written in Rust that generates code based on YAML specifications. The current v2 implementation supports multiple programming languages including Rust, Go, Python, TypeScript, JavaScript, and generates project structures based on simple YAML configurations.

## Architecture (v2)

The v2 implementation follows a clean architecture pattern:

- **CLI Layer** (`src/cli/`): Command-line interface
  - `command/new.rs`: Interactive project initialization
  - `command/up.rs`: Code generation execution
  - `command/analyze.rs`: Project analysis features
- **Code Generation Layer** (`src/code_generation/`): Core generation logic
  - `core/`: Core builders (directory_builder.rs, file_builder.rs, generator.rs)
  - `language/`: Language-specific generators (rust, go, python, typescript, javascript)
  - `service/`: Generation services
- **Project Management** (`src/project_management/`): Configuration handling
  - `config/`: YAML configuration parsing and validation
  - `repository/`: Configuration repository pattern
  - `service/`: Project management services
- **Claude Code Integration** (`src/claude_code_integration/`): AI assistance features
- **Shared** (`src/shared/`): Common utilities and error handling

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

## Configuration (v2)

- **saba.yml**: Project specification file that defines:
  - `name`: Project name
  - `root`: Boolean indicating if this is the root project
  - `lang`: Programming language (rust, go, python, typescript, javascript)
  - `upstream`: Hierarchical module structure
  - `codefile`: Individual code files within modules

Example:
```yaml
- name: my-project
  root: true
  lang: typescript
  upstream:
    - name: src
      upstream:
        - name: components
          codefile:
            - name: Button.tsx    # Extension preserved
            - name: Modal.vue     # Extension preserved
            - name: utils         # Gets .ts extension
```

## Code Generation Flow (v2)

1. `saba new` creates a `saba.yml` specification file through interactive prompts
2. `saba up` reads the specification and generates code using language-specific generators
3. Each language generator creates:
   - Directory structure based on module hierarchy
   - Code files with appropriate extensions
   - Language-specific project files (e.g., Cargo.toml for Rust, package.json for TypeScript)

## Language-Specific Features

### Rust
- Generates `mod.rs`, `main.rs`, or `lib.rs` with proper module declarations
- Supports `pub` visibility controls
- Creates `Cargo.toml` with project metadata

### TypeScript
- Generates `index.ts` files with barrel exports for each module
- Creates `package.json` and `tsconfig.json` configuration files
- **File Extension Priority**: If `codefile.name` contains a dot (.), the existing extension is preserved (.tsx, .vue, .jsx, etc.). Otherwise, adds .ts extension.
- Barrel exports remove extensions for proper TypeScript module resolution

## Important Implementation Notes

### File Extension Handling
The `filename_with_extension()` method in `CodeFile` implements extension priority:
- Names with existing extensions (e.g., "Button.tsx", "Modal.vue") are preserved
- Names without extensions get language-specific extensions (e.g., "utils" → "utils.ts")
- This enables mixed-extension projects common in TypeScript/React/Vue development

## Dependencies

Key dependencies include:
- `clap`: CLI argument parsing
- `anyhow`: Error handling
- `askama`: Template engine
- `inquire`: Interactive prompts
- `regex`: Pattern matching
- `yaml-rust`: YAML parsing