use include_dir::{include_dir, Dir};
use once_cell::sync::Lazy;

pub const TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");
pub const EXTENSIONS_SRC: Dir = include_dir!("$CARGO_MANIFEST_DIR/extensions");

pub const APP_RUNTIME_CONFIG_DIR: &str = ".forge";
pub const APP_RUNTIME_SCRIPTS_DIR: &str = ".forge/scripts";
pub const APP_RUNTIME_EXTENSIONS_DIR: &str = ".forge/extensions";
pub const APP_RUNTIME_CONFIG_FILE_NAME: &str = "Forge.toml";
pub const APP_RUNTIME_DATABASE_PATH: &str = ".forge/db/app.sqlite";

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    let home = dirs::home_dir().expect("Could not determine home directory");

    let db_dir = home.join(".forge");
    std::fs::create_dir_all(&db_dir).expect("Failed to create database directory");

    let db_file = db_dir.join("forge.db");

    let url = url::Url::from_file_path(&db_file).expect("Failed to convert DB path to file URL");

    // SQLite expects the `sqlite:` scheme
    format!("sqlite:///{}?mode=rwc", url.path())
});

pub static SCRIPTS_DIR: Lazy<String> = Lazy::new(|| {
    let home = dirs::home_dir().expect("Could not determine home directory");

    home.join(APP_RUNTIME_SCRIPTS_DIR)
        .to_str()
        .unwrap()
        .to_string()
});

pub static EXTENSIONS_DIR: Lazy<String> = Lazy::new(|| {
    let home = dirs::home_dir().expect("Could not determine home directory");

    home.join(APP_RUNTIME_EXTENSIONS_DIR)
        .to_str()
        .unwrap()
        .to_string()
});
