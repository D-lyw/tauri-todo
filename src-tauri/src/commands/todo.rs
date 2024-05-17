use entity::todos;
use sea_orm::DbErr;

use crate::service::{Mutation, Query};

#[tauri::command]
pub async fn add_todo_item(title: String) -> Result<bool, String> {
    let result = Mutation::create_todo_item(title).await;
    Ok(result.is_ok())
}

#[tauri::command]
pub async fn query_list_by_page(page: u64) -> Result<Vec<todos::Model>, String> {
    match Query::query_todo_list(page).await {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn delete_item_by_id(id: i32) -> Result<bool, String> {
    let result = Mutation::delete_item_by_id(id).await;
    Ok(result.is_ok())
}

#[tauri::command]
pub async fn switch_item_status(id: i32, is_done: bool) -> Result<bool, String> {
    match Mutation::switch_item_status(id, is_done).await {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}