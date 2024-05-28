use entity::todos;
use sea_orm::DbConn;
use tauri::State;

use crate::{
    error::AppError,
    service::{Mutation, Query},
};

#[tauri::command]
pub async fn add_todo_item(db: State<'_, DbConn>, title: String) -> Result<bool, String> {
    let result = Mutation::create_todo_item(db.inner(), title).await;
    Ok(result.is_ok())
}

#[tauri::command]
pub async fn query_list_by_page(
    db: State<'_, DbConn>,
    page: u64,
) -> Result<Vec<todos::Model>, String> {
    match Query::query_todo_list(db.inner(), page).await {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn delete_item_by_id(db: State<'_, DbConn>, id: i32) -> Result<bool, AppError> {
    Mutation::delete_item_by_id(&db, id).await
}

#[tauri::command]
pub async fn switch_item_status(
    db: State<'_, DbConn>,
    id: i32,
    is_done: bool,
) -> Result<bool, AppError> {
    Mutation::switch_item_status(db.inner(), id, is_done).await
}
