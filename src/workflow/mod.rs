//! Request chaining and workflow execution

pub mod chain;
pub mod executor;
pub mod step;

pub use chain::{RequestChain, ChainConfig};
pub use executor::{WorkflowExecutor, ExecutionResult};
pub use step::{WorkflowStep, StepResult};

use crate::error::Result;

/// Execute a workflow chain
pub fn execute_chain(chain: &RequestChain) -> Result<ExecutionResult> {
    let executor = WorkflowExecutor::new();
    executor.execute(chain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_module() {
        // Basic module test
        assert!(true);
    }
}
