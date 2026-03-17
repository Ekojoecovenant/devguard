use clap::{Parser, Subcommand};
use colored::Colorize;

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
}

fn main() {
    // clap reads args automatcally here
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { path } => {
            let path = path.unwrap_or(".env".to_string());

            execute(path);
        }
    }
}

fn execute(path: String) {
    println!("\n🔍 StackSafe - scanning .env...\n");
    let lines_map = parser::parser_env(&path);
    let valid = validator::validate_env(lines_map);

    for valid_error in &valid {
        println!("❌ {} -> {}", valid_error.key.red(), valid_error.message)
    }
    if valid.is_empty() {
        println!("✅ All checks passed! Your .env looks good!");
    } else {
        println!("\n⚠️  {} issue(s) found", valid.len());
    }
}
