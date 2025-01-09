use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/rust_api_db".to_string());
        
        let host = std::env::var("SERVER__HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());
        
        let port = std::env::var("SERVER__PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .unwrap_or(8080);

        Ok(Config {
            database_url,
            server: ServerConfig { host, port },
        })
    }
}
