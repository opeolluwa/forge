use std::{
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    process::Command,
};

use dialoguer::{theme::ColorfulTheme, Select};

use crate::{
    constants::EXTENSIONS_DIR, errors::file_system::FileSystemError, helpers::console::LogMessage,
};

pub fn create_extension(name: &str) -> Result<(), FileSystemError> {
    let dir = PathBuf::from(EXTENSIONS_DIR.as_str());
    fs::create_dir_all(&dir)?;

    let lua_path = dir.join(format!("{}.lua", name));
    let yaml_path = dir.join(format!("{}.yaml", name));

    if lua_path.exists() {
        LogMessage::error(&format!("Extension '{}' already exists", name));
        return Err(FileSystemError::IoError(Error::new(
            ErrorKind::AlreadyExists,
            "extension already exists",
        )));
    }

    let lua_template =
        format!("-- {name}\n-- Write your extension logic here\n\nprint(\"Running {name}\")\n");

    let yaml_template =
        format!("name: {name}\ndescription: \"\"\nargs: []\ncommands:\n  - lua {name}.lua\n");

    fs::write(&lua_path, lua_template)?;
    fs::write(&yaml_path, yaml_template)?;

    LogMessage::success(&format!(
        "Created extension '{}' at {}",
        name,
        lua_path.display()
    ));

    Ok(())
}

pub fn update_extension(name: &str) -> Result<(), FileSystemError> {
    let lua_path = PathBuf::from(EXTENSIONS_DIR.as_str()).join(format!("{}.lua", name));

    if !lua_path.exists() {
        LogMessage::error(&format!("Extension '{}' not found", name));
        return Err(FileSystemError::IoError(Error::new(
            ErrorKind::NotFound,
            "extension not found",
        )));
    }

    let editors = ["Zed", "VS Code"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your preferred editor")
        .items(&editors)
        .default(0)
        .interact()
        .map_err(|e| FileSystemError::OperationError(e.to_string()))?;

    let editor_bin = match selection {
        0 => "zed",
        _ => "code",
    };

    Command::new(editor_bin)
        .arg(&lua_path)
        .spawn()
        .map_err(|e| {
            FileSystemError::OperationError(format!(
                "Failed to open editor '{}': {}",
                editor_bin, e
            ))
        })?;

    Ok(())
}

pub fn bootstrap_extensions(source_dir: &Path) -> Result<(), FileSystemError> {
    if !source_dir.is_dir() {
        LogMessage::error("Provided path is not a directory");
        return Err(FileSystemError::IoError(Error::new(
            ErrorKind::InvalidInput,
            "path is not a directory",
        )));
    }

    // Collect all .lua files that have a matching .yaml file
    let pairs: Vec<String> = fs::read_dir(source_dir)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if path.extension()?.to_str()? == "lua" {
                let stem = path.file_stem()?.to_str()?.to_owned();
                let yaml = source_dir.join(format!("{}.yaml", stem));
                if yaml.exists() {
                    Some(stem)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    if pairs.is_empty() {
        LogMessage::warning("No matching .lua/.yaml pairs found in the provided directory");
        return Ok(());
    }

    println!("Found {} extension pair(s):", pairs.len());
    for name in &pairs {
        println!("  • {}.lua + {}.yaml", name, name);
    }

    let confirmed = dialoguer::Confirm::new()
        .with_prompt("Add these extensions to the extensions directory?")
        .default(true)
        .interact()
        .map_err(|e| FileSystemError::OperationError(e.to_string()))?;

    if !confirmed {
        LogMessage::info("Aborted");
        return Ok(());
    }

    let dest_dir = PathBuf::from(EXTENSIONS_DIR.as_str());
    fs::create_dir_all(&dest_dir)?;

    for name in &pairs {
        for ext in ["lua", "yaml"] {
            let src = source_dir.join(format!("{}.{}", name, ext));
            let dst = dest_dir.join(format!("{}.{}", name, ext));
            fs::copy(&src, &dst)?;
        }
        LogMessage::success(&format!("Imported extension '{}'", name));
    }

    Ok(())
}

pub fn open_extensions_dir() -> Result<(), FileSystemError> {
    let dir = PathBuf::from(EXTENSIONS_DIR.as_str());
    fs::create_dir_all(&dir)?;

    let editors = ["Zed", "VS Code"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your preferred editor")
        .items(&editors)
        .default(0)
        .interact()
        .map_err(|e| FileSystemError::OperationError(e.to_string()))?;

    let editor_bin = match selection {
        0 => "zed",
        _ => "code",
    };

    Command::new(editor_bin).arg(&dir).spawn().map_err(|e| {
        FileSystemError::OperationError(format!("Failed to open editor '{}': {}", editor_bin, e))
    })?;

    Ok(())
}

pub fn clean_extensions() -> Result<(), FileSystemError> {
    let dir = PathBuf::from(EXTENSIONS_DIR.as_str());

    if !dir.exists() {
        LogMessage::info("Extensions directory is already empty");
        return Ok(());
    }

    let entries: Vec<_> = fs::read_dir(&dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|x| x.to_str())
                .map(|x| x == "lua" || x == "yaml")
                .unwrap_or(false)
        })
        .collect();

    if entries.is_empty() {
        LogMessage::info("No extensions to remove");
        return Ok(());
    }

    println!("The following files will be removed:");
    for entry in &entries {
        println!("  • {}", entry.file_name().to_string_lossy());
    }

    let confirmed = dialoguer::Confirm::new()
        .with_prompt("Remove all extensions? This cannot be undone")
        .default(false)
        .interact()
        .map_err(|e| FileSystemError::OperationError(e.to_string()))?;

    if !confirmed {
        LogMessage::info("Aborted");
        return Ok(());
    }

    for entry in &entries {
        fs::remove_file(entry.path())?;
    }

    LogMessage::success(&format!("Removed {} extension file(s)", entries.len()));

    Ok(())
}

pub fn remove_extension(name: &str) -> Result<(), FileSystemError> {
    let dir = PathBuf::from(EXTENSIONS_DIR.as_str());
    let lua_path = dir.join(format!("{}.lua", name));
    let yaml_path = dir.join(format!("{}.yaml", name));

    if !lua_path.exists() {
        LogMessage::error(&format!("Extension '{}' not found", name));
        return Err(FileSystemError::IoError(Error::new(
            ErrorKind::NotFound,
            "extension not found",
        )));
    }

    fs::remove_file(&lua_path)?;

    if yaml_path.exists() {
        fs::remove_file(&yaml_path)?;
    }

    LogMessage::success(&format!("Removed extension '{}'", name));

    Ok(())
}
