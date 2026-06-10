#[allow(clippy::all)]
pub mod localharness {
    include!(concat!(env!("OUT_DIR"), "/antigravity.localharness.rs"));
}

use crate::connections::connection::Connection;
use crate::types;
use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::stream::BoxStream;
use prost::Message;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

#[allow(dead_code)]
pub struct LocalConnection {
    harness_process: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl LocalConnection {
    pub async fn new(harness_path: &str) -> Result<Self> {
        let mut child = Command::new(harness_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .context("Failed to spawn harness process")?;

        let stdin = child.stdin.take().context("Failed to open stdin")?;
        let stdout = BufReader::new(child.stdout.take().context("Failed to open stdout")?);

        Ok(Self {
            harness_process: child,
            stdin,
            stdout,
        })
    }

    #[allow(dead_code)]
    async fn write_message<M: Message>(&mut self, msg: M) -> Result<()> {
        let mut buf = Vec::new();
        msg.encode(&mut buf)?;

        let len = buf.len() as u32;
        self.stdin.write_all(&len.to_le_bytes()).await?;
        self.stdin.write_all(&buf).await?;
        self.stdin.flush().await?;
        Ok(())
    }

    #[allow(dead_code)]
    async fn read_message<M: Message + Default>(&mut self) -> Result<M> {
        let mut len_buf = [0u8; 4];
        self.stdout.read_exact(&mut len_buf).await?;
        let len = u32::from_le_bytes(len_buf) as usize;

        let mut msg_buf = vec![0u8; len];
        self.stdout.read_exact(&mut msg_buf).await?;

        Ok(M::decode(&msg_buf[..])?)
    }
}

#[async_trait]
impl Connection for LocalConnection {
    async fn send(&mut self, _prompt: Option<types::Content>) -> Result<()> {
        Ok(())
    }

    fn receive_steps(&mut self) -> Result<BoxStream<'static, types::Step>> {
        // Since BoxStream requires returning a Stream of items,
        // we'll return an empty stream to satisfy the signature for Sprint 2.
        // It's a stub until Sprint 4 multi-turn state streaming.
        Ok(Box::pin(futures::stream::empty()))
    }

    async fn send_trigger_notification(&mut self, _content: String) -> Result<()> {
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        self.harness_process.kill().await?;
        Ok(())
    }
}
