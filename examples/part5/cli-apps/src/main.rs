//! CLI Applications Example
//!
//! Demonstrates CLI patterns with clap derive macros.
//!
//! # CLI Structure
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │                  CLI Application Structure              │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!          ┌─────────────────┼─────────────────┐
//!          ▼                 ▼                 ▼
//!     ┌─────────┐      ┌─────────┐      ┌─────────┐
//!     │ Command │      │ Options │      │  Args   │
//!     └─────────┘      └─────────┘      └─────────┘
//!          │                │                │
//!          ▼                ▼                ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │  myapp command --flag --option=value arg1 arg2          │
//!     └─────────────────────────────────────────────────────────┘
//! ```

use clap::{Args, Parser, Subcommand, ValueEnum};

/// Example CLI application demonstrating clap patterns
#[derive(Parser, Debug)]
#[command(name = "myapp")]
#[command(author = "Example Author")]
#[command(version = "1.0")]
#[command(about = "Demonstrates CLI patterns", long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new project
    Init {
        /// Project name
        name: String,

        /// Project template
        #[arg(short, long, default_value = "basic")]
        template: String,
    },

    /// Build the project
    Build(BuildArgs),

    /// Run the project
    Run {
        /// Arguments to pass to the program
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Args, Debug)]
struct BuildArgs {
    /// Build in release mode
    #[arg(short, long)]
    release: bool,

    /// Target triple
    #[arg(short, long)]
    target: Option<String>,

    /// Number of parallel jobs
    #[arg(short, long, default_value = "4")]
    jobs: usize,

    /// Features to enable
    #[arg(short = 'F', long, value_delimiter = ',')]
    features: Vec<String>,

    /// Output format
    #[arg(long, value_enum, default_value = "binary")]
    output: OutputFormat,
}

#[derive(Subcommand, Debug)]
enum ConfigAction {
    /// Show current configuration
    Show,

    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,

        /// Configuration value
        value: String,
    },

    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },

    /// List all configuration keys
    List {
        /// Output format
        #[arg(long, value_enum, default_value = "table")]
        format: ListFormat,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Binary,
    Library,
    Both,
}

#[derive(ValueEnum, Clone, Debug)]
enum ListFormat {
    Table,
    Json,
    Yaml,
}

fn main() {
    // For demonstration, we'll show different argument combinations
    println!("=== CLI Application Patterns ===\n");

    // Simulate different command-line inputs
    demonstrate_parsing();

    println!("\n--- Actual CLI Parsing ---");
    // Parse actual command line (will use defaults or show help)
    let cli = Cli::try_parse();

    match cli {
        Ok(cli) => {
            println!("  Parsed CLI: {:?}", cli);
            handle_command(&cli);
        }
        Err(e) => {
            println!("  (No args provided, showing simulated examples above)");
            println!("  Actual error: {}", e.kind());
        }
    }
}

fn demonstrate_parsing() {
    // Simulate parsing different commands
    let test_cases = vec![
        vec![
            "myapp",
            "--verbose",
            "init",
            "my-project",
            "--template",
            "web",
        ],
        vec![
            "myapp",
            "build",
            "--release",
            "--target",
            "x86_64-unknown-linux-gnu",
        ],
        vec!["myapp", "build", "-F", "serde,tokio", "--jobs", "8"],
        vec!["myapp", "run", "--", "arg1", "arg2", "--flag"],
        vec!["myapp", "config", "set", "debug", "true"],
        vec!["myapp", "config", "list", "--format", "json"],
    ];

    for args in test_cases {
        println!("  Command: {}", args.join(" "));
        match Cli::try_parse_from(&args) {
            Ok(cli) => {
                println!("    Verbose: {}", cli.verbose);
                println!("    Config: {}", cli.config);
                print_command(&cli.command);
            }
            Err(e) => println!("    Error: {}", e),
        }
        println!();
    }
}

fn print_command(cmd: &Commands) {
    match cmd {
        Commands::Init { name, template } => {
            println!("    Init: name={}, template={}", name, template);
        }
        Commands::Build(args) => {
            println!("    Build:");
            println!("      release: {}", args.release);
            println!("      target: {:?}", args.target);
            println!("      jobs: {}", args.jobs);
            println!("      features: {:?}", args.features);
            println!("      output: {:?}", args.output);
        }
        Commands::Run { args } => {
            println!("    Run args: {:?}", args);
        }
        Commands::Config { action } => {
            println!("    Config action: {:?}", action);
        }
    }
}

