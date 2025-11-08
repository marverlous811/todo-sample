use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};

use crate::mysql::{Todo, TodoUpdate, model, option_to_active_value};

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

    pub async fn update(&self, id: u32, update: TodoUpdate) -> Result<Todo, DbErr> {
        let todo = model::todo::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("not found record for update".to_string()))?;
        let todo = model::todo::ActiveModel {
            id: Set(todo.id),
            name: option_to_active_value(update.name),
            done: option_to_active_value(update.done),
            ..todo.into()
        };
        todo.update(&self.db).await.map(Into::into)
    }

    pub async fn delete(&self, id: u32) -> Result<u64, DbErr> {
        let res = model::todo::Entity::delete_by_id(id).exec(&self.db).await;

        res.map(|res| res.rows_affected)
    }
}
