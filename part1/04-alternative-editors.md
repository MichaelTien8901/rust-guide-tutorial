---
layout: default
title: Alternative Editors
parent: Part 1 - Getting Started
nav_order: 4
---

# Alternative Editors

While VS Code is popular, many excellent editors support Rust development. Here's how to set up alternatives.

## IntelliJ IDEA / CLion

JetBrains IDEs offer excellent Rust support through the official Rust plugin.

### IntelliJ IDEA (Community or Ultimate)

1. Install [IntelliJ IDEA](https://www.jetbrains.com/idea/download/)
2. Go to Settings → Plugins
3. Search "Rust" and install the official plugin
4. Restart the IDE

### CLion

CLion has built-in Rust support:

1. Install [CLion](https://www.jetbrains.com/clion/)
2. The Rust plugin is included
3. Create a new Rust project or import existing

### Features

- Full IDE experience
- Integrated debugger
- Refactoring tools
- Database tools (Ultimate/CLion)
- Built-in terminal

{: .note }
IntelliJ Community is free. CLion and IntelliJ Ultimate require a license but offer more features.

## Neovim

Modern Vim with excellent LSP support for Rust.

### Installation

```bash
# Ubuntu/Debian
sudo apt install neovim

# macOS
brew install neovim

# Arch
sudo pacman -S neovim
```

### LSP Configuration with nvim-lspconfig

Add to your Neovim config (e.g., `~/.config/nvim/init.lua`):

```lua
-- Install rust-analyzer
-- rustup component add rust-analyzer

-- Using lazy.nvim package manager
require("lazy").setup({
    "neovim/nvim-lspconfig",
    "hrsh7th/nvim-cmp",
    "hrsh7th/cmp-nvim-lsp",
    "simrat39/rust-tools.nvim",
})

-- Configure rust-analyzer
local rt = require("rust-tools")
rt.setup({
    server = {
        on_attach = function(_, bufnr)
            -- Keybindings
            vim.keymap.set("n", "K", rt.hover_actions.hover_actions, { buffer = bufnr })
            vim.keymap.set("n", "<Leader>a", rt.code_action_group.code_action_group, { buffer = bufnr })
        end,
    },
})
```

### Alternative: coc.nvim

```vim
" Install coc.nvim, then:
:CocInstall coc-rust-analyzer
```

### Key Plugins for Neovim + Rust

| Plugin | Purpose |
|--------|---------|
| nvim-lspconfig | LSP client |
| rust-tools.nvim | Enhanced Rust support |
| nvim-cmp | Completion |
| nvim-dap | Debugging |
| trouble.nvim | Diagnostics list |

## Vim

Classic Vim can work with rust-analyzer through various plugins.

### Using vim-lsp

```vim
Plug 'prabirshrestha/vim-lsp'
Plug 'mattn/vim-lsp-settings'

" After :PlugInstall, open a .rs file and run:
" :LspInstallServer rust-analyzer
```

### Using ALE

```vim
Plug 'dense-analysis/ale'

let g:ale_linters = {'rust': ['analyzer']}
let g:ale_fixers = {'rust': ['rustfmt']}
let g:ale_fix_on_save = 1
```

## Emacs

Emacs has strong Rust support through rustic-mode.

### Using rustic-mode

```elisp
;; Using use-package
(use-package rustic
  :ensure t
  :config
  (setq rustic-format-on-save t))

;; LSP mode for rust-analyzer
(use-package lsp-mode
  :ensure t
  :commands lsp
  :hook (rustic-mode . lsp))
```

### Key Packages

| Package | Purpose |
|---------|---------|
| rustic | Rust major mode |
| lsp-mode | LSP client |
| company | Completion |
| flycheck | On-the-fly checking |
| dap-mode | Debugging |

## Zed

A modern, high-performance editor with built-in Rust support.

### Installation

Download from [zed.dev](https://zed.dev/)

### Features

- Written in Rust
- Native rust-analyzer integration
- No configuration needed
- Extremely fast
- Built-in collaboration

Rust support works out of the box—just open a `.rs` file.

## Helix

A post-modern modal text editor with built-in LSP support.

### Installation

```bash
# macOS
brew install helix

# Arch
sudo pacman -S helix

# From source
git clone https://github.com/helix-editor/helix
cd helix
cargo install --path helix-term
```

### Configuration

Helix works with rust-analyzer automatically. Add to `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "rust"
auto-format = true

[language-server.rust-analyzer.config]
check.command = "clippy"
```

### Key Features

- Modal editing (Vim-like)
- Built-in LSP
- Tree-sitter for syntax
- Multiple selections
- No plugins needed for basic Rust

## Sublime Text

### Installation

1. Install [Package Control](https://packagecontrol.io/)
2. Install "LSP" package
3. Install "LSP-rust-analyzer" package
4. Install "Rust Enhanced" for syntax

### Configuration

Preferences → Package Settings → LSP → Settings:

```json
{
    "clients": {
        "rust-analyzer": {
            "enabled": true
        }
    }
}
```

## Editor Comparison

| Editor | Type | Cost | Setup | Features |
|--------|------|------|-------|----------|
| VS Code | GUI | Free | Easy | ★★★★★ |
| IntelliJ | GUI | Free/Paid | Easy | ★★★★★ |
| CLion | GUI | Paid | Easy | ★★★★★ |
| Neovim | Terminal | Free | Medium | ★★★★☆ |
| Emacs | Both | Free | Hard | ★★★★☆ |
| Zed | GUI | Free | Easy | ★★★★☆ |
| Helix | Terminal | Free | Easy | ★★★★☆ |
| Sublime | GUI | Paid | Easy | ★★★☆☆ |

## Choosing an Editor

- **New to programming**: VS Code or Zed
- **JetBrains user**: IntelliJ/CLion
- **Vim user**: Neovim or Helix
- **Emacs user**: Emacs with rustic
- **Want minimal setup**: Zed or Helix
- **Terminal-only**: Helix or Neovim

## Next Steps

Now that your editor is set up, continue to [Docker Setup]({% link part1/05-docker-setup.md %}) or skip to [Hello World]({% link part1/06-hello-world.md %}).
