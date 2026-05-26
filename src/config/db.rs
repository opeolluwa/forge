use sea_orm::{ConnectOptions, Database as SeaOrmDatabase, DatabaseConnection};

use migration::{Migrator, MigratorTrait};

use crate::{
    config::app::ToolboxConfig,
    errors::{app::AppError, database::DatabaseError},
};

pub struct Database;

impl Database {
    pub async fn init() -> Result<DatabaseConnection, AppError> {
        let config = ToolboxConfig::load()?;

        let database_url = config.env.database_url.trim();

        // Fail fast with a clear, user‑friendly error if the URL is missing.
        if database_url.is_empty() {
            return Err(AppError::DatabaseError(DatabaseError::InvalidConfig(
                "No database_url configured. Please run the configuration step (e.g. `tb script cfg`) to set up your database connection.".to_string(),
            )));
        }

        // Optional: basic sanity check on the scheme so we don't try to connect with a nonsense URL.
        if !database_url.starts_with("sqlite://") {
            return Err(AppError::DatabaseError(DatabaseError::InvalidConfig(
                format!(
                    "Unsupported database_url '{database_url}'. Expected a URL starting with 'sqlite://'.",
                ),
            )));
        }

        let opt = ConnectOptions::new(database_url.to_owned());

        let conn = SeaOrmDatabase::connect(opt).await.map_err(|err| {
            AppError::DatabaseError(DatabaseError::ConnectionFailed(err.to_string()))
        })?;

        Migrator::up(&conn, None).await.unwrap(); //TODO: IMPROVE error handling

        Ok(conn)
    }
}
