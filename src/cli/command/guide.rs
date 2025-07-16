use clap::Command;
use anyhow::Result;

pub fn spec() -> Command {
    Command::new("guide")
        .about("Comprehensive guide for Claude Code on using saba")
        .long_about(
            "Provides detailed guidance for Claude Code (claude.ai/code) on how to use saba effectively. \
            This command outputs comprehensive information about saba's capabilities, workflows, \
            and best practices specifically formatted for AI assistance."
        )
}

pub fn action() -> Result<()> {
    print_saba_guide();
    Ok(())
}

fn print_saba_guide() {
    println!(r#"
# Saba v2.0 Guide for Claude Code

## Overview
Saba is a declarative development framework that generates multi-language project structures from YAML specifications.

## Supported Languages
- **Rust**: With Cargo.toml, workspace support, and proper module declarations
- **Go**: With go.mod, go.sum, and package management
- **Python**: With setup.py, requirements.txt, and __init__.py barrel imports
- **TypeScript**: With package.json, tsconfig.json, and barrel exports
- **JavaScript**: With package.json and ES module support

## Core Commands

### saba new
Initialize projects with two modes:
```bash
# Human Mode (Interactive)
saba new

# AI Mode (Direct specification)
saba new --lang rust
saba new --lang typescript
saba new --lang go
saba new --lang python
saba new --lang javascript
```

**Features:**
- Auto-generates sequential project names (app_1, app_2, app_3...)
- Smart multi-project handling (removes root: true from existing projects)
- Language-specific directory structures

### saba up
Generate project structures from saba.yml:
```bash
saba up
```

**Modes:**
- **Single Project**: Projects with `root: true` generate in current directory
- **Multi-Project**: Creates separate directories for each project

## Project Structure Patterns

### Rust Projects
- Generate with `src/` directory structure
- Individual `Cargo.toml` files for each project
- Workspace `Cargo.toml` + `Makefile.toml` for multi-project setups

### Other Languages (Go/Python/TypeScript/JavaScript)
- Generate with root-level file structure
- Language-specific configuration files (go.mod, package.json, setup.py)

## Workflow for Claude Code

### 1. Project Initialization
Always start with `saba new`:
- For human users: `saba new` (interactive)
- For AI automation: `saba new --lang <language>`

### 2. Code Generation
After creating saba.yml, run:
```bash
saba up
```

### 3. Testing Workflow
**CRITICAL**: Always use build.sh for testing:
```bash
./build.sh    # Builds and copies to _test directory
cd _test      # Change to test directory
# Run saba commands here
```

## saba.yml Configuration Guide

### Configuration Structure
saba.yml is a YAML array where each item represents a project:

```yaml
- name: <project-name>      # Required: Project name
  root: <boolean>           # Optional: true for single project mode
  lang: <language>          # Required: Programming language
  upstream: [...]           # Optional: Module hierarchy
  codefile: [...]           # Optional: Files in current level
```

### Required Fields
- **name**: Project identifier (string)
- **lang**: Programming language (rust|go|python|typescript|javascript)

### Optional Fields
- **root**: Boolean flag for single-project mode (only one project should have this)
- **upstream**: Array of sub-modules/directories
- **codefile**: Array of files at the current level

### File Specification
```yaml
codefile:
  - name: simple-file      # Gets language extension (.rs, .go, .py, .ts, .js)
  - name: Component.tsx    # Extension preserved
  - name: styles.css       # Extension preserved
  - name: config.json      # Extension preserved
```

### Module Hierarchy
```yaml
upstream:
  - name: src              # Directory name
    upstream:              # Nested modules
      - name: components
        codefile:
          - name: Button
          - name: Modal.vue
      - name: utils
        codefile:
          - name: helpers
          - name: constants
    codefile:              # Files directly in src/
      - name: main
      - name: lib
```

### Language-Specific Patterns

#### Rust Projects
```yaml
- name: rust-app
  root: true
  lang: rust
  upstream:
    - name: src            # Always use src/ for Rust
      upstream:
        - name: models
          codefile:
            - name: user   # Becomes user.rs
            - name: post   # Becomes post.rs
        - name: handlers
          codefile:
            - name: auth   # Becomes auth.rs
      codefile:
        - name: main       # Becomes src/main.rs
        - name: lib        # Becomes src/lib.rs (if library)
  codefile:
    - name: build          # Becomes build.rs (build script)
```

#### TypeScript/React Projects
```yaml
- name: react-app
  root: true
  lang: typescript
  upstream:
    - name: src
      upstream:
        - name: components
          codefile:
            - name: Button.tsx     # Preserved extension
            - name: Modal.vue      # Vue component
            - name: Header.jsx     # JSX extension preserved
        - name: hooks
          codefile:
            - name: useAuth        # Becomes useAuth.ts
            - name: useApi         # Becomes useApi.ts
        - name: types
          codefile:
            - name: api.d.ts       # Declaration file preserved
      codefile:
        - name: index              # Becomes src/index.ts
        - name: App                # Becomes src/App.ts
  codefile:
    - name: index                  # Becomes index.ts (root level)
```

#### Go Projects
```yaml
- name: go-service
  root: true
  lang: go
  upstream:
    - name: pkg
      upstream:
        - name: models
          codefile:
            - name: user           # Becomes user.go
        - name: handlers
          codefile:
            - name: auth           # Becomes auth.go
    - name: cmd
      upstream:
        - name: server
          codefile:
            - name: main           # Becomes main.go
  codefile:
    - name: main                   # Becomes main.go (root level)
```

#### Python Projects
```yaml
- name: python-app
  root: true
  lang: python
  upstream:
    - name: src
      upstream:
        - name: models
          codefile:
            - name: user           # Becomes user.py
            - name: __init__       # Becomes __init__.py
        - name: services
          codefile:
            - name: auth_service   # Becomes auth_service.py
      codefile:
        - name: main               # Becomes src/main.py
  codefile:
    - name: main                   # Becomes main.py (root level)
    - name: setup                  # Becomes setup.py
```

#### JavaScript/Node.js Projects
```yaml
- name: node-api
  root: true
  lang: javascript
  upstream:
    - name: src
      upstream:
        - name: routes
          codefile:
            - name: auth.mjs       # ES module preserved
            - name: users          # Becomes users.js
        - name: middleware
          codefile:
            - name: cors           # Becomes cors.js
      codefile:
        - name: index              # Becomes src/index.js
        - name: server             # Becomes src/server.js
  codefile:
    - name: index                  # Becomes index.js (root level)
```

### Multi-Project Configuration
```yaml
# Backend service (Rust)
- name: api-server
  lang: rust
  upstream:
    - name: src
      upstream:
        - name: handlers
          codefile:
            - name: auth
            - name: users
      codefile:
        - name: main

# Frontend application (TypeScript)  
- name: web-client
  lang: typescript
  upstream:
    - name: src
      upstream:
        - name: components
          codefile:
            - name: App.tsx
            - name: Router.tsx
      codefile:
        - name: index

# Shared library (Go)
- name: shared-utils
  lang: go
  upstream:
    - name: pkg
      upstream:
        - name: validators
          codefile:
            - name: email
            - name: password
  codefile:
    - name: main
```

### Advanced Patterns

#### Mixed Technology Stack
```yaml
# Microservice architecture
- name: auth-service
  lang: go
  upstream:
    - name: internal
      upstream:
        - name: handlers
        - name: models
        - name: database

- name: user-service  
  lang: rust
  upstream:
    - name: src
      upstream:
        - name: handlers
        - name: models

- name: admin-dashboard
  lang: typescript
  upstream:
    - name: src
      upstream:
        - name: pages
        - name: components
        - name: hooks

- name: data-processor
  lang: python
  upstream:
    - name: src
      upstream:
        - name: processors
        - name: models
        - name: utils
```

#### Monorepo with Shared Libraries
```yaml
# Shared types/interfaces
- name: shared-types
  lang: typescript
  upstream:
    - name: src
      upstream:
        - name: api
          codefile:
            - name: user.d.ts
            - name: auth.d.ts
        - name: common
          codefile:
            - name: enums.ts

# Backend using shared types
- name: backend
  lang: typescript  
  upstream:
    - name: src
      upstream:
        - name: controllers
        - name: services
        - name: models

# Frontend using shared types
- name: frontend
  lang: typescript
  upstream:
    - name: src
      upstream:
        - name: components
        - name: pages
        - name: stores
```

### Best Practices for saba.yml

1. **Project Naming**: Use kebab-case (auth-service, user-management)
2. **Module Organization**: Group related functionality together
3. **File Extensions**: 
   - Specify extensions for special files (.tsx, .vue, .d.ts)
   - Let saba handle standard extensions (.rs, .go, .py, .ts, .js)
4. **Root Project**: Only use `root: true` for single-project setups
5. **Language-Specific Structure**:
   - Rust: Always use `src/` directory
   - Others: Use `src/` for larger projects, root-level for simple ones
6. **Consistent Naming**: Use consistent naming patterns within a project

## File Extension Handling
- **With extensions**: Preserved (Button.tsx, Modal.vue, Header.jsx)
- **Without extensions**: Language-specific extensions added
  - Rust: .rs
  - Go: .go
  - Python: .py
  - TypeScript: .ts
  - JavaScript: .js

## Key Features

### Multi-Project Support
- Automatic workspace detection for Rust projects
- Intelligent root: true management
- Sequential project naming

### Language-Specific Generation
- **Rust**: mod.rs, main.rs/lib.rs with proper declarations
- **Go**: Package naming (main for root, directory-based for modules)
- **Python**: __init__.py with barrel imports
- **TypeScript/JavaScript**: index files with barrel exports

### Intelligent Defaults
- Main files: main (Rust/Go/Python), index (TypeScript/JavaScript)
- Project structure: src/ for Rust, root-level for others
- Configuration files automatically generated per language

## Best Practices for Claude Code

1. **Always use build.sh** before testing changes
2. **Use AI mode** (`--lang` option) for automated project creation
3. **Check existing saba.yml** before running commands
4. **Understand single vs multi-project** modes
5. **Verify language support** before using

## Error Handling
- Missing saba.yml: Run `saba new` first
- Invalid language: Check supported languages list
- Build issues: Use `./build.sh` workflow

## Examples

### Create and Generate Rust Project
```bash
saba new --lang rust    # Creates app_1 with Rust structure
saba up                 # Generates Cargo.toml, src/, main.rs
```

### Add TypeScript Project to Existing Setup
```bash
saba new --lang typescript  # Adds app_2, removes root: true from app_1
saba up                      # Generates workspace with both projects
```

### Multi-Language Workspace
```bash
saba new --lang rust        # app_1 (Rust)
saba new --lang go          # app_2 (Go)  
saba new --lang typescript  # app_3 (TypeScript)
saba up                     # Generates all projects with workspace
```

This guide covers all essential aspects of using saba effectively for multi-language project generation.
"#);
}