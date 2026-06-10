use futures::future::BoxFuture;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, args: &Value) -> BoxFuture<'static, Result<Value, String>>;
}

pub struct ToolRunner {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRunner {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    pub async fn execute(&self, name: &str, args: &Value) -> Result<Value, String> {
        if let Some(tool) = self.get_tool(name) {
            tool.execute(args).await
        } else {
            Err(format!("Unknown tool: {}", name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future;

    struct DummyTool;

    impl Tool for DummyTool {
        fn name(&self) -> &str {
            "dummy_tool"
        }

        fn execute(&self, args: &Value) -> BoxFuture<'static, Result<Value, String>> {
            let res = if args.get("fail").is_some() {
                Err("Tool failed".to_string())
            } else {
                Ok(serde_json::json!({"status": "success"}))
            };
            Box::pin(future::ready(res))
        }
    }

    #[tokio::test]
    async fn test_register_and_execute() {
        let mut runner = ToolRunner::new();
        runner.register(Arc::new(DummyTool));

        // Test successful execution
        let args = serde_json::json!({});
        let result = runner.execute("dummy_tool", &args).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), serde_json::json!({"status": "success"}));

        // Test failing execution
        let fail_args = serde_json::json!({"fail": true});
        let fail_result = runner.execute("dummy_tool", &fail_args).await;
        assert!(fail_result.is_err());
        assert_eq!(fail_result.unwrap_err(), "Tool failed");

        // Test unknown tool
        let unknown_result = runner.execute("unknown", &args).await;
        assert!(unknown_result.is_err());
        assert_eq!(unknown_result.unwrap_err(), "Unknown tool: unknown");
    }
}

impl Default for ToolRunner {
    fn default() -> Self {
        Self::new()
    }
}
