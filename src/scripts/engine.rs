//! Script execution engine

use crate::error::{Error, Result};
use crate::scripts::{Script, ScriptContext};
use rhai::{Dynamic, Engine, Map, Scope};
use std::sync::{Arc, Mutex};

/// Script execution engine
pub struct ScriptEngine {
    /// Rhai engine
    engine: Engine,

    /// Console log storage
    console_logs: Arc<Mutex<Vec<String>>>,
}

impl ScriptEngine {
    /// Create a new script engine
    pub fn new() -> Self {
        let mut engine = Engine::new();
        let console_logs = Arc::new(Mutex::new(Vec::new()));

        // Register console.log function
        let logs_clone = Arc::clone(&console_logs);
        engine.register_fn("log", move |message: &str| {
            if let Ok(mut logs) = logs_clone.lock() {
                logs.push(message.to_string());
            }
        });

        Self {
            engine,
            console_logs,
        }
    }

    /// Execute a script
    pub fn execute(&mut self, script: &Script, context: &mut ScriptContext) -> Result<()> {
        if !script.should_execute() {
            return Ok(());
        }

        // Clear previous console logs
        if let Ok(mut logs) = self.console_logs.lock() {
            logs.clear();
        }

        // Create scope
        let mut scope = Scope::new();

        // Add context variables to scope
        for (name, var) in context.variables() {
            scope.push(name.clone(), var.value.clone());
        }

        // Add request data as a map
        let mut req_map = Map::new();
        for (key, value) in context.request_data() {
            req_map.insert(key.clone().into(), Dynamic::from(value.clone()));
        }
        scope.push_constant("request", req_map);

        // Add response data as a map
        let mut res_map = Map::new();
        for (key, value) in context.response_data() {
            res_map.insert(key.clone().into(), Dynamic::from(value.clone()));
        }
        scope.push_constant("response", res_map);

        // Execute script
        let _ = self
            .engine
            .eval_with_scope::<Dynamic>(&mut scope, &script.code)
            .map_err(|e| Error::InvalidCommand(format!("Script execution error: {}", e)))?;

        // Extract modified variables back to context
        // Clear existing variables
        let var_names: Vec<String> = context.variables().keys().cloned().collect();
        for name in var_names {
            context.remove_variable(&name);
        }

        // Add all scope variables
        for (name, _, value) in scope.iter() {
            if name != "request" && name != "response" {
                if let Ok(val_str) = value.clone().into_string() {
                    context.set_variable(name.to_string(), val_str);
                }
            }
        }

        // Extract console logs
        if let Ok(logs) = self.console_logs.lock() {
            for log in logs.iter() {
                context.console_log(log.clone());
            }
        }

        Ok(())
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scripts::ScriptType;

    #[test]
    fn test_engine_creation() {
        let _engine = ScriptEngine::new();
    }

    #[test]
    fn test_execute_simple_script() {
        let mut engine = ScriptEngine::new();
        let script = Script::new(ScriptType::PreRequest, "let x = 5 + 3;".to_string());
        let mut context = ScriptContext::new();

        let result = engine.execute(&script, &mut context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_set_variable() {
        let mut engine = ScriptEngine::new();
        let script = Script::new(
            ScriptType::PreRequest,
            "let test = \"value123\";".to_string(),
        );
        let mut context = ScriptContext::new();

        engine.execute(&script, &mut context).unwrap();
        assert_eq!(context.get_variable_value("test"), Some("value123"));
    }

    #[test]
    fn test_execute_modify_variable() {
        let mut engine = ScriptEngine::new();
        let mut context = ScriptContext::new();
        context.set_variable("existing".to_string(), "hello".to_string());

        let script = Script::new(ScriptType::PreRequest, "let copy = existing;".to_string());

        engine.execute(&script, &mut context).unwrap();
        assert_eq!(context.get_variable_value("copy"), Some("hello"));
    }

    #[test]
    fn test_execute_console_log() {
        let mut engine = ScriptEngine::new();
        let script = Script::new(
            ScriptType::PreRequest,
            "log(\"Test message\"); log(\"Another message\");".to_string(),
        );
        let mut context = ScriptContext::new();

        engine.execute(&script, &mut context).unwrap();
        assert_eq!(context.console_output().len(), 2);
        assert_eq!(context.console_output()[0], "Test message");
        assert_eq!(context.console_output()[1], "Another message");
    }

    #[test]
    fn test_execute_request_data_access() {
        let mut engine = ScriptEngine::new();
        let mut context = ScriptContext::new();
        context.set_request_data("method".to_string(), "POST".to_string());

        let script = Script::new(
            ScriptType::PreRequest,
            "let req_method = request[\"method\"];".to_string(),
        );

        engine.execute(&script, &mut context).unwrap();
        assert_eq!(context.get_variable_value("req_method"), Some("POST"));
    }

    #[test]
    fn test_execute_response_data_access() {
        let mut engine = ScriptEngine::new();
        let mut context = ScriptContext::new();
        context.set_response_data("status".to_string(), "200".to_string());

        let script = Script::new(
            ScriptType::PostResponse,
            "let status_code = response[\"status\"];".to_string(),
        );

        engine.execute(&script, &mut context).unwrap();
        assert_eq!(context.get_variable_value("status_code"), Some("200"));
    }

    #[test]
    fn test_execute_disabled_script() {
        let mut engine = ScriptEngine::new();
        let script = Script::new(ScriptType::PreRequest, "invalid syntax here!".to_string())
            .with_enabled(false);
        let mut context = ScriptContext::new();

        // Should not execute and not error
        let result = engine.execute(&script, &mut context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_empty_script() {
        let mut engine = ScriptEngine::new();
        let script = Script::new(ScriptType::PreRequest, "  ".to_string());
        let mut context = ScriptContext::new();

        // Should not execute
        let result = engine.execute(&script, &mut context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_invalid_script() {
        let mut engine = ScriptEngine::new();
        let script = Script::new(
            ScriptType::PreRequest,
            "this is not valid rhai syntax @#$%".to_string(),
        );
        let mut context = ScriptContext::new();

        let result = engine.execute(&script, &mut context);
        assert!(result.is_err());
    }
}
