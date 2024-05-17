use std::env;

use migration::Migrator;
use migration::MigratorTrait;
use sea_orm::{Database, DbConn, DbErr};

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::connect(database_url).await?;
    
    // TODO: refactor only migrate database once
    Migrator::up(&db, None).await?;

    Ok(db)
}
