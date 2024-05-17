use entity::todos;
use sea_orm::{entity::*, query::*, DbErr};

use crate::establish_connection;


pub struct Query ;

impl Query {
    pub async fn query_todo_list(page_num: u64 ) -> Result<Vec<todos::Model>, DbErr> {
        let db = establish_connection().await?;
        let todo_pages = todos::Entity::find().order_by_desc(todos::Column::Id).paginate(&db, 100);

        let list = todo_pages.fetch_page(page_num - 1).await?;
        Ok(list)
    }
}