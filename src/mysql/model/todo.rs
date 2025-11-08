use sea_orm::entity::prelude::*;

use crate::mysql::Todo;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub done: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Todo {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            done: model.done,
        }
    }
}
