use serde_json;

#[derive(Debug)]
pub enum SimulatorError {
    UnspecifiedError(String),
    FileNotExist(String),
    JsonError(serde_json::Error),
    ConfigError(String),
}