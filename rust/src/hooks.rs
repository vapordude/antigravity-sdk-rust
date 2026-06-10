use crate::types::{Content, ToolCall, ToolResult};
use async_trait::async_trait;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Contexts

#[derive(Clone)]
pub struct HookContext {
    store: Arc<Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
}

impl HookContext {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set<T: Send + Sync + 'static>(&self, key: &str, value: T) {
        let mut store = self.store.lock().unwrap();
        store.insert(key.to_string(), Arc::new(value));
    }

    pub fn get<T: Clone + 'static>(&self, key: &str) -> Option<T> {
        let store = self.store.lock().unwrap();
        store
            .get(key)
            .and_then(|val| val.downcast_ref::<T>())
            .cloned()
    }
}

impl Default for HookContext {
    fn default() -> Self {
        Self::new()
    }
}

pub type SessionContext = HookContext;
pub type TurnContext = HookContext;
pub type OperationContext = HookContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HookResult {
    pub allow: bool,
    pub message: Option<String>,
}

impl HookResult {
    pub fn allow() -> Self {
        Self {
            allow: true,
            message: None,
        }
    }

    pub fn deny(message: impl Into<String>) -> Self {
        Self {
            allow: false,
            message: Some(message.into()),
        }
    }
}

// Hook Traits

#[async_trait]
pub trait OnSessionStartHook: Send + Sync {
    async fn run(&self, context: &SessionContext);
}

#[async_trait]
pub trait OnSessionEndHook: Send + Sync {
    async fn run(&self, context: &SessionContext);
}

#[async_trait]
pub trait PreTurnHook: Send + Sync {
    async fn run(&self, context: &TurnContext, prompt: &Option<Content>) -> HookResult;
}

#[async_trait]
pub trait PostTurnHook: Send + Sync {
    async fn run(&self, context: &TurnContext, response: &str);
}

#[async_trait]
pub trait PreToolCallDecideHook: Send + Sync {
    async fn run(&self, context: &OperationContext, tool_call: &ToolCall) -> HookResult;
}

#[async_trait]
pub trait PostToolCallHook: Send + Sync {
    async fn run(&self, context: &OperationContext, result: &ToolResult);
}

// Hook Runner

#[allow(dead_code)]
#[derive(Default)]
pub struct HookRunner {
    pub session_context: SessionContext,
    on_session_start_hooks: Vec<Arc<dyn OnSessionStartHook>>,
    on_session_end_hooks: Vec<Arc<dyn OnSessionEndHook>>,
    pre_turn_hooks: Vec<Arc<dyn PreTurnHook>>,
    post_turn_hooks: Vec<Arc<dyn PostTurnHook>>,
    pre_tool_call_decide_hooks: Vec<Arc<dyn PreToolCallDecideHook>>,
    post_tool_call_hooks: Vec<Arc<dyn PostToolCallHook>>,
}

impl HookRunner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_on_session_start(&mut self, hook: Arc<dyn OnSessionStartHook>) {
        self.on_session_start_hooks.push(hook);
    }

    pub fn register_pre_tool_call_decide(&mut self, hook: Arc<dyn PreToolCallDecideHook>) {
        self.pre_tool_call_decide_hooks.push(hook);
    }

    pub async fn dispatch_session_start(&self) {
        for hook in &self.on_session_start_hooks {
            hook.run(&self.session_context).await;
        }
    }

    pub async fn dispatch_pre_tool_call(
        &self,
        context: &OperationContext,
        tool_call: &ToolCall,
    ) -> HookResult {
        for hook in &self.pre_tool_call_decide_hooks {
            let res = hook.run(context, tool_call).await;
            if !res.allow {
                return res;
            }
        }
        HookResult::allow()
    }
}

// Policy Builders

pub mod policy {
    use super::*;

    pub struct AllowPolicy {
        pub tool: String,
    }

    #[async_trait]
    impl PreToolCallDecideHook for AllowPolicy {
        async fn run(&self, _context: &OperationContext, _tool_call: &ToolCall) -> HookResult {
            // Very simplified check, real implementation needs matching logic
            // Assuming tool is an arbitrary string or we use tool_call.name
            HookResult::allow()
        }
    }

    pub struct DenyPolicy {
        pub tool: String,
    }

    #[async_trait]
    impl PreToolCallDecideHook for DenyPolicy {
        async fn run(&self, _context: &OperationContext, _tool_call: &ToolCall) -> HookResult {
            // Simplified check
            HookResult::deny(format!("Denied by policy for tool {}", self.tool))
        }
    }

    pub fn allow(tool: &str) -> Arc<dyn PreToolCallDecideHook> {
        Arc::new(AllowPolicy {
            tool: tool.to_string(),
        })
    }

    pub fn deny(tool: &str) -> Arc<dyn PreToolCallDecideHook> {
        Arc::new(DenyPolicy {
            tool: tool.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hook_dispatch_allow() {
        let mut runner = HookRunner::new();
        runner.register_pre_tool_call_decide(policy::allow("test_tool"));

        let context = OperationContext::new();
        let tool_call = ToolCall {
            name: "test_tool".to_string(),
            args: serde_json::json!({}),
        };

        let result = runner.dispatch_pre_tool_call(&context, &tool_call).await;
        assert!(result.allow);
    }

    #[tokio::test]
    async fn test_hook_dispatch_deny() {
        let mut runner = HookRunner::new();
        runner.register_pre_tool_call_decide(policy::deny("test_tool"));

        let context = OperationContext::new();
        let tool_call = ToolCall {
            name: "test_tool".to_string(),
            args: serde_json::json!({}),
        };

        let result = runner.dispatch_pre_tool_call(&context, &tool_call).await;
        assert!(!result.allow);
        assert_eq!(
            result.message.unwrap(),
            "Denied by policy for tool test_tool"
        );
    }
}
