use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};

use crate::mysql::{Todo, model};

#[derive(Clone)]
pub struct TodoRepository {
    db: DatabaseConnection,
}

impl TodoRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, name: &str) -> Result<Todo, DbErr> {
        let todo = model::todo::ActiveModel {
            name: Set(name.to_string()),
            ..Default::default()
        };
        todo.insert(&self.db).await.map(Into::into)
    }

    pub async fn find_id(&self, id: u32) -> Result<Option<Todo>, DbErr> {
        let res = model::todo::Entity::find_by_id(id).one(&self.db).await?;
        Ok(res.map(Into::into))
    }
}
