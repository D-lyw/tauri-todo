use migration::{Migrator, MigratorTrait};
use sea_orm::{DatabaseConnection, SqlxSqliteConnector};
use sqlx::SqlitePool;
use std::env;
use std::fs::{self, OpenOptions};
use std::sync::Arc;
use tauri::{
    api::path::{app_data_dir, app_log_dir},
    App, Config, Manager,
};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use console_subscriber;

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let handler = app.handle();
    tracing(&handler.config());

    tokio::spawn(async move {
        let db_conn = get_database_pool(&handler.config()).await;
        handler.manage(db_conn);
    });

    info!("Finished Setup");
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

pub fn tracing(config: &Config) {
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .pretty()
        .with_level(true)
        .with_target(true)
        .with_ansi(false);

    // log save to log file in production mode
    if cfg!(debug_assertions) {
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .with(console_subscriber::spawn())
            .init();
    } else {
        let file_path = app_log_dir(config)
            .expect("log directory not specified")
            .join("todo.log");
        // directory must exist
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create log directory");
        }

        // 使用 OpenOptions 以追加模式打开文件
        let file = OpenOptions::new()
            .create(true) // 如果文件不存在则创建
            .append(true) // 以追加模式打开
            .open(&file_path)
            .expect("Failed to open log file");

        let file_layer = fmt::layer().with_ansi(false).with_writer(Arc::new(file));

        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .with(file_layer)
            .with(console_subscriber::spawn())
            .init();
    };
}
