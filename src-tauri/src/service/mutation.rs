use chrono::SecondsFormat;
use entity::todos;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DbConn, EntityTrait, Set};

use crate::error::AppError;

pub struct Mutation;

impl Mutation {
    pub async fn create_todo_item(db: &DbConn,title: String) -> Result<todos::ActiveModel, AppError> {
        let result = todos::ActiveModel {
            id: NotSet,
            title: Set(title),
            datetime: Set(chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
            ..Default::default()
        }
        .save(db)
        .await?;
        Ok(result)
    }

    pub async fn delete_item_by_id(db: &DbConn, id: i32) -> Result<bool, AppError> {
        todos::Entity::delete_by_id(id).exec(db).await?;
        Ok(true)
    }

    pub async fn switch_item_status(db: &DbConn, id: i32, is_done: bool) -> Result<bool, AppError> {
        let item = todos::Entity::find_by_id(id).one(db).await?;
        match item {
            Some(item) => {
                let mut item: todos::ActiveModel = item.into();
                item.done = Set(Some(is_done));
                item.save(db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
