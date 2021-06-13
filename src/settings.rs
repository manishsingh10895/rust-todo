use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::fmt;

const CONFIG_FILE_PATH: &str = "./config/Default.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";

#[derive(Debug, Clone, Deserialize)]
pub enum ENV {
    Development,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Db {
    pub url: String,
    pub user: String,
    pub pass: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub db: Db,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or(String::from("Default"));
        let mut s = Config::new();

        s.set("env", env.clone())?;

        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

        s.merge(Environment::with_prefix("ea").separator("__"))?;

        s.try_into()
    }
}
