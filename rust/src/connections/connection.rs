use crate::types;
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;

#[async_trait]
pub trait Connection: Send + Sync {
    fn is_idle(&self) -> bool {
        true
    }

    fn conversation_id(&self) -> String {
        String::new()
    }

    async fn send(&mut self, prompt: Option<types::Content>) -> Result<()>;

    // Receive steps as they occur
    fn receive_steps(&mut self) -> Result<BoxStream<'static, types::Step>>;

    async fn disconnect(&mut self) -> Result<()> {
        Ok(())
    }

    async fn cancel(&mut self) -> Result<()> {
        Ok(())
    }

    async fn delete(&mut self) -> Result<()> {
        Ok(())
    }

    async fn signal_idle(&mut self) -> Result<()> {
        Ok(())
    }

    async fn wait_for_idle(&mut self) -> Result<()> {
        Ok(())
    }

    async fn wait_for_wakeup(&mut self, _timeout: f64) -> Result<bool> {
        Ok(false)
    }

    async fn send_tool_results(&mut self, _results: Vec<types::ToolResult>) -> Result<()> {
        Ok(())
    }

    async fn send_trigger_notification(&mut self, content: String) -> Result<()>;
}

#[async_trait]
pub trait ConnectionStrategy {
    fn connect(&self) -> Box<dyn Connection>;
    async fn setup(&mut self) -> Result<()>;
    async fn teardown(&mut self) -> Result<()>;
}
