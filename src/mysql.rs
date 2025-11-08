use std::time::Duration;

use sea_orm::{ActiveValue, ConnectOptions, Database, DatabaseConnection, DbErr};
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

#[derive(Debug, Clone, Serialize)]
pub struct TodoUpdate {
    pub name: Option<String>,
    pub done: Option<bool>,
}

pub struct SqlAdapter {
    db: DatabaseConnection,
}

impl SqlAdapter {
    pub async fn new(sql_dsn: &str) -> Self {
        let mut opt = ConnectOptions::new(sql_dsn);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(30))
            .acquire_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(60))
            .max_lifetime(Duration::from_secs(60));
        let db = Database::connect(opt).await.expect("must connected to database");
        Self { db }
    }

    pub fn repository(&self) -> repository::TodoRepository {
        repository::TodoRepository::new(self.db.clone())
    }

    pub async fn migrate(&self, step: Option<u32>) -> Result<(), DbErr> {
        Migrator::up(&self.db, step).await
    }
}

fn option_to_active_value<T>(value: Option<T>) -> ActiveValue<T>
where
    T: Into<sea_orm::Value>,
{
    match value {
        Some(value) => ActiveValue::Set(value),
        None => ActiveValue::NotSet,
    }
}
