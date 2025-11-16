//! Request chaining and workflow execution

pub mod chain;
pub mod executor;
pub mod step;

pub use chain::{ChainConfig, RequestChain};
pub use executor::{ExecutionResult, WorkflowExecutor};
pub use step::{StepResult, WorkflowStep};

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
        // Basic module test - verify module can be imported
        let chain = RequestChain::new("test".to_string());
        assert_eq!(chain.name, "test");
    }
}
