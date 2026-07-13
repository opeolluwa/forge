use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{constants::DATABASE_URL, errors::file_system::FileSystemError};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForgeConfig {
    pub scripts: ScriptConfig,
    pub env: EnvConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptConfig {
    pub runner: String,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvConfig {
    pub database_url: String,
}
impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            database_url: DATABASE_URL.to_string(),
        }
    }
}

impl Default for ScriptConfig {
    fn default() -> Self {
        Self {
            runner: "python3".to_string(),
            source: Default::default(),
        }
    }
}
impl ForgeConfig {
    const APP_NAME: &str = "forge";
    // const
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn save(&self) -> Result<(), FileSystemError> {
        confy::store(Self::APP_NAME, None, self)?;

        Ok(())
    }

    pub fn load() -> Result<Self, FileSystemError> {
        let cfg: ForgeConfig = confy::load(Self::APP_NAME, None)?;
        Ok(cfg)
    }

    pub fn create(&self) -> Result<(), FileSystemError> {
        unimplemented!()
    }

    pub fn file_path(&self) -> Result<PathBuf, FileSystemError> {
        let path = confy::get_configuration_file_path(Self::APP_NAME, None)?;

        Ok(path)
    }
}
