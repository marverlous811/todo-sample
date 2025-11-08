use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct Migrator;

mod m20251108_init_todo;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20251108_init_todo::Migration)]
    }
}
