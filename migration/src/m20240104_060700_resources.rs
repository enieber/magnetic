use std::borrow::BorrowMut;

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Resources::Table)
                    .col(pk_auto(Resources::Id).borrow_mut())
                    .col(integer(Resources::Memory).borrow_mut())
                    .col(integer(Resources::SaleId).borrow_mut())
                    .col(integer(Resources::Space).borrow_mut())
                    .col(integer(Resources::Core).borrow_mut())
                    .col(string(Resources::Hostname).borrow_mut())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-resources-sales")
                            .from(Resources::Table, Resources::SaleId)
                            .to(Sales::Table, Sales::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Resources::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Resources {
    Table,
    Id,
    Memory,
    SaleId,
    Space,
    Core,
    Hostname,
    
}


#[derive(DeriveIden)]
enum Sales {
    Table,
    Id,
}
