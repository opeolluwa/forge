use std::{fs, path::PathBuf};

use mlua::Lua;
use serde::Deserialize;

use crate::{errors::file_system::FileSystemError, helpers::console::LogMessage};

const EXTENSIONS_DIR: &str = "extension";

#[derive(Debug, Deserialize)]
struct ExtensionArg {
    name: String,
    description: String,
    #[serde(default)]
    required: bool,
    #[serde(default)]
    interactive: bool,
}

#[derive(Debug, Deserialize)]
struct ExtensionManifest {
    name: String,
    description: String,
    #[serde(default)]
    args: Vec<ExtensionArg>,
    #[serde(default)]
    commands: Vec<String>,
}

fn load_manifest(name: &str) -> Result<ExtensionManifest, FileSystemError> {
    let yaml_path = PathBuf::from(EXTENSIONS_DIR).join(format!("{}.yaml", name));
    if !yaml_path.exists() {
        return Err(FileSystemError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("no extension '{}' found", name),
        )));
    }
    let content = fs::read_to_string(&yaml_path)?;
    serde_yaml::from_str(&content).map_err(|e| {
        FileSystemError::OperationError(format!("failed to parse extension manifest: {}", e))
    })
}

pub fn run_extension(name: &str) -> Result<(), FileSystemError> {
    let lua_path = PathBuf::from(EXTENSIONS_DIR).join(format!("{}.lua", name));
    if !lua_path.exists() {
        return Err(FileSystemError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("no extension '{}' found", name),
        )));
    }

    let script = fs::read_to_string(&lua_path)?;
    let lua = Lua::new();
    lua.load(&script)
        .exec()
        .map_err(|e| FileSystemError::OperationError(format!("lua error in '{}': {}", name, e)))?;

    Ok(())
}

pub fn show_extension_help(name: &str) -> Result<(), FileSystemError> {
    let manifest = load_manifest(name)?;

    LogMessage::info(&format!("Extension:   {}", manifest.name));
    LogMessage::neutral(&format!("Description: {}", manifest.description));

    if !manifest.args.is_empty() {
        println!("\nArguments:");
        for arg in &manifest.args {
            let mut flags: Vec<&str> = Vec::new();
            if arg.required {
                flags.push("required");
            }
            if arg.interactive {
                flags.push("interactive");
            }
            let flag_str = if flags.is_empty() {
                String::new()
            } else {
                format!("  [{}]", flags.join(", "))
            };
            LogMessage::neutral(&format!(
                "  {:<14} {}{}",
                arg.name, arg.description, flag_str
            ));
        }
    }

    if !manifest.commands.is_empty() {
        println!("\nCommands:");
        for cmd in &manifest.commands {
            LogMessage::neutral(&format!("  {}", cmd));
        }
    }

    LogMessage::neutral(&format!("\nRun with: x {}", manifest.name));

    Ok(())
}
