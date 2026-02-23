#[derive(Debug, Clone)]
pub struct Config {
    pub endpoint: String,
    pub project_key: String,
    pub heartbeat_interval_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            endpoint: "http://localhost:3000/beacon".to_string(),
            project_key: "p123456789".to_string(),
            heartbeat_interval_ms: 10_000,
        }
    }
}
