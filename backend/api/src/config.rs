use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct FrontendConfigs {
    pub web: ServerConfig,
    pub admin: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub frontend: FrontendConfigs,
}

impl AppConfig {
    /// Load configuration from config.toml file
    /// Looks for config.toml in the project root directory
    pub fn load() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config"))
            .build()?;

        config.try_deserialize()
    }

    /// Get the server address in the format "host:port" or just "host" if port is not specified
    pub fn server_address(&self) -> String {
        match self.server.port {
            Some(port) => format!("{}:{}", self.server.host, port),
            None => self.server.host.clone(),
        }
    }

    /// Get allowed CORS origins from frontend configurations
    pub fn cors_origins(&self) -> Vec<String> {
        let mut origins = Vec::new();

        // Add web frontend origins
        match self.frontend.web.port {
            Some(port) => {
                origins.push(format!("http://{}:{}", self.frontend.web.host, port));
                origins.push(format!("http://localhost:{}", port));
            }
            None => {
                origins.push(self.frontend.web.host.clone());
            }
        }

        // Add admin frontend origins
        match self.frontend.admin.port {
            Some(port) => {
                origins.push(format!("http://{}:{}", self.frontend.admin.host, port));
                origins.push(format!("http://localhost:{}", port));
            }
            None => {
                origins.push(self.frontend.admin.host.clone());
            }
        }

        origins
    }
}
