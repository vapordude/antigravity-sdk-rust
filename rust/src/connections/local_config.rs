use crate::connections::connection::Connection;
use crate::connections::connection::ConnectionStrategy;
use crate::connections::local::LocalConnection;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConnectionConfig {
    pub harness_path: String,
}

pub struct LocalConnectionStrategy {
    config: LocalConnectionConfig,
    connection: Option<LocalConnection>,
}

impl LocalConnectionStrategy {
    pub fn new(config: LocalConnectionConfig) -> Self {
        Self {
            config,
            connection: None,
        }
    }
}

#[async_trait]
impl ConnectionStrategy for LocalConnectionStrategy {
    fn connect(&self) -> Box<dyn Connection> {
        // In a real implementation this might return a clone or handle state better
        unimplemented!("connect is handled in setup/connect pattern");
    }

    async fn setup(&mut self) -> Result<()> {
        let conn = LocalConnection::new(&self.config.harness_path).await?;
        self.connection = Some(conn);
        Ok(())
    }

    async fn teardown(&mut self) -> Result<()> {
        if let Some(mut conn) = self.connection.take() {
            conn.disconnect().await?;
        }
        Ok(())
    }
}
