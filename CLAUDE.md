# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`saba` is a declarative development framework written in Rust that generates code based on YAML specifications. The current v2 implementation supports multiple programming languages including Rust, Go, Python, TypeScript, JavaScript, and generates project structures based on simple YAML configurations.

## Architecture (v2)

The v2 implementation follows a clean architecture pattern:

- **CLI Layer** (`src/cli/`): Command-line interface
  - `command/new.rs`: Project initialization with dual modes (interactive/AI)
  - `command/up.rs`: Code generation execution using CodeGenerator
  - `command/analyze.rs`: Project analysis features
- **Code Generation Layer** (`src/code_generation/`): Core generation logic
  - `core/generator.rs`: Unified CodeGenerator for all languages
  - `language/`: Language-specific generators (rust, go, python, typescript, javascript)
    - Each language has dedicated generators and handlers
    - Supports both single-project and multi-project configurations
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
# Initialize new project with interactive language selection (Human mode)
saba new

# Initialize new project with specified language (AI mode)
saba new --lang rust
saba new --lang typescript
saba new --lang go
saba new --lang python
saba new --lang javascript

# Generate code from saba.yml specification
saba up
```

### Testing
```bash
# Test all language generators (requires build.sh first)
./build.sh
cd _test

# Test Go project
echo "- name: test-go-project
  root: true
  lang: go
  upstream:
    - name: pkg
      upstream:
        - name: models
          codefile:
            - name: user
  codefile:
    - name: main" > saba.yml
./saba up

# Test Python project  
echo "- name: test-python-project
  root: true
  lang: python
  upstream:
    - name: src
      upstream:
        - name: models
          codefile:
            - name: user
  codefile:
    - name: main" > saba.yml
./saba up

# Test JavaScript project
echo "- name: test-js-project
  root: true
  lang: javascript
  upstream:
    - name: src
      upstream:
        - name: components
          codefile:
            - name: Button
            - name: Header.jsx
  codefile:
    - name: index" > saba.yml
