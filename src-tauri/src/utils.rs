use std::env;
use std::fs::{create_dir_all, File};
use std::path::Path;

use migration::Migrator;
use migration::MigratorTrait;
use sea_orm::{Database, DbConn, DbErr};

use crate::APP;

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let database_url = format!("sqlite://{}?mode=rwc", get_db_path());

    let db = Database::connect(database_url).await?;

    // TODO: refactor only migrate database once
    Migrator::up(&db, None).await?;

    Ok(db)
}

pub fn init_db() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();
    if !db_dir.exists() {
        create_dir_all(db_dir).unwrap();
    }
    if !Path::new(&db_path).exists() {
        File::create(&db_path).unwrap();
    }
    // let db = Database::connect(database_url).await?;

    // Migrator::up(&db, None).await?;
}

pub fn get_db_path() -> String {
    // development environment path
    if cfg!(debug_assertions) {
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    } else {
        let data_dir = APP
            .get()
            .expect("fail")
            .path_resolver()
            .app_data_dir()
            .expect("fail");
        let db_path = data_dir.join("db.sqlite");
        println!("{}", db_path.to_string_lossy());

        db_path.to_str().unwrap().to_string()
    }
}
