//! CLI command definitions

use clap::{Parser, Subcommand};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "Hassan Bazzoun <hassan.bazzoundev@gmail.com>";

/// Main CLI structure
#[derive(Parser)]
#[command(name = "bazzounquester")]
#[command(author = AUTHOR)]
#[command(version = VERSION)]
#[command(about = "A powerful HTTP request CLI tool - Your Postman in the terminal", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// Start interactive mode
    Interactive,

    /// Send a GET request
    Get {
        /// URL to send the request to
        url: String,

        /// Headers in format "Key:Value" (can be specified multiple times)
        #[arg(short = 'H', long)]
        header: Vec<String>,

        /// Query parameters in format "key=value" (can be specified multiple times)
        #[arg(short, long)]
        query: Vec<String>,
    },

    /// Send a POST request
    Post {
        /// URL to send the request to
        url: String,

        /// Headers in format "Key:Value" (can be specified multiple times)
        #[arg(short = 'H', long)]
        header: Vec<String>,

        /// JSON body as a string
        #[arg(short, long)]
        body: Option<String>,

        /// Query parameters in format "key=value" (can be specified multiple times)
        #[arg(short, long)]
        query: Vec<String>,
    },

    /// Send a PUT request
    Put {
        /// URL to send the request to
        url: String,

        /// Headers in format "Key:Value" (can be specified multiple times)
        #[arg(short = 'H', long)]
        header: Vec<String>,

        /// JSON body as a string
        #[arg(short, long)]
        body: Option<String>,

        /// Query parameters in format "key=value" (can be specified multiple times)
        #[arg(short, long)]
        query: Vec<String>,
    },

    /// Send a DELETE request
    Delete {
        /// URL to send the request to
        url: String,

        /// Headers in format "Key:Value" (can be specified multiple times)
        #[arg(short = 'H', long)]
        header: Vec<String>,

        /// Query parameters in format "key=value" (can be specified multiple times)
        #[arg(short, long)]
        query: Vec<String>,
    },

    /// Send a PATCH request
    Patch {
        /// URL to send the request to
        url: String,

        /// Headers in format "Key:Value" (can be specified multiple times)
        #[arg(short = 'H', long)]
        header: Vec<String>,

        /// JSON body as a string
        #[arg(short, long)]
        body: Option<String>,

        /// Query parameters in format "key=value" (can be specified multiple times)
        #[arg(short, long)]
        query: Vec<String>,
    },
}
