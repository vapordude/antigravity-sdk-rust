use crate::connections::connection::Connection;
use futures::future::BoxFuture;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct TriggerContext {
    connection: Arc<Mutex<Box<dyn Connection>>>,
}

impl TriggerContext {
    pub fn new(connection: Arc<Mutex<Box<dyn Connection>>>) -> Self {
        Self { connection }
    }

    pub async fn send(&self, content: String) -> anyhow::Result<()> {
        let mut conn = self.connection.lock().await;
        conn.send_trigger_notification(content).await
    }
}

pub trait Trigger: Send + Sync {
    fn run(&self, ctx: TriggerContext) -> BoxFuture<'static, ()>;
}

pub struct TriggerRunner {
    triggers: Vec<Arc<dyn Trigger>>,
    connection: Arc<Mutex<Box<dyn Connection>>>,
    tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl TriggerRunner {
    pub fn new(
        triggers: Vec<Arc<dyn Trigger>>,
        connection: Arc<Mutex<Box<dyn Connection>>>,
    ) -> Self {
        Self {
            triggers,
            connection,
            tasks: Vec::new(),
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        if !self.tasks.is_empty() {
            return Err(anyhow::anyhow!("TriggerRunner is already started."));
        }

        for trigger in &self.triggers {
            let ctx = TriggerContext::new(self.connection.clone());
            let trigger_clone = trigger.clone();

            let handle = tokio::spawn(async move {
                // Wrapper to catch unhandled errors from within trigger logic if we wanted to
                // but currently the `run` method returns unit type `()`.
                trigger_clone.run(ctx).await;
            });

            self.tasks.push(handle);
        }

        Ok(())
    }

    pub async fn stop(&mut self) {
        for task in &self.tasks {
            task.abort();
        }

        // Wait for tasks to be fully aborted
        for task in self.tasks.drain(..) {
            let _ = task.await; // Suppress Cancelled errors
        }
    }

    pub fn is_running(&self) -> bool {
        !self.tasks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types;
    use async_trait::async_trait;
    use futures::stream::BoxStream;

    struct MockConnection {
        messages: Arc<Mutex<Vec<String>>>,
    }

    #[async_trait]
    impl Connection for MockConnection {
        async fn send(&mut self, _prompt: Option<types::Content>) -> anyhow::Result<()> {
            Ok(())
        }
        fn receive_steps(&mut self) -> anyhow::Result<BoxStream<'static, types::Step>> {
            Ok(Box::pin(futures::stream::empty()))
        }
        async fn send_trigger_notification(&mut self, content: String) -> anyhow::Result<()> {
            self.messages.lock().await.push(content);
            Ok(())
        }
    }

    struct DummyTrigger;

    impl Trigger for DummyTrigger {
        fn run(&self, ctx: TriggerContext) -> BoxFuture<'static, ()> {
            Box::pin(async move {
                let _ = ctx.send("hello".to_string()).await;
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                let _ = ctx.send("world".to_string()).await;
            })
        }
    }

    #[tokio::test]
    async fn test_trigger_start_stop() {
        let messages = Arc::new(Mutex::new(Vec::new()));
        let conn = MockConnection {
            messages: messages.clone(),
        };
        let connection = Arc::new(Mutex::new(Box::new(conn) as Box<dyn Connection>));

        let trigger: Arc<dyn Trigger> = Arc::new(DummyTrigger);
        let mut runner = TriggerRunner::new(vec![trigger], connection);

        assert!(!runner.is_running());

        let start_res = runner.start().await;
        assert!(start_res.is_ok());
        assert!(runner.is_running());

        // Wait a bit for the first message to be sent
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Stop before the second message is sent
        runner.stop().await;

        let sent = messages.lock().await;
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0], "hello");
    }
}
