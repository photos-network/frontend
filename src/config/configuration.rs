use serde::{Deserialize, Serialize};
use tracing::info;
use std::{fmt, fs, sync::OnceLock};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub core_url: String,
}

impl Configuration {
    pub fn new(path: &str) -> Option<Self> {
        info!("Load configuration file {}", path);

        let read_file_result = fs::read_to_string(path);

        let config = match read_file_result {
            Ok(data) => serde_json::from_str(&data)
                .expect("Configuration file could not be parsed as JSON!"),
            Err(_) => {
                let default_config = Configuration::empty();

                fs::write(path, serde_json::to_string_pretty(&default_config).unwrap())
                    .expect("Could not write default Configuration to file!");

                default_config
            }
        };

        Some(config)
    }
    pub fn empty() -> Self {
        Configuration {
            core_url: "127.0.0.1".into(),
        }
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, "\n\tcore_url: {}", self.core_url)?;
        write!(f, "\n}}")
    }
}

static CONFIG: OnceLock<Configuration> = OnceLock::new();

pub fn config() -> &'static Configuration {
    CONFIG.get_or_init(|| {
        let path = std::env::var("FRONTEND_CONFIG").unwrap_or_else(|_| "config/frontend.json".to_string());
        let contents = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read config file: {}", path));
        serde_json::from_str(&contents)
            .unwrap_or_else(|e| panic!("Failed to parse config file {}: {}", path, e))
    })
}

pub fn core_url(path: &str) -> String {
    format!("{}{}", config().core_url.trim_end_matches('/'), path)
}


