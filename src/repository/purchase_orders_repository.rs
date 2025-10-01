use sea_orm::{DbErr, Set, EntityTrait, ActiveModelTrait, DatabaseConnection, QueryFilter, ColumnTrait, ModelTrait, TransactionTrait, ConnectionTrait};
use crate::entities::{purchase_orders, purchase_order_items, suppliers, stores, products};
use crate::repository::inventory_repository::InventoryRepository;
use crate::handler::purchase_orders_handler::ReceiveItem;
use chrono::{Utc, DateTime};

pub struct PurchaseOrderRepository;

impl PurchaseOrderRepository {
    pub async fn create_purchase_order<C: ConnectionTrait>(
        db: &C,
        supplier_id: i32,
        store_id: i32,
        employee_id: Option<i32>,
    ) -> Result<purchase_orders::Model, DbErr> {
        let now: DateTime<Utc> = Utc::now();
        let new_po = purchase_orders::ActiveModel {
            supplier_id: Set(supplier_id),
            store_id: Set(store_id),
            employee_id: Set(employee_id),
            order_date: Set(now.into()),
            status: Set("draft".to_owned()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let po = new_po.insert(db).await?;
        Ok(po)
    }

    pub async fn add_item_to_purchase_order<C: ConnectionTrait>(
        db: &C,
        purchase_order_id: i32,
        product_id: i32,
        quantity_ordered: i32,
        unit_price: sea_orm::prelude::Decimal,
    ) -> Result<purchase_order_items::Model, DbErr> {
        let now: DateTime<Utc> = Utc::now();
        let new_item = purchase_order_items::ActiveModel {
            purchase_order_id: Set(purchase_order_id),
            product_id: Set(product_id),
            quantity_ordered: Set(quantity_ordered),
            unit_price: Set(unit_price),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let item = new_item.insert(db).await?;
        Ok(item)
    }

    pub async fn find_po_with_relations<C: ConnectionTrait>(
        db: &C,
        id: i32,
    ) -> Result<Option<(purchase_orders::Model, Option<suppliers::Model>, Option<stores::Model>)>, DbErr> {
        let po = purchase_orders::Entity::find_by_id(id).one(db).await?;
        if let Some(po) = po {
            let supplier = po.find_related(suppliers::Entity).one(db).await?;
            let store = po.find_related(stores::Entity).one(db).await?;
            Ok(Some((po, supplier, store)))
        } else {
            Ok(None)
        }
    }

    pub async fn find_all_pos_with_relations<C: ConnectionTrait>(
        db: &C,
    ) -> Result<Vec<(purchase_orders::Model, Option<suppliers::Model>, Option<stores::Model>)>, DbErr> {
        let pos = purchase_orders::Entity::find().all(db).await?;
        let mut result = Vec::new();
        for po in pos {
            let supplier = po.find_related(suppliers::Entity).one(db).await?;
            let store = po.find_related(stores::Entity).one(db).await?;
            result.push((po, supplier, store));
        }
        Ok(result)
    }

    pub async fn find_items_for_po<C: ConnectionTrait>(
        db: &C,
        po_id: i32,
    ) -> Result<Vec<(purchase_order_items::Model, Option<products::Model>)>, DbErr> {
        purchase_order_items::Entity::find()
            .filter(purchase_order_items::Column::PurchaseOrderId.eq(po_id))
            .find_also_related(products::Entity)
            .all(db)
            .await
    }

    pub async fn update_po_status<C: ConnectionTrait>(
        db: &C,
        id: i32,
        new_status: String,
    ) -> Result<purchase_orders::Model, DbErr> {
        let po = purchase_orders::Entity::find_by_id(id).one(db).await?;
        if let Some(po) = po {
            let mut active_po: purchase_orders::ActiveModel = po.into();
            active_po.status = Set(new_status);
            active_po.updated_at = Set(Utc::now());
            active_po.update(db).await
        } else {
            Err(DbErr::Custom("Purchase Order not found".to_owned()))
        }
    }

    pub async fn receive_purchase_order_items(
        db: &DatabaseConnection,
        po_id: i32,
        items: Vec<ReceiveItem>,
    ) -> Result<(), DbErr> {
        let txn = db.begin().await?;

        let po = purchase_orders::Entity::find_by_id(po_id)
            .one(&txn)
            .await?
            .ok_or_else(|| DbErr::Custom("Purchase Order not found".to_owned()))?;

        for item in items {
            let po_item = purchase_order_items::Entity::find_by_id(item.purchase_order_item_id)
                .one(&txn)
                .await?
                .ok_or_else(|| DbErr::Custom("Purchase Order Item not found".to_owned()))?;

            // Ensure the item belongs to the correct PO
            if po_item.purchase_order_id != po_id {
                return Err(DbErr::Custom("Item does not belong to this Purchase Order".to_owned()));
            }

            // Update inventory
            InventoryRepository::increase_quantity(
                &txn,
                po_item.product_id,
                po.store_id,
                item.quantity_received,
            ).await?;

            // Update the received quantity on the PO item
            let mut po_item_active: purchase_order_items::ActiveModel = po_item.into();
            let new_quantity = po_item_active.quantity_received.as_ref() + item.quantity_received;
            po_item_active.quantity_received = Set(new_quantity);
            po_item_active.updated_at = Set(Utc::now());
            po_item_active.update(&txn).await?;
        }

        // Check if all items are fully received and update PO status
        let all_items = Self::find_items_for_po(&txn, po_id).await?;
        let all_received = all_items.iter().all(|(item, _)| item.quantity_received >= item.quantity_ordered);

        let new_status = if all_received {
            "completed".to_string()
        } else {
            "partially_received".to_string()
        };

        Self::update_po_status(&txn, po_id, new_status).await?;

        txn.commit().await
    }
}
