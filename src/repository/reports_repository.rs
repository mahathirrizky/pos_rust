use sea_orm::{
    ConnectionTrait, DbErr, EntityTrait, QuerySelect, ColumnTrait, Condition, QueryFilter,
    RelationTrait,
};
use sea_orm::sea_query::{Expr};
use crate::entities::{orders, order_items, stores};
use chrono::{Utc, Duration, NaiveDateTime};

pub struct ReportRepository;

impl ReportRepository {
    pub async fn get_total_sales_by_period<C: ConnectionTrait>(
        db: &C,
        period: &str,
    ) -> Result<f64, DbErr> {
        let now = Utc::now().naive_utc();
        let start_of_period: NaiveDateTime = match period {
            "today" => (now - Duration::days(1)).date().and_hms_opt(0, 0, 0).unwrap(),
            "week" => (now - Duration::days(7)).date().and_hms_opt(0, 0, 0).unwrap(),
            "month" => (now - Duration::days(30)).date().and_hms_opt(0, 0, 0).unwrap(),
            "year" => (now - Duration::days(365)).date().and_hms_opt(0, 0, 0).unwrap(),
            _ => return Err(DbErr::Custom("Invalid period specified".to_string())),
        };

        let total_sales: Option<f64> = orders::Entity::find()
            .join(
                sea_orm::JoinType::InnerJoin,
                orders::Relation::OrderItem.def(),
            )
            .filter(
                Condition::all()
                    .add(orders::Column::OrderDate.gte(start_of_period))
                    .add(orders::Column::Status.eq("completed")),
            )
            .select_only()
            .column_as(
                Expr::expr(
                    Expr::col(order_items::Column::UnitPrice)
                        .mul(Expr::col(order_items::Column::Quantity))
                ).sum(),
                "total_sales"
            )
            .into_tuple()
            .one(db)
            .await?
            .and_then(|(total,)| total);

        Ok(total_sales.unwrap_or(0.0))
    }

    pub async fn get_total_revenue<C: ConnectionTrait>(
        db: &C,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
        store_id: Option<i32>,
    ) -> Result<f64, DbErr> {
        let mut query = orders::Entity::find()
            .join(
                sea_orm::JoinType::InnerJoin,
                orders::Relation::OrderItem.def(),
            )
            .filter(
                Condition::all()
                    .add(orders::Column::OrderDate.between(start_date, end_date))
                    .add(orders::Column::Status.eq("completed")),
            )
            .select_only()
            .column_as(
                Expr::expr(
                    Expr::col(order_items::Column::UnitPrice)
                        .mul(Expr::col(order_items::Column::Quantity)),
                )
                .sum(),
                "total_revenue",
            );

        if let Some(id) = store_id {
            query = query.filter(orders::Column::StoreId.eq(id));
        }

        let total_revenue: Option<f64> = query
            .into_tuple()
            .one(db)
            .await?
            .and_then(|(total,)| total);

        Ok(total_revenue.unwrap_or(0.0))
    }

    pub async fn get_total_transactions<C: ConnectionTrait>(
        db: &C,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
        store_id: Option<i32>,
    ) -> Result<i64, DbErr> {
        let mut query = orders::Entity::find()
            .filter(
                Condition::all()
                    .add(orders::Column::OrderDate.between(start_date, end_date))
                    .add(orders::Column::Status.eq("completed")),
            )
            .select_only()
            .column_as(orders::Column::Id.count(), "total_transactions");

        if let Some(id) = store_id {
            query = query.filter(orders::Column::StoreId.eq(id));
        }

        let total_transactions: Option<i64> = query
            .into_tuple()
            .one(db)
            .await?
            .and_then(|(total,)| total);

        Ok(total_transactions.unwrap_or(0))
    }

    pub async fn get_sales_over_time<C: ConnectionTrait>(
        db: &C,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
        store_id: Option<i32>,
    ) -> Result<Vec<(String, f64)>, DbErr> {
        let mut query = orders::Entity::find()
            .join(
                sea_orm::JoinType::InnerJoin,
                orders::Relation::OrderItem.def(),
            )
            .filter(
                Condition::all()
                    .add(orders::Column::OrderDate.between(start_date, end_date))
                    .add(orders::Column::Status.eq("completed")),
            )
            .select_only()
            .column_as(
                Expr::cust_with_exprs("DATE(?)", [Expr::col(orders::Column::OrderDate).into()]),
                "date"
            )
            .column_as(
                Expr::expr(
                    Expr::col(order_items::Column::UnitPrice)
                        .mul(Expr::col(order_items::Column::Quantity)),
                )
                .sum(),
                "daily_sales",
            )
            .group_by(Expr::cust_with_exprs(
                "DATE(?)",
                [Expr::col(orders::Column::OrderDate).into()],
            ));

        if let Some(id) = store_id {
            query = query.filter(orders::Column::StoreId.eq(id));
        }

        let sales_data: Vec<(String, f64)> = query
            .into_tuple()
            .all(db)
            .await?;

        Ok(sales_data)
    }

    pub async fn get_sales_by_store<C: ConnectionTrait>(
        db: &C,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
        store_id: Option<i32>,
    ) -> Result<Vec<(String, f64)>, DbErr> {
        let mut query = orders::Entity::find()
            .join(
                sea_orm::JoinType::InnerJoin,
                orders::Relation::OrderItem.def(),
            )
            .join(
                sea_orm::JoinType::InnerJoin,
                orders::Relation::Store.def(),
            )
            .filter(
                Condition::all()
                    .add(orders::Column::OrderDate.between(start_date, end_date))
                    .add(orders::Column::Status.eq("completed")),
            )
            .select_only()
            .column_as(stores::Column::Name, "store_name")
            .column_as(
                Expr::expr(
                    Expr::col(order_items::Column::UnitPrice)
                        .mul(Expr::col(order_items::Column::Quantity)),
                )
                .sum(),
                "total_sales",
            )
            .group_by(stores::Column::Name);

        if let Some(id) = store_id {
            query = query.filter(orders::Column::StoreId.eq(id));
        }

        let sales_data: Vec<(String, f64)> = query
            .into_tuple()
            .all(db)
            .await?;

        Ok(sales_data)
    }
}
