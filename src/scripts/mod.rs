//! Pre-request and post-response scripting support

pub mod context;
pub mod engine;
pub mod types;

pub use context::{ScriptContext, ScriptVariable};
pub use engine::ScriptEngine;
pub use types::{Script, ScriptType};

use crate::error::Result;

/// Execute a pre-request script
pub fn execute_pre_request(
    script: &Script,
    context: &mut ScriptContext,
) -> Result<()> {
    if script.script_type != ScriptType::PreRequest {
        return Ok(());
    }

    let mut engine = ScriptEngine::new();
    engine.execute(script, context)
}

/// Execute a post-response script
pub fn execute_post_response(
    script: &Script,
    context: &mut ScriptContext,
) -> Result<()> {
    if script.script_type != ScriptType::PostResponse {
        return Ok(());
    }

    let mut engine = ScriptEngine::new();
    engine.execute(script, context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_pre_request() {
        let script = Script::pre_request("let x = 5;".to_string());
        let mut context = ScriptContext::new();

        let result = execute_pre_request(&script, &mut context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_post_response() {
        let script = Script::post_response("let y = 10;".to_string());
        let mut context = ScriptContext::new();

        let result = execute_post_response(&script, &mut context);
        assert!(result.is_ok());
    }
}
