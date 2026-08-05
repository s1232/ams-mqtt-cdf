use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub cognite: CogniteConfig,
}

#[derive(Deserialize)]
pub struct MqttConfig {
    pub broker_host: String,
    pub broker_port: u16,
    pub topic: String,
}

#[derive(Deserialize)]
pub struct CogniteConfig {
    pub client_id: String,
    pub client_secret: String,
    pub token_url: String,
    pub base_url: String,
    pub project: String,
    pub timeseries_space: String,
    pub timeseries_external_id: String,
}

pub fn load() -> Config {
    let path = std::env::var("AMSCLIENT_CONFIG").unwrap_or_else(|_| "config.toml".to_string());
    let contents =
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("Could not read {path}: {e}"));
    toml::from_str(&contents).unwrap_or_else(|e| panic!("Could not parse {path}: {e}"))
}
