use std::env;

use migration::{Migrator, MigratorTrait};
use sea_orm::{DatabaseConnection, SqlxSqliteConnector};
use sqlx::SqlitePool;
use tauri::{api::path::app_data_dir, App, Config, Manager};

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let handler = app.handle();
    tokio::spawn(async move {
        let db_conn = get_database_pool(&handler.config()).await;
        handler.manage(db_conn);
    });

    Ok(())
}

pub async fn get_database_pool(config: &Config) -> DatabaseConnection {
    let database_url = get_db_path(config);

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect database");

    let db_conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);

    Migrator::up(&db_conn, None)
        .await
        .expect("Failed to migrate");

    db_conn
}

pub fn get_db_path(config: &Config) -> String {
    if cfg!(debug_assertions) {
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    } else {
        let app_data_dir = app_data_dir(config).expect("DATABASE_URL must be");
        std::fs::create_dir_all(&app_data_dir).unwrap();

        format!(
            "sqlite://{}?mode=rwc",
            app_data_dir.join("db.sqlite").to_string_lossy()
        )
    }
}
