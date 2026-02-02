---
layout: default
title: IDE Setup
parent: Part 1 - Getting Started
nav_order: 3
---

# IDE Setup with VS Code

Visual Studio Code with rust-analyzer provides an excellent Rust development experience with features like code completion, inline errors, and debugging.

## Installing VS Code

Download from [code.visualstudio.com](https://code.visualstudio.com/) for your platform.

## Essential Extensions

### rust-analyzer (Required)

The official Rust language server providing:
- Code completion
- Go to definition
- Find references
- Inline type hints
- Error diagnostics
- Code actions and quick fixes

**Install:**
1. Open Extensions (Ctrl+Shift+X)
2. Search "rust-analyzer"
3. Click Install

Or from command line:
```bash
code --install-extension rust-lang.rust-analyzer
```

### CodeLLDB (Recommended)

Native debugger for Rust:
- Breakpoints
- Step debugging
- Variable inspection
- Watch expressions

```bash
code --install-extension vadimcn.vscode-lldb
```

### Even Better TOML

Syntax highlighting and validation for Cargo.toml:

```bash
code --install-extension tamasfe.even-better-toml
```

### Error Lens

Show errors and warnings inline:

```bash
code --install-extension usernamehw.errorlens
```

### crates

Helps manage Cargo.toml dependencies:
- Version hints
- Update notifications
- Quick documentation links

```bash
code --install-extension serayuzgur.crates
```

## Recommended Extensions Summary

| Extension | Purpose | Install Command |
|-----------|---------|-----------------|
| rust-analyzer | Language server | `code --install-extension rust-lang.rust-analyzer` |
| CodeLLDB | Debugging | `code --install-extension vadimcn.vscode-lldb` |
| Even Better TOML | TOML support | `code --install-extension tamasfe.even-better-toml` |
| Error Lens | Inline errors | `code --install-extension usernamehw.errorlens` |
| crates | Dependency management | `code --install-extension serayuzgur.crates` |
| Dependi | Dependency insights | `code --install-extension fill-labs.dependi` |
| GitLens | Git integration | `code --install-extension eamodio.gitlens` |

## VS Code Settings

Add these to your `settings.json` (Ctrl+Shift+P → "Preferences: Open Settings (JSON)"):

```json
{
    // Rust-analyzer settings
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.inlayHints.parameterHints.enable": true,
    "rust-analyzer.inlayHints.typeHints.enable": true,
    "rust-analyzer.lens.enable": true,
    "rust-analyzer.lens.references.enable": true,

    // Format on save
    "[rust]": {
        "editor.formatOnSave": true,
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    },

    // File associations
    "files.associations": {
        "*.rs": "rust"
    },

    // Exclude target folder from search
    "files.watcherExclude": {
        "**/target/**": true
    },
    "search.exclude": {
        "**/target": true
    }
}
```

## Debugging Configuration

Create `.vscode/launch.json` in your project:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable",
            "cargo": {
                "args": [
                    "build",
                    "--bin=${workspaceFolderBasename}",
                    "--package=${workspaceFolderBasename}"
                ],
                "filter": {
                    "name": "${workspaceFolderBasename}",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=${workspaceFolderBasename}",
                    "--package=${workspaceFolderBasename}"
                ],
                "filter": {
                    "name": "${workspaceFolderBasename}",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

## Tasks Configuration

Create `.vscode/tasks.json` for common commands:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "type": "cargo",
            "command": "build",
            "problemMatcher": ["$rustc"],
            "group": "build",
            "label": "cargo build"
        },
        {
            "type": "cargo",
            "command": "run",
            "problemMatcher": ["$rustc"],
            "label": "cargo run"
        },
        {
            "type": "cargo",
            "command": "test",
            "problemMatcher": ["$rustc"],
            "group": "test",
            "label": "cargo test"
        },
        {
            "type": "cargo",
            "command": "clippy",
            "problemMatcher": ["$rustc"],
            "label": "cargo clippy"
        }
    ]
}
```

## Keyboard Shortcuts

Useful shortcuts for Rust development:

| Action | Shortcut |
|--------|----------|
| Go to Definition | F12 |
| Peek Definition | Alt+F12 |
| Find References | Shift+F12 |
| Rename Symbol | F2 |
| Quick Fix | Ctrl+. |
| Format Document | Shift+Alt+F |
| Toggle Comment | Ctrl+/ |
| Run Task | Ctrl+Shift+B |
| Debug | F5 |

## Verifying Setup

1. Create a test project:
   ```bash
   cargo new hello-vscode
   cd hello-vscode
   code .
   ```

2. Open `src/main.rs`

3. Verify rust-analyzer is working:
   - You should see "rust-analyzer" in the status bar
   - Hover over `println!` should show documentation
   - Type errors should show inline

4. Test debugging:
   - Set a breakpoint on the `println!` line
   - Press F5 to start debugging
   - The debugger should stop at your breakpoint

## Troubleshooting

### rust-analyzer Not Starting

1. Check the Output panel (View → Output → rust-analyzer)
2. Ensure `rustup` is in your PATH
3. Try: `rust-analyzer --version`

### Slow Performance

Add to settings.json:
```json
{
    "rust-analyzer.cargo.buildScripts.enable": false,
    "rust-analyzer.procMacro.enable": false
}
```

### Missing Code Actions

Ensure clippy is installed:
```bash
rustup component add clippy
```

## Next Steps

Want to use a different editor? Check out [Alternative Editors]({% link part1/04-alternative-editors.md %}).

Ready to code? Skip to [Hello World]({% link part1/06-hello-world.md %}).
