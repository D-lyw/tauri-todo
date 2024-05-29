use entity::todos;
use sea_orm::DbConn;
use tauri::State;
use tracing::{debug, info, instrument};

use crate::{
    error::AppError,
    service::{Mutation, Query},
};

#[instrument]
#[tauri::command]
pub async fn add_todo_item(db: State<'_, DbConn>, title: String) -> Result<bool, String> {
    info!("Command | add toto item: {}", title);
    debug!("test");
    let result = Mutation::create_todo_item(db.inner(), title.clone()).await;
    Ok(result.is_ok())
}

#[instrument]
#[tauri::command]
pub async fn query_list_by_page(
    db: State<'_, DbConn>,
    page: u64,
) -> Result<Vec<todos::Model>, String> {
    info!("Command | query todo list");

    match Query::query_todo_list(db.inner(), page).await {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}

#[instrument]
#[tauri::command]
pub async fn delete_item_by_id(db: State<'_, DbConn>, id: i32) -> Result<bool, AppError> {
    info!("Command | delete todo item {}", id);
    Mutation::delete_item_by_id(&db, id).await
}

#[instrument]
#[tauri::command]
pub async fn switch_item_status(
    db: State<'_, DbConn>,
    id: i32,
    is_done: bool,
) -> Result<bool, AppError> {
    info!("Command | switch todo item {} status {}", id, is_done);
    Mutation::switch_item_status(db.inner(), id, is_done).await
}
