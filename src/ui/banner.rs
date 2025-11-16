//! Banner and version display

use colored::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "Hassan Bazzoun <hassan.bazzoundev@gmail.com>";

/// Banner display utilities
pub struct Banner;

impl Banner {
    /// Show welcome banner
    pub fn show_welcome() {
        println!();
        println!(
            "{}",
            "╔═══════════════════════════════════════════════════════════════╗"
                .cyan()
                .bold()
        );
        println!(
            "{}",
            "║              Bazzounquester - Interactive Mode                ║"
                .cyan()
                .bold()
        );
        println!(
            "{}",
            "║          Your Postman in the Terminal - HTTP CLI Tool        ║"
                .cyan()
                .bold()
        );
        println!(
            "{}",
            "╚═══════════════════════════════════════════════════════════════╝"
                .cyan()
                .bold()
        );
        println!();
        println!(
            "  {} v{}",
            "Version:".bright_black(),
            VERSION.bright_white()
        );
        println!("  {} {}", "Author:".bright_black(), AUTHOR.bright_white());
        println!();
        println!(
            "{}",
            "  Type 'help' for commands | 'version' for info | 'exit' to quit".bright_black()
        );
        println!();
    }

    /// Show version information
    pub fn show_version() {
        println!();
        println!(
            "{}",
            "╔═══════════════════════════════════════════════════════════════╗".cyan()
        );
        println!(
            "║                      {} v{}                     ║",
            "Bazzounquester".bright_white().bold(),
            VERSION.bright_white()
        );
        println!(
            "{}",
            "╚═══════════════════════════════════════════════════════════════╝".cyan()
        );
        println!();
        println!("  {} {}", "Author:".bright_black(), AUTHOR.bright_white());
        println!(
            "  {} {}",
            "Description:".bright_black(),
            "HTTP request CLI tool with interactive mode".bright_white()
        );
        println!("  {} {}", "License:".bright_black(), "MIT".bright_white());
        println!();
        println!(
            "  {} A powerful tool to make HTTP requests from your terminal",
            "*".bright_black()
        );
        println!(
            "  {} Supports GET, POST, PUT, DELETE, PATCH methods",
            "*".green()
        );
        println!(
            "  {} Interactive REPL mode with command history",
            "*".green()
        );
        println!("  {} Pretty-printed JSON responses", "*".green());
        println!("  {} Custom headers and query parameters", "*".green());
        println!();
    }
}
