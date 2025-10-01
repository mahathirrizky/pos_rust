use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .add_column(ColumnDef::new(Employees::PhotoUrl).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .drop_column(Employees::PhotoUrl)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Employees {
    Table,
    PhotoUrl,
}
