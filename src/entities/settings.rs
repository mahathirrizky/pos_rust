use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::entities::settings_model::Settings as SettingsModel;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "settings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub config: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Json> for SettingsModel {
    fn from(json: Json) -> Self {
        serde_json::from_value(json).unwrap_or_default()
    }
}
