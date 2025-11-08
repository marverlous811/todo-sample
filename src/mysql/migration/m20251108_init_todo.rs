use sea_orm::EntityName;
use sea_orm_migration::prelude::*;

use crate::mysql::model::todo::{Column, Entity};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity.table_ref())
                    .col(ColumnDef::new(Column::Id).unsigned().primary_key().auto_increment())
                    .col(ColumnDef::new(Column::Name).string().not_null())
                    .col(ColumnDef::new(Column::Done).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity.table_ref()).to_owned())
            .await?;
        Ok(())
    }
}
