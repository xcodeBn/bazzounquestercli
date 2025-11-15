//! Bazzounquester - A powerful HTTP request CLI tool
//! Author: Hassan Bazzoun <hassan.bazzoundev@gmail.com>
//! License: MIT

use bazzounquester::{
    cli::{Cli, Commands},
    http::{HttpClient, HttpMethod, RequestBuilder, ResponseFormatter},
    repl::ReplMode,
};
use clap::Parser;
use colored::*;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        None | Some(Commands::Interactive) => {
            if let Err(e) = run_interactive_mode() {
                eprintln!("{} {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
        Some(Commands::Get { url, header, query }) => {
            execute_request(HttpMethod::Get, &url, header, None, query);
        }
        Some(Commands::Post {
            url,
            header,
            body,
            query,
        }) => {
            execute_request(HttpMethod::Post, &url, header, body, query);
        }
        Some(Commands::Put {
            url,
            header,
            body,
            query,
        }) => {
            execute_request(HttpMethod::Put, &url, header, body, query);
        }
        Some(Commands::Delete { url, header, query }) => {
            execute_request(HttpMethod::Delete, &url, header, None, query);
        }
        Some(Commands::Patch {
            url,
            header,
            body,
            query,
        }) => {
            execute_request(HttpMethod::Patch, &url, header, body, query);
        }
    }
}

fn run_interactive_mode() -> bazzounquester::Result<()> {
    let mut repl = ReplMode::new()?;
    repl.run()
}

fn execute_request(
    method: HttpMethod,
    url: &str,
    headers: Vec<String>,
    body: Option<String>,
    query_params: Vec<String>,
) {
    // Build request
    let mut request = RequestBuilder::new(method, url.to_string());

    if !headers.is_empty() {
        request = request.headers(headers);
    }

    if !query_params.is_empty() {
        request = request.queries(query_params);
    }

    if let Some(b) = body {
        request = request.body(b);
    }

    // Display request info
    println!();
    println!(
        "{} {}",
        "→".blue().bold(),
        format!("{} {}", method.as_str(), url).bold()
    );
    println!();

    // Execute request
    let client = HttpClient::new();
    match client.execute(&request) {
        Ok(response) => {
            print!("{}", ResponseFormatter::format(&response));
        }
        Err(e) => {
            eprintln!();
            eprintln!("{} {}", "✗".red().bold(), e);
            eprintln!();
            std::process::exit(1);
        }
    }
}
