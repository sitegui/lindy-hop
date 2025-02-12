use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub thumbnail_hex_chars_prefix: usize,
    pub file_access_salt: String,
    pub file_access_iterations: u32,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        envy::from_env().context("failed to parse env vars into Config")
    }
}