fn handle_command(cli: &Cli) {
    if cli.verbose {
        println!("  Verbose mode enabled");
        println!("  Using config: {}", cli.config);
    }

    match &cli.command {
        Commands::Init { name, template } => {
            println!(
                "  Initializing project '{}' with template '{}'",
                name, template
            );
            // In real app: create directories, files, etc.
        }
        Commands::Build(args) => {
            let mode = if args.release { "release" } else { "debug" };
            println!("  Building in {} mode with {} jobs", mode, args.jobs);
            if let Some(target) = &args.target {
                println!("  Target: {}", target);
            }
            if !args.features.is_empty() {
                println!("  Features: {}", args.features.join(", "));
            }
        }
        Commands::Run { args } => {
            println!("  Running with args: {:?}", args);
        }
        Commands::Config { action } => match action {
            ConfigAction::Show => println!("  Showing configuration..."),
            ConfigAction::Set { key, value } => {
                println!("  Setting {}={}", key, value);
            }
            ConfigAction::Get { key } => {
                println!("  Getting value for key: {}", key);
            }
            ConfigAction::List { format } => {
                println!("  Listing config in {:?} format", format);
            }
        },
    }
}

// ============================================
// Additional CLI Patterns
// ============================================

/// Example of positional arguments
#[derive(Parser, Debug)]
#[command(name = "greet")]
struct GreetCli {
    /// Name to greet
    name: String,

    /// Number of times to greet
    #[arg(default_value = "1")]
    count: u32,
}

/// Example of environment variable fallback
#[derive(Parser, Debug)]
#[command(name = "server")]
struct ServerCli {
    /// Server host
    #[arg(long, env = "SERVER_HOST", default_value = "localhost")]
    host: String,

    /// Server port
    #[arg(long, env = "SERVER_PORT", default_value = "8080")]
    port: u16,

    /// Database URL
    #[arg(long, env = "DATABASE_URL")]
    database_url: Option<String>,
}

/// Example of mutually exclusive options
#[derive(Parser, Debug)]
#[command(name = "format")]
struct FormatCli {
    /// Input file
    input: String,

    #[command(flatten)]
    output: OutputOptions,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct OutputOptions {
    /// Output to file
    #[arg(short, long)]
    output: Option<String>,

    /// Output to stdout
    #[arg(long)]
    stdout: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_command() {
        let cli = Cli::try_parse_from(["myapp", "init", "test-project"]).unwrap();
        match cli.command {
            Commands::Init { name, template } => {
                assert_eq!(name, "test-project");
                assert_eq!(template, "basic");
            }
            _ => panic!("Expected Init command"),
        }
    }

    #[test]
    fn test_build_command() {
        let cli = Cli::try_parse_from(["myapp", "build", "--release", "--jobs", "8"]).unwrap();

        match cli.command {
            Commands::Build(args) => {
                assert!(args.release);
                assert_eq!(args.jobs, 8);
            }
            _ => panic!("Expected Build command"),
        }
    }

    #[test]
    fn test_global_verbose() {
        let cli = Cli::try_parse_from(["myapp", "--verbose", "init", "test"]).unwrap();

        assert!(cli.verbose);
    }

    #[test]
    fn test_features_list() {
        let cli = Cli::try_parse_from(["myapp", "build", "-F", "serde,tokio,async"]).unwrap();

        match cli.command {
            Commands::Build(args) => {
                assert_eq!(args.features, vec!["serde", "tokio", "async"]);
            }
            _ => panic!("Expected Build command"),
        }
    }

    #[test]
    fn test_config_subcommand() {
        let cli = Cli::try_parse_from(["myapp", "config", "set", "key", "value"]).unwrap();

        match cli.command {
            Commands::Config {
                action: ConfigAction::Set { key, value },
            } => {
                assert_eq!(key, "key");
                assert_eq!(value, "value");
            }
            _ => panic!("Expected Config Set command"),
        }
    }
}
