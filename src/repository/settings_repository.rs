use sea_orm::{DbConn, DbErr, EntityTrait, ActiveModelTrait, IntoActiveModel};
use std::result::Result;

use crate::entities::settings;
use crate::entities::settings_model::Settings as SettingsModel;


pub async fn get_settings(db: &DbConn) -> Result<SettingsModel, DbErr> {
    let setting_entity = settings::Entity::find()
        .one(db)
        .await?;

    if let Some(entity) = setting_entity {
        Ok(SettingsModel::from(entity.config))
    } else {
        // If no settings found, return default and maybe log a warning
        println!("Warning: No settings found in the database, returning default settings.");
        Ok(SettingsModel::default())
    }
}

pub async fn update_settings(db: &DbConn, settings_data: SettingsModel) -> Result<SettingsModel, DbErr> {
    let settings_json = serde_json::to_value(&settings_data).map_err(|e| DbErr::Custom(e.to_string()))?;

    let current_settings = settings::Entity::find().one(db).await?;

    if let Some(current) = current_settings {
        let mut active_model = current.into_active_model();
        active_model.config = sea_orm::Set(settings_json);
        let updated = active_model.update(db).await?;
        Ok(SettingsModel::from(updated.config))
    } else {
        // If no settings exist, create a new record
        let active_model = settings::ActiveModel {
            config: sea_orm::Set(settings_json),
            ..Default::default()
        };
        let new_settings = active_model.insert(db).await?;
        Ok(SettingsModel::from(new_settings.config))
    }
}