./saba up
```

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

1. `saba new` creates a `saba.yml` specification file:
   - **Human Mode**: Interactive language selection prompts when no `--lang` option provided
   - **AI Mode**: Direct language specification via `--lang` option
   - **Project Naming**: Auto-generates sequential project names (`app_1`, `app_2`, etc.)
   - **Multi-Project Support**: Automatically appends to existing `saba.yml` and removes `root: true` from previous projects
2. `saba up` uses the unified `CodeGenerator` to process the specification:
   - **Single Project Mode**: `root: true` generates directly in the current directory
   - **Multi-Project Mode**: Creates separate directories for each project
   - **Workspace Generation**: Automatically creates workspace files for multi-project Rust configurations
3. Each language generator creates:
   - Directory structure based on module hierarchy
   - Code files with appropriate extensions and content
   - Language-specific project files:
     - **Rust**: `Cargo.toml` (for individual projects), workspace `Cargo.toml` + `Makefile.toml` (for multi-project)
     - **TypeScript**: `package.json`, `tsconfig.json`
     - **Go**: `go.mod`, `go.sum`, `main.go`
     - **Python**: `setup.py`, `requirements.txt`, `main.py`
     - **JavaScript**: `package.json`, `index.js`

## Language-Specific Features

### Rust
- Generates `mod.rs`, `main.rs`, or `lib.rs` with proper module declarations
- Supports `pub` visibility controls for modules and files
- Auto-detects library vs binary projects based on `lib.rs` presence
- **Workspace Support**: Generates workspace `Cargo.toml` and `Makefile.toml` for multi-project configurations

### TypeScript
- Generates `index.ts` files with barrel exports for each module
- Creates `package.json` and `tsconfig.json` configuration files
- **File Extension Priority**: If `codefile.name` contains a dot (.), the existing extension is preserved (.tsx, .vue, .jsx, etc.). Otherwise, adds .ts extension.
- Barrel exports remove extensions for proper TypeScript module resolution

### Go
- Generates `go.mod` and `go.sum` for dependency management
- Creates `main.go` with "Hello, world!" template when no main file is explicitly defined
- **Package Naming**: Root-level files use `package main`, module files use directory name as package
- Proper package sanitization (hyphens to underscores, lowercase)

### Python
- Generates `setup.py` with project metadata and `requirements.txt` for dependencies
- Creates `__init__.py` files automatically for each module directory
- **Barrel Imports**: `__init__.py` files contain `from .module import *` statements
- Creates `main.py` with proper entry point template when no main file is explicitly defined

### JavaScript
- Generates `package.json` with ES module configuration and Node.js scripts
- Creates `index.js` files with ES module barrel exports (`export * from`)
- **Mixed Extensions**: Preserves original extensions (.jsx, .mjs) in import statements
- Supports modern ES module syntax with proper file extension handling

## Important Implementation Notes

### File Extension Handling
The `filename_with_extension()` method in `CodeFile` implements extension priority:
- Names with existing extensions (e.g., "Button.tsx", "Modal.vue", "Header.jsx") are preserved
- Names without extensions get language-specific extensions (e.g., "utils" → "utils.ts", "utils.py", "utils.go", "utils.js")
- This enables mixed-extension projects common in TypeScript/React/Vue development

### Build and Test Workflow
**CRITICAL**: Always run `build.sh` before testing:
1. `build.sh` clears `_test` directory contents (`rm -rf ./_test/*`)
2. Performs release build (`cargo build --release`)
3. Copies executable to `_test` directory
4. This ensures testing uses the latest implementation

### Multi-Project vs Single-Project
- **Single Project** (`root: true`): Generates directly in current directory
- **Multi-Project**: Creates separate subdirectories for each project
- **Workspace Detection**: Automatically generates Rust workspace files when multiple Rust projects exist

### saba new Command Features
- **Dual Mode Operation**:
  - `saba new` (no args): Interactive language selection for human users
  - `saba new --lang <language>`: Direct language specification for AI automation
- **Sequential Project Names**: Auto-generates `app_1`, `app_2`, `app_3`, etc.
- **Smart Multi-Project Handling**:
  - First project: Creates new `saba.yml` with `root: true`
  - Subsequent projects: Appends to existing `saba.yml` and removes `root: true` from all projects
- **Language Directory Structure**:
  - **Rust projects**: Generate with `src/` directory structure
  - **All other languages**: Generate with root-level file structure

## File Protection System

**CRITICAL**: The saba framework implements a three-tier file protection system to prevent code loss:

### 1. Code Files (Complete Protection)
- **Files**: `.rs`, `.go`, `.py`, `.js`, `.ts`, `.jsx`, `.tsx`, `.vue`, etc.
- **Behavior**: Once created, these files are **never overwritten**
- **Implementation**: Uses `if !file_path.exists()` checks before writing
- **Purpose**: Protects developer's implementation code

### 2. Management Files (Partial Updates)
- **Files**: `mod.rs`, `lib.rs`, `main.rs`, `__init__.py`, `index.js`, `index.ts`
- **Behavior**: Updates only the saba-managed sections between header/footer markers
- **Implementation**: Uses `ContentUpdater` with regex pattern matching
- **Headers/Footers**:
  - Rust: `// start auto exported by saba.` ... `// end auto exported by saba.`
  - Python: `# start auto exported by saba.` ... `# end auto exported by saba.`
  - JS/TS: `// start auto exported by saba.` ... `// end auto exported by saba.`
- **Purpose**: Allows saba to manage imports/exports while preserving custom code

### 3. Project Configuration Files (Initial Creation Only)
- **Files**: `package.json`, `tsconfig.json`, `Cargo.toml`, `go.mod`, `go.sum`, `requirements.txt`, `setup.py`
- **Behavior**: Created only if they don't exist
- **Implementation**: Uses `if !file_path.exists()` checks
- **Purpose**: Prevents overwriting user's project configuration changes

### Module Management Files Always Generate
- **mod.rs** (Rust): Generated for all modules except `src`
- **index.js/index.ts** (JS/TS): Generated for all modules
- **__init__.py** (Python): Generated for all modules
- **Purpose**: Ensures proper module structure even for empty directories

## Git Commit Guidelines

**IMPORTANT**: All commit messages must be written in Japanese.

### Commit Message Format
```
<動作の概要>

<詳細な説明>
- 実装した機能や修正内容
- 変更の理由や背景
- 影響範囲

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

### Examples
```bash
# 新機能追加
git commit -m "ファイル保護システムの実装

- 3層ファイル保護システムを追加
- ContentUpdaterユーティリティで部分更新を実装
- 既存コードの上書きを防ぐ仕組みを構築

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"

# バグ修正
git commit -m "mod.rsの空モジュール対応を修正

- 空のモジュールでもmod.rsが生成されるように修正
- v1仕様との互換性を確保
- 全言語で管理ファイルが常に生成されるよう統一

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

### Commit Types (Japanese)
- `実装`: 新機能の実装
- `修正`: バグ修正
- `改善`: 既存機能の改良
- `リファクタリング`: コード構造の改善
- `テスト`: テストの追加・修正
- `ドキュメント`: ドキュメントの更新
- `設定`: 設定ファイルの変更

## Dependencies

Key dependencies include:
- `clap`: CLI argument parsing
- `anyhow`: Error handling
- `askama`: Template engine
- `inquire`: Interactive prompts
- `regex`: Pattern matching (for ContentUpdater)
- `yaml-rust`: YAML parsing