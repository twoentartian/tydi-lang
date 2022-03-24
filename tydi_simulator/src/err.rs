use serde_json;

#[derive(Debug)]
pub enum SimulatorError {
    UnspecifiedError(String),
    JsonError(serde_json::Error),
    ConfigError(String),
}