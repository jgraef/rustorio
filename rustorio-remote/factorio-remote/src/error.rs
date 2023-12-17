use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Rcon error: {0}")]
    Rcon(#[from] rcon::Error),
    #[error("Lua error: {0}")]
    Lua(#[from] LuaError),
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("Initial ping failed")]
    InitialPingFailed,
    #[error("Missing response")]
    MissingResponse,
    #[error("Invalid value returned for blueprint import: {0}")]
    InvalidImportBlueprintResult(i32),
    #[error("Malformed packet: {0}")]
    MalformedPacket(String),
    #[error("Incorrect pong: expected {expected}, but got {got}")]
    IncorrectPong { expected: i32, got: i32 },
}

#[derive(Clone, Debug, Deserialize, Error)]
#[error("Remote Error[{}]: {}", {code}, {message})]
pub struct LuaError {
    code: i32,
    message: String,
}

impl Default for LuaError {
    fn default() -> Self {
        Self {
            code: 0,
            message: "Unknown error".to_owned(),
        }
    }
}
