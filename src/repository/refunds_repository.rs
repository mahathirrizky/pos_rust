use sea_orm::{DbErr, Set, ActiveModelTrait, ConnectionTrait, EntityTrait, QueryFilter, ColumnTrait};

use crate::entities::{refunds, refund_items};

pub async fn create_refund<C>(
    db: &C,
    refund: refunds::ActiveModel,
    items: Vec<refund_items::ActiveModel>,
) -> Result<(refunds::Model, Vec<refund_items::Model>), DbErr> where C: ConnectionTrait {
    let refund = refund.insert(db).await?;
    let mut created_items = vec![];

    for mut item in items {
        item.refund_id = Set(refund.id);
        created_items.push(item.insert(db).await?);
    }

    Ok((refund, created_items))
}

pub async fn get_all<C>(db: &C) -> Result<Vec<refunds::Model>, DbErr> where C: ConnectionTrait {
    refunds::Entity::find().all(db).await
}

pub async fn get_all_by_store<C>(db: &C, store_id: i32) -> Result<Vec<refunds::Model>, DbErr> where C: ConnectionTrait {
    refunds::Entity::find()
        .filter(refunds::Column::StoreId.eq(store_id))
        .all(db)
        .await
}

pub async fn find_by_id<C>(db: &C, id: i32) -> Result<Option<refunds::Model>, DbErr> where C: ConnectionTrait {
    refunds::Entity::find_by_id(id).one(db).await
}