//! Help text display

use colored::*;

/// Help text display utilities
pub struct Help;

impl Help {
    /// Show interactive mode help
    pub fn show_interactive() {
        println!();
        println!(
            "{}",
            "╔═══════════════════════════════════════════════════════════════╗"
                .bright_white()
        );
        println!(
            "{}",
            "║                        Available Commands                     ║"
                .bright_white()
                .bold()
        );
        println!(
            "{}",
            "╚═══════════════════════════════════════════════════════════════╝"
                .bright_white()
        );
        println!();
        println!("{}", "HTTP Methods:".bright_white().bold());
        println!();
        println!("  {} <url> [options]", "get".green().bold());
        println!(
            "    Options: {} \"Header:Value\"  {} \"key=value\"",
            "-H".yellow(),
            "-q".yellow()
        );
        println!();
        println!("  {} <url> [options]", "post".green().bold());
        println!(
            "    Options: {} \"Header:Value\"  {} \"key=value\"  {} '{}'",
            "-H".yellow(),
            "-q".yellow(),
            "-b".yellow(),
            "{\"key\":\"value\"}"
        );
        println!();
        println!("  {} <url> [options]", "put".green().bold());
        println!(
            "    Options: {} \"Header:Value\"  {} \"key=value\"  {} '{}'",
            "-H".yellow(),
            "-q".yellow(),
            "-b".yellow(),
            "{\"key\":\"value\"}"
        );
        println!();
        println!("  {} <url> [options]", "patch".green().bold());
        println!(
            "    Options: {} \"Header:Value\"  {} \"key=value\"  {} '{}'",
            "-H".yellow(),
            "-q".yellow(),
            "-b".yellow(),
            "{\"key\":\"value\"}"
        );
        println!();
        println!("  {} <url> [options]", "delete".green().bold());
        println!(
            "    Options: {} \"Header:Value\"  {} \"key=value\"",
            "-H".yellow(),
            "-q".yellow()
        );
        println!();
        println!("{}", "Built-in Commands:".bright_white().bold());
        println!("  {}      - Show this help message", "help".cyan());
        println!("  {}   - Show version and info", "version".cyan());
        println!("  {}     - Clear the screen", "clear".cyan());
        println!("  {}      - Exit interactive mode", "exit".cyan());
        println!();
        println!("{}", "Examples:".bright_white().bold());
        println!(
            "  {} get https://httpbin.org/get -q \"test=hello\"",
            "→".bright_black()
        );
        println!(
            "  {} post https://httpbin.org/post -H \"Content-Type:application/json\" -b '{{\"name\":\"John\"}}'",
            "→".bright_black()
        );
        println!(
            "  {} put https://api.example.com/users/1 -b '{{\"status\":\"active\"}}'",
            "→".bright_black()
        );
        println!();
    }
}
