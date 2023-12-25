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
                table_auto(Sales::Table)
                    .col(pk_auto(Sales::Id).borrow_mut())
                    .col(string(Sales::Status).borrow_mut())
                    .col(integer(Sales::UserId).borrow_mut())
                    .col(integer(Sales::ProductId).borrow_mut())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sales-users")
                            .from(Sales::Table, Sales::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sales-products")
                            .from(Sales::Table, Sales::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sales::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Sales {
    Table,
    Id,
    Status,
    UserId,
    ProductId,
    
}


#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Products {
    Table,
    Id,
}
