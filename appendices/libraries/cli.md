---
layout: default
title: CLI
parent: Libraries
grand_parent: Appendices
nav_order: 4
---

# CLI Libraries

Libraries for building command-line applications.

## clap

The most popular argument parser for Rust.

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

### Derive API

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "myapp")]
#[command(about = "A sample CLI application")]
struct Cli {
    /// Input file path
    #[arg(short, long)]
    input: String,

    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Number of iterations
    #[arg(short, long, default_value_t = 1)]
    count: u32,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Process the input
    Process {
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Validate the input
    Validate,
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Input: {}", cli.input);
    }

    match cli.command {
        Some(Commands::Process { output }) => {
            println!("Processing...");
        }
        Some(Commands::Validate) => {
            println!("Validating...");
        }
        None => {}
    }
}
```

### Argument Types

| Attribute | Purpose |
|-----------|---------|
| `#[arg(short, long)]` | `-v` and `--verbose` |
| `#[arg(required = true)]` | Must be provided |
| `#[arg(default_value = "x")]` | Default value |
| `#[arg(value_parser)]` | Custom parser |
| `#[arg(env = "VAR")]` | From environment |
| `#[arg(num_args = 1..)]` | Multiple values |

### Builder API

```rust
use clap::{Command, Arg};

let matches = Command::new("myapp")
    .version("1.0")
    .author("Your Name")
    .about("Does awesome things")
    .arg(Arg::new("input")
        .short('i')
        .long("input")
        .required(true))
    .arg(Arg::new("verbose")
        .short('v')
        .long("verbose")
        .action(clap::ArgAction::SetTrue))
    .get_matches();

let input = matches.get_one::<String>("input").unwrap();
let verbose = matches.get_flag("verbose");
```

## indicatif

Progress bars and spinners.

```toml
[dependencies]
indicatif = "0.17"
```

### Progress Bar

```rust
use indicatif::{ProgressBar, ProgressStyle};

let pb = ProgressBar::new(100);
pb.set_style(ProgressStyle::with_template(
    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})"
)?.progress_chars("#>-"));

for i in 0..100 {
    pb.inc(1);
    // Do work
}
pb.finish_with_message("Done!");
```

### Spinner

```rust
use indicatif::ProgressBar;
use std::time::Duration;

let spinner = ProgressBar::new_spinner();
spinner.set_message("Working...");
spinner.enable_steady_tick(Duration::from_millis(100));

// Do work
spinner.finish_with_message("Complete!");
```

### Multi Progress

```rust
use indicatif::{MultiProgress, ProgressBar};

let multi = MultiProgress::new();
let pb1 = multi.add(ProgressBar::new(100));
let pb2 = multi.add(ProgressBar::new(100));

// Progress bars update independently
pb1.inc(10);
pb2.inc(20);
```

## colored

Terminal colors and styles.

```toml
[dependencies]
colored = "2"
```

```rust
use colored::*;

println!("{}", "This is red".red());
println!("{}", "This is blue and bold".blue().bold());
println!("{}", "Green on yellow".green().on_yellow());
println!("{}", "Underlined".underline());

// Conditional coloring
if error {
    println!("{}", message.red());
} else {
    println!("{}", message.green());
}
```

## dialoguer

Interactive prompts.

```toml
[dependencies]
dialoguer = "0.11"
```

```rust
use dialoguer::{Input, Confirm, Select, MultiSelect, Password};

// Text input
let name: String = Input::new()
    .with_prompt("Your name")
    .default("Anonymous".into())
    .interact_text()?;

// Confirmation
let proceed = Confirm::new()
    .with_prompt("Do you want to continue?")
    .default(true)
    .interact()?;

// Selection
let options = vec!["Option A", "Option B", "Option C"];
let selection = Select::new()
    .with_prompt("Choose an option")
    .items(&options)
    .default(0)
    .interact()?;

// Multi-select
let selections = MultiSelect::new()
    .with_prompt("Select items")
    .items(&options)
    .interact()?;

// Password
let password = Password::new()
    .with_prompt("Password")
    .interact()?;
```

## console

Low-level terminal manipulation.

```toml
[dependencies]
console = "0.15"
```

```rust
use console::{Term, style};

let term = Term::stdout();

// Clear screen
term.clear_screen()?;

// Move cursor
term.move_cursor_to(10, 5)?;

// Styled output
println!("{}", style("Error!").red().bold());
println!("{}", style("Success!").green());

// Read input
let input = term.read_line()?;

// Terminal size
let (height, width) = term.size();
```

## tui / ratatui

Terminal user interfaces.

```toml
[dependencies]
ratatui = "0.26"
crossterm = "0.27"
```

```rust
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    loop {
        terminal.draw(|frame| {
            let block = Block::default()
                .title("My App")
                .borders(Borders::ALL);
            let paragraph = Paragraph::new("Hello, TUI!")
                .block(block);
            frame.render_widget(paragraph, frame.size());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
```

## env_logger

Simple logging from environment.

```toml
[dependencies]
env_logger = "0.11"
log = "0.4"
```

```rust
use log::{info, warn, error};

fn main() {
    env_logger::init();

    info!("Starting application");
    warn!("Warning message");
    error!("Error occurred");
}
```

```bash
RUST_LOG=info ./myapp
RUST_LOG=myapp=debug ./myapp
```

## Summary

| Crate | Purpose |
|-------|---------|
| clap | Argument parsing |
| indicatif | Progress bars |
| colored | Terminal colors |
| dialoguer | Interactive prompts |
| console | Terminal control |
| ratatui | TUI framework |
| env_logger | Simple logging |

## Choosing Libraries

| Need | Recommendation |
|------|----------------|
| Argument parsing | clap |
| Progress indication | indicatif |
| Colored output | colored |
| User prompts | dialoguer |
| Full TUI | ratatui |
