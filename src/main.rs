use clap::{Parser, Subcommand};
use colored::Colorize;

use crate::scanner::scan_files;

mod config;
mod error;
mod init;
mod missing;
mod parser;
mod scanner;
mod validator;

// this struct represents the entire CLI program
#[derive(Parser)]
#[command(name = "guardstack")]
#[command(about = "A fast project security scanner for Node.js projects")]
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
        #[arg(long, help = "Path to config file")]
        config: Option<String>,
    },
    Init,
    Scan {
        #[arg(long, help = "Path to scan")]
        path: Option<String>,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "❌ Error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { path, config } => {
            let path = path.unwrap_or_else(|| ".env".to_string());
            let config_path = config.unwrap_or_else(|| "guardstack.config.toml".to_string());

            execute(path, config_path)?;
        }
        Commands::Init => init::init_env()?,
        Commands::Scan { path } => {
            println!("\n🔍 GuardStack - scanning for secret leaks...\n");

            let results = scan_files(path.as_deref());

            if results.is_empty() {
                println!("✅ No secret leaks detected in scanned files!");
            } else {
                println!("{}", "=== Secret Leaks Detected ===".red().bold());
                for result in &results {
                    println!(
                        "❌ {} line {} -> {}",
                        result.file.yellow(),
                        result.line_number.to_string().cyan(),
                        result.reason.red()
                    );
                    println!("   {}", result.line.dimmed());
                }
                println!("\n⚠️    {} potential secret leak(s) found!!", results.len());
            }
        }
    }
    Ok(())
}

fn execute(path: String, config_path: String) -> anyhow::Result<()> {
    println!("\n🔍 GuardStack - scanning .env...");

    // load config
    let config = config::load_config(&config_path);

    let (lines_map, warnings) = parser::parser_env(&path)?;

    // Warnings Section
    if !warnings.is_empty() {
        println!("{}", "\n=== Warning(s) ===".yellow().bold());
        for warning in &warnings {
            println!("{}", warning.yellow());
        }
    }

    // Validation Errors Section
    let valid = validator::validate_env(&lines_map, &config);
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
    Ok(())
}
