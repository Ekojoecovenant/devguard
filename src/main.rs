use clap::{Parser, Subcommand};
use colored::Colorize;

mod error;
mod init;
mod missing;
mod parser;
mod validator;

// this struct represents the entire CLI program
#[derive(Parser)]
#[command(name = "devguard")]
#[command(about = "A fast .env scanner for Node.js projects")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// this enum lists ALL possible subcommands
#[derive(Subcommand)]
enum Commands {
    // "check" subcommand with optional --path flag
    Check {
        #[arg(long, help = "Path to .env file")]
        path: Option<String>,
    },
    Init,
}

fn main() {
    // clap reads args automatcally here
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { path } => {
            let path = path.unwrap_or(".env".to_string());

            execute(path);
        }
        Commands::Init => init::init_env(),
    }
}

fn execute(path: String) {
    println!("\n🔍 DevGuard - scanning .env...");
    match parser::parser_env(&path) {
        Ok((lines_map, warnings)) => {
            // Warnings Section
            if !warnings.is_empty() {
                println!("{}", "\n=== Warning(s) ===".yellow().bold());
                for warning in &warnings {
                    println!("{}", warning.yellow());
                }
            }

            // Validation Errors Section
            let valid = validator::validate_env(&lines_map);
            if !valid.is_empty() {
                println!("{}", "\n=== Error(s) ===".red().bold());
                for valid_error in &valid {
                    println!("❌ {} -> {}", valid_error.key.red(), valid_error.message)
                }
            }

            // Missing Keys
            let missing = missing::check_missing_keys(&lines_map, ".env.example");
            if !missing.is_empty() {
                println!("{}", "\n=== Missing(s) ===".red().bold());
                for error in &missing {
                    println!("❌ {} -> {}", error.key.red(), error.message);
                }
            }

            // Summary
            if valid.is_empty() && warnings.is_empty() && missing.is_empty() {
                println!("✅ All checks passed! Your .env looks good!");
            } else {
                let total_errors = valid.len() + missing.len();
                let total_warnings = warnings.len();

                if total_errors > 0 && total_warnings > 0 {
                    println!(
                        "\n⚠️  {} error(s) and {} warning(s) found",
                        total_errors, total_warnings
                    );
                } else if total_errors > 0 {
                    println!("\n⚠️  {} error(s) found", total_errors);
                } else {
                    println!("\n⚠️  {} warning(s) found", total_warnings);
                }
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    };
}
