//! Interactive REPL implementation

use crate::cli::CommandParser;
use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::ui::{Banner, Help};
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

/// Interactive REPL mode handler
pub struct ReplMode {
    editor: DefaultEditor,
    client: HttpClient,
}

impl ReplMode {
    /// Create a new REPL mode instance
    pub fn new() -> Result<Self> {
        let editor = DefaultEditor::new()?;
        let client = HttpClient::new();

        Ok(Self { editor, client })
    }

    /// Run the interactive REPL
    pub fn run(&mut self) -> Result<()> {
        // Display welcome banner
        Banner::show_welcome();

        loop {
            let readline = self
                .editor
                .readline(&format!("{} ", "bazzounquester>".green().bold()));

            match readline {
                Ok(line) => {
                    if line.trim().is_empty() {
                        continue;
                    }

                    self.editor.add_history_entry(line.as_str()).ok();

                    let trimmed = line.trim();

                    // Handle built-in commands
                    if self.handle_builtin_command(trimmed)? {
                        continue;
                    }

                    // Handle HTTP commands
                    if let Err(e) = self.handle_http_command(&line) {
                        eprintln!();
                        eprintln!("{} {}", "✗".red().bold(), e);
                        eprintln!();
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!(
                        "{}",
                        "CTRL-C pressed. Use 'exit' or 'quit' to exit.".yellow()
                    );
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("{}", "CTRL-D pressed. Exiting...".cyan());
                    break;
                }
                Err(e) => {
                    return Err(Error::from(e));
                }
            }
        }

        // Display goodbye message
        println!();
        println!("{}", "Thank you for using Bazzounquester!".cyan().bold());
        println!();

        Ok(())
    }

    /// Handle built-in commands (help, version, exit, etc.)
    /// Returns true if command was handled, false otherwise
    fn handle_builtin_command(&self, command: &str) -> Result<bool> {
        match command {
            "exit" | "quit" => {
                println!();
                println!("{}", "Thank you for using Bazzounquester!".cyan().bold());
                println!();
                std::process::exit(0);
            }
            "help" => {
                Help::show_interactive();
                Ok(true)
            }
            "version" | "--version" | "-v" => {
                Banner::show_version();
                Ok(true)
            }
            "clear" | "cls" => {
                print!("\x1B[2J\x1B[1;1H");
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// Handle HTTP commands
    fn handle_http_command(&self, input: &str) -> Result<()> {
        use crate::http::ResponseFormatter;

        // Parse command line
        let args = CommandParser::parse_line(input)?;

        if args.is_empty() {
            return Ok(());
        }

        let command = args[0].to_lowercase();

        // Check if it's a valid HTTP method
        match command.as_str() {
            "get" | "post" | "put" | "delete" | "patch" | "head" | "options" => {
                // Parse HTTP command
                let request = CommandParser::parse_http_command(&command, &args[1..])?;

                // Display request info
                println!();
                println!(
                    "{} {}",
                    "→".cyan().bold(),
                    format!("{} {}", request.method.as_str(), request.url)
                        .bright_white()
                        .bold()
                );
                println!();

                // Execute request
                let response = self.client.execute(&request)?;

                // Display response
                print!("{}", ResponseFormatter::format(&response));

                Ok(())
            }
            _ => Err(Error::InvalidCommand(format!(
                "Unknown command: '{}'. Type 'help' for available commands.",
                command
            ))),
        }
    }
}

impl Default for ReplMode {
    fn default() -> Self {
        Self::new().expect("Failed to create REPL mode")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_creation() {
        let result = ReplMode::new();
        assert!(result.is_ok());
    }

    // More integration tests would go here
}
