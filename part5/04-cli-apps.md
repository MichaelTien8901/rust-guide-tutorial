---
layout: default
title: CLI Apps
parent: Part 5 - Patterns
nav_order: 4
---

# CLI Applications

Build command-line tools with clap.

## Basic CLI with clap

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "myapp")]
#[command(about = "A sample CLI application")]
struct Args {
    /// Input file to process
    #[arg(short, long)]
    input: String,

    /// Output file path
    #[arg(short, long, default_value = "output.txt")]
    output: String,

    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Number of threads to use
    #[arg(short = 'j', long, default_value_t = 4)]
    threads: usize,
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("Input: {}", args.input);
        println!("Output: {}", args.output);
        println!("Threads: {}", args.threads);
    }

    // Process files...
}
```

Add to Cargo.toml:
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

## Subcommands

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "git-like")]
#[command(about = "A git-like CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new repository
    Init {
        /// Path to initialize
        #[arg(default_value = ".")]
        path: String,
    },
    /// Clone a repository
    Clone {
        /// Repository URL
        url: String,
        /// Target directory
        #[arg(short, long)]
        directory: Option<String>,
    },
    /// Show status
    Status {
        /// Show short format
        #[arg(short, long)]
        short: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            println!("Initializing repository at: {}", path);
        }
        Commands::Clone { url, directory } => {
            let dir = directory.unwrap_or_else(|| url.split('/').last().unwrap().to_string());
            println!("Cloning {} into {}", url, dir);
        }
        Commands::Status { short } => {
            if short {
                println!("M file.txt");
            } else {
                println!("Modified: file.txt");
            }
        }
    }
}
```

## Value Validation

```rust
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Port number (1-65535)
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,

    /// Log level
    #[arg(short, long, value_parser = ["debug", "info", "warn", "error"])]
    level: String,

    /// Config file (must exist)
    #[arg(short, long, value_parser = file_exists)]
    config: PathBuf,
}

fn file_exists(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.exists() {
        Ok(path)
    } else {
        Err(format!("File not found: {}", s))
    }
}
```

## Environment Variables

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// API key (can also use MYAPP_API_KEY env var)
    #[arg(long, env = "MYAPP_API_KEY")]
    api_key: String,

    /// Database URL
    #[arg(long, env = "DATABASE_URL", default_value = "sqlite://local.db")]
    database: String,
}
```

## Progress Bars with indicatif

```rust
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

fn main() {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    for _ in 0..100 {
        pb.inc(1);
        std::thread::sleep(Duration::from_millis(50));
    }

    pb.finish_with_message("Done!");
}
```

Add to Cargo.toml:
```toml
[dependencies]
indicatif = "0.17"
```

## Colored Output

```rust
use colored::Colorize;

fn main() {
    println!("{}", "Success!".green().bold());
    println!("{}", "Warning: something happened".yellow());
    println!("{}", "Error: operation failed".red().bold());

    // Conditional coloring
    let status = true;
    let message = if status {
        "PASS".green()
    } else {
        "FAIL".red()
    };
    println!("Test: {}", message);
}
```

Add to Cargo.toml:
```toml
[dependencies]
colored = "2"
```

## Interactive Prompts with dialoguer

```rust
use dialoguer::{Confirm, Input, Select, MultiSelect};

fn main() {
    // Text input
    let name: String = Input::new()
        .with_prompt("Your name")
        .default("Anonymous".into())
        .interact_text()
        .unwrap();

    // Confirmation
    let confirmed = Confirm::new()
        .with_prompt("Continue?")
        .default(true)
        .interact()
        .unwrap();

    // Selection
    let options = vec!["Option A", "Option B", "Option C"];
    let selection = Select::new()
        .with_prompt("Choose an option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    // Multi-select
    let features = vec!["Feature 1", "Feature 2", "Feature 3"];
    let selected = MultiSelect::new()
        .with_prompt("Select features")
        .items(&features)
        .interact()
        .unwrap();

    println!("Name: {}", name);
    println!("Confirmed: {}", confirmed);
    println!("Selected: {}", options[selection]);
    println!("Features: {:?}", selected.iter().map(|&i| features[i]).collect::<Vec<_>>());
}
```

Add to Cargo.toml:
```toml
[dependencies]
dialoguer = "0.11"
```

## Complete Example

```rust
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "processor")]
#[command(about = "Process files efficiently")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Process input files
    Process {
        /// Input files to process
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Output directory
        #[arg(short, long, default_value = "output")]
        output: PathBuf,
    },
    /// Show configuration
    Config,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Process { files, output } => {
            println!("{}", "Starting processing...".cyan().bold());

            let pb = ProgressBar::new(files.len() as u64);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40}] {pos}/{len}")?);

            for file in &files {
                if cli.verbose {
                    println!("Processing: {}", file.display());
                }
                // Process file...
                pb.inc(1);
            }

            pb.finish();
            println!("{}", "Done!".green().bold());
        }
        Commands::Config => {
            println!("Current configuration:");
            println!("  Verbose: {}", cli.verbose);
        }
    }

    Ok(())
}
```

## Best Practices

1. **Use derive macros** for cleaner argument definitions
2. **Add help text** with `///` doc comments
3. **Support environment variables** for sensitive data
4. **Validate inputs** with value parsers
5. **Use colors** but respect `NO_COLOR` environment variable
6. **Show progress** for long operations

## Summary

| Crate | Purpose |
|-------|---------|
| `clap` | Argument parsing |
| `indicatif` | Progress bars |
| `colored` | Terminal colors |
| `dialoguer` | Interactive prompts |
| `anyhow` | Error handling |

## Next Steps

Learn about [Web Services]({% link part5/05-web-services.md %}) to build HTTP APIs.
