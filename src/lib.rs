//! Bazzounquester - A powerful HTTP request CLI tool
//!
//! This library provides the core functionality for making HTTP requests,
//! managing collections, and handling various API testing scenarios.

pub mod assertions;
pub mod auth;
pub mod cli;
pub mod collections;
pub mod env;
pub mod error;
pub mod history;
pub mod http;
pub mod repl;
pub mod scripts;
pub mod session;
pub mod ui;
pub mod upload;
pub mod workflow;

pub use error::{Error, Result};
