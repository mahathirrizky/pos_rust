use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PasswordResetTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PasswordResetTokens::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PasswordResetTokens::EmployeeId).integer().not_null())
                    .col(ColumnDef::new(PasswordResetTokens::Token).string().unique_key().not_null())
                    .col(ColumnDef::new(PasswordResetTokens::ExpiresAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_password_reset_token_employee_id")
                            .from(PasswordResetTokens::Table, PasswordResetTokens::EmployeeId)
                            .to(Employees::Table, Employees::Id)
                            .on_delete(ForeignKeyAction::Cascade), // If employee is deleted, cascade delete tokens
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PasswordResetTokens::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PasswordResetTokens {
    Table,
    Id,
    EmployeeId,
    Token,
    ExpiresAt,
}

// We need to reference the Employees table for the foreign key
#[derive(DeriveIden)]
enum Employees {
    Table,
    Id,
}
