use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
use serde::Serialize;

use crate::mysql::migration::Migrator;

pub mod migration;
pub mod model;
pub mod repository;

#[derive(Clone, Debug, Serialize)]
pub struct Todo {
    pub id: u32,
    pub name: String,
    pub done: bool,
}

pub struct SqlAdapter {
    db: DatabaseConnection,
}

impl SqlAdapter {
    pub async fn new(sql_dsn: &str) -> Self {
        let db = Database::connect(sql_dsn).await.expect("must connected to database");
        Self { db }
    }

    pub fn repository(&self) -> repository::TodoRepository {
        repository::TodoRepository::new(self.db.clone())
    }

    pub async fn migrate(&self, step: Option<u32>) -> Result<(), DbErr> {
        Migrator::up(&self.db, step).await
    }
}
