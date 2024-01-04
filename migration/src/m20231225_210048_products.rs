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
                table_auto(Products::Table)
                    .col(pk_auto(Products::Id).borrow_mut())
                    .col(integer(Products::Cpu).borrow_mut())
                    .col(integer(Products::Memory).borrow_mut())
                    .col(integer(Products::StorageSize).borrow_mut())
                    .col(string(Products::StorageType).borrow_mut())
                    .col(string(Products::Name).borrow_mut())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Products::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Id,
    Cpu,
    Memory,
    StorageSize,
    StorageType,
    Name,
    
}


