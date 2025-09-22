use sea_orm::{DbErr, Set, ActiveModelTrait, ConnectionTrait};

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
