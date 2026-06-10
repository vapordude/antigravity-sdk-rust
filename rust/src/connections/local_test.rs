#[cfg(test)]
mod tests {
    use super::super::local_config::{LocalConnectionConfig, LocalConnectionStrategy};
    use crate::connections::connection::ConnectionStrategy;

    #[tokio::test]
    async fn test_local_connection_strategy_setup_failure() {
        let config = LocalConnectionConfig {
            harness_path: "/nonexistent/path/to/harness".to_string(),
        };

        let mut strategy = LocalConnectionStrategy::new(config);
        let result = strategy.setup().await;
        assert!(
            result.is_err(),
            "Setup should fail with a nonexistent harness path"
        );
    }
}
