use sea_orm_migration::prelude::*;

use crate::entities::settings_model::Settings as SettingsData;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Settings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Settings::Config).json().not_null())
                    .to_owned(),
            )
            .await?;

        // Seed the table with default settings
        let _db = manager.get_connection();
        let default_settings = SettingsData::default();
        let settings_json = serde_json::to_value(default_settings).unwrap();

        let insert = Query::insert()
            .into_table(Settings::Table)
            .columns([Settings::Config])
            .values_panic(vec![settings_json.into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Settings {
    Table,
    Id,
    Config,
}
