//! Environment variables and configuration management

pub mod environment;
pub mod manager;
pub mod substitution;

pub use environment::Environment;
pub use manager::EnvironmentManager;
pub use substitution::VariableSubstitutor;
