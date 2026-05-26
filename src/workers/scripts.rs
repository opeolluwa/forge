use std::{
    fs::{self},
    io::{Error, ErrorKind},
    path::Path,
    process::{Command, Stdio},
};

use dialoguer::Input;

use crate::{
    config::app::ToolboxConfig,
    constants::{APP_RUNTIME_DATABASE_PATH, APP_RUNTIME_SCRIPTS_DIR, SCRIPTS_DIR},
    errors::file_system::FileSystemError,
    helpers::console::LogMessage,
};

/// create a folder .dev_toolbox in the default home directory
pub fn configure_scripts(_overwrite: bool) -> Result<(), FileSystemError> {
    let home_dir = dirs::home_dir().ok_or(FileSystemError::IoError(Error::new(
        ErrorKind::NotFound,
        "could not find device $HOME directory",
    )))?;

    let scripts_dir = format!(
        "{}/{}",
        home_dir.to_str().unwrap_or_default(),
        APP_RUNTIME_SCRIPTS_DIR
    );

    let database_dir = format!(
        "{}/{}",
        home_dir.to_str().unwrap_or_default(),
        APP_RUNTIME_DATABASE_PATH
    );

    fs::create_dir_all(&scripts_dir)?;
    fs::create_dir_all(&database_dir)?;

    let database_url = format!(
        "sqlite://{}/{}",
        home_dir.to_str().unwrap_or_default(),
        APP_RUNTIME_DATABASE_PATH
    );

    let script_runner: String = Input::new()
        .with_prompt("Your script runner?")
        .default("python3".to_string())
        .interact_text()?;

    let scripts_path: String = Input::new()
        .with_prompt("Where do you want to store your scripts")
        .default(scripts_dir)
        .interact_text()?;

    let mut cfg = ToolboxConfig::new();
    cfg.env.database_url = database_url;
    cfg.scripts.runner = script_runner;
    cfg.scripts.source = scripts_path;

    cfg.save()?;

    let binding = cfg.file_path()?;
    let config_path = binding.as_path().to_str();

    LogMessage::success(&format!(" configuration saved to {:#?}", config_path));

    Ok(())
}

pub fn execute_custom_script(script: &str) -> Result<(), FileSystemError> {
    let home_dir = dirs::home_dir().ok_or(FileSystemError::IoError(Error::new(
        ErrorKind::NotFound,
        "could not find device $HOME directory",
    )))?;

    let script_path = home_dir.join(APP_RUNTIME_SCRIPTS_DIR).join(script);

    // Load toolbox configuration to determine which runner to use
    let cfg = ToolboxConfig::load()?;
    let runner = cfg.scripts.runner;

    // Execute the script using the configured runner and propagate errors
    let mut cmd = Command::new(runner)
        .arg(script_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let status = cmd.wait()?;

    if !status.success() {
        eprintln!("Script exited with status: {}", status);
    }
    Ok(())
}

pub fn add_script_command(script_file_path: &Path) -> Result<(), FileSystemError> {
    let file_path = script_file_path;

    if !file_path.exists() {
        LogMessage::error("Source file does not exist");
        return Err(FileSystemError::IoError(Error::new(
            ErrorKind::NotFound,
            "file does. not exist",
        )));
    }

    if !file_path.is_file() {
        LogMessage::error("Provided path is not a file");
        return Err(FileSystemError::IoError(Error::new(
            ErrorKind::InvalidInput,
            "unsupported file format",
        )));
    }

    let file_name = match file_path.file_name() {
        Some(name) => name,
        None => {
            LogMessage::error("Invalid file path: no filename");
            return Err(FileSystemError::IoError(Error::new(
                ErrorKind::InvalidInput,
                "unsupported file format",
            )));
        }
    };

    let home_dir = dirs::home_dir().ok_or(FileSystemError::IoError(Error::new(
        ErrorKind::NotFound,
        "could not find device $HOME directory",
    )))?;

    let scripts_dir = format!(
        "{}/{}",
        home_dir.to_str().unwrap_or_default(),
        APP_RUNTIME_SCRIPTS_DIR
    );

    let dest_dir = Path::new(&scripts_dir);

    // 🔑 Ensure destination directory exists
    if let Err(e) = fs::create_dir_all(dest_dir) {
        LogMessage::error(&format!(
            "Failed to create destination directory '{}': {}",
            dest_dir.display(),
            e
        ));
        return Err(FileSystemError::IoError(e));
    }

    let dest_path = dest_dir.join(file_name);

    let full_src_path = match file_path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            LogMessage::error(&format!("Failed to resolve source path: {}", e));
            return Err(FileSystemError::IoError(e));
        }
    };

    match fs::copy(&full_src_path, &dest_path) {
        Ok(bytes) => {
            LogMessage::info(&format!(
                "Successfully copied '{}' → '{}' ({} bytes)",
                full_src_path.display(),
                dest_path.display(),
                bytes
            ));
            Ok(())
        }
        Err(e) => {
            LogMessage::error(&format!(
                "Failed to copy '{}' → '{}': {}",
                full_src_path.display(),
                dest_path.display(),
                e
            ));
            Err(FileSystemError::IoError(e))
        }
    }
}

pub fn list_script() {
    let paths = fs::read_dir(SCRIPTS_DIR.as_str()).unwrap();

    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        println!("file name: {}", name);
    }
}
