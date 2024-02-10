mod m20240210_105733_user;
mod m20240210_110012_class;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240210_105733_user::Migration),
            Box::new(m20240210_110012_class::Migration),
        ]
    }
}
