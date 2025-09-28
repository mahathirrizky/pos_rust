use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::roles::{CreateRole, UpdateRole};
use crate::entities::permissions::{CreatePermission, UpdatePermission};
use crate::entities::role_permissions::{AssignPermissionToRole, RemovePermissionFromRole};
use crate::repository::roles_repository::RoleRepository;
use crate::repository::permissions_repository::PermissionsRepository;
use crate::extractor::claims_extractor::ClaimsExtractor;

pub async fn get_all_roles(db: web::Data<DatabaseConnection>, claims: ClaimsExtractor) -> impl Responder {
    if !claims.0.permissions.contains(&"manage_roles".to_string()) {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }
    match RoleRepository::get_all(db.get_ref()).await {
        Ok(roles) => HttpResponse::Ok().json(ApiResponse::new(roles)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch roles".to_string())),
    }
}

pub async fn get_role_by_id(db: web::Data<DatabaseConnection>, path: web::Path<i32>, claims: ClaimsExtractor) -> impl Responder {
    if !claims.0.permissions.contains(&"manage_roles".to_string()) {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }
    let id = path.into_inner();
    match RoleRepository::find_by_id(db.get_ref(), id).await {
        Ok(Some(role)) => HttpResponse::Ok().json(ApiResponse::new(role)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new(format!("Role with ID {} not found", id))),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch role".to_string())),
    }
}

pub async fn create_role(db: web::Data<DatabaseConnection>, new_role: web::Json<CreateRole>, claims: ClaimsExtractor) -> impl Responder {
    if !claims.0.permissions.contains(&"manage_roles".to_string()) {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }
    match RoleRepository::create(db.get_ref(), new_role.into_inner()).await {
        Ok(role) => HttpResponse::Ok().json(ApiResponse::new(role)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create role".to_string())),
    }
}

pub async fn update_role(db: web::Data<DatabaseConnection>, path: web::Path<i32>, update_data: web::Json<UpdateRole>, claims: ClaimsExtractor) -> impl Responder {
    if !claims.0.permissions.contains(&"manage_roles".to_string()) {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }
    let id = path.into_inner();
    match RoleRepository::update(db.get_ref(), id, update_data.into_inner()).await {
        Ok(Some(role)) => HttpResponse::Ok().json(ApiResponse::new(role)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new(format!("Role with ID {} not found", id))),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update role".to_string())),
    }
}

pub async fn delete_role(db: web::Data<DatabaseConnection>, path: web::Path<i32>, claims: ClaimsExtractor) -> impl Responder {
    if !claims.0.permissions.contains(&"manage_roles".to_string()) {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }
    let id = path.into_inner();
    match RoleRepository::delete(db.get_ref(), id).await {
        Ok(rows_affected) if rows_affected > 0 => HttpResponse::Ok().json(ApiResponse::new("Role deleted successfully".to_string())),
        Ok(_) => HttpResponse::NotFound().json(ApiError::new(format!("Role with ID {} not found", id))),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete role".to_string())),
    }
}

pub async fn get_role_permissions(db: web::Data<DatabaseConnection>, path: web::Path<i32>, claims: ClaimsExtractor) -> impl Responder {
    if !claims.0.permissions.contains(&"manage_permissions".to_string()) {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }
    let role_id = path.into_inner();
    match PermissionsRepository::find_permissions_for_role(db.get_ref(), role_id).await {
        Ok(permissions) => HttpResponse::Ok().json(ApiResponse::new(permissions)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch role permissions".to_string())),
    }
}

pub async fn assign_permission_to_role(db: web::Data<DatabaseConnection>, payload: web::Json<AssignPermissionToRole>, claims: ClaimsExtractor) -> impl Responder {
    if !claims.0.permissions.contains(&"manage_permissions".to_string()) {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }
    match RoleRepository::assign_permission(db.get_ref(), payload.role_id, payload.permission_id).await {
        Ok(role_permission) => HttpResponse::Ok().json(ApiResponse::new(role_permission)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to assign permission".to_string())),
    }
}

pub async fn remove_permission_from_role(db: web::Data<DatabaseConnection>, payload: web::Json<RemovePermissionFromRole>, claims: ClaimsExtractor) -> impl Responder {
    if !claims.0.permissions.contains(&"manage_permissions".to_string()) {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }
    match RoleRepository::remove_permission(db.get_ref(), payload.role_id, payload.permission_id).await {
        Ok(rows_affected) if rows_affected > 0 => HttpResponse::Ok().json(ApiResponse::new("Permission removed successfully".to_string())),
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Permission not found for role".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to remove permission".to_string())),
    }
}

pub async fn get_all_permissions(db: web::Data<DatabaseConnection>, claims: ClaimsExtractor) -> impl Responder {
    if !claims.0.permissions.contains(&"manage_permissions".to_string()) {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }
    match PermissionsRepository::get_all(db.get_ref()).await {
        Ok(permissions) => HttpResponse::Ok().json(ApiResponse::new(permissions)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch permissions".to_string())),
    }
}

pub async fn create_permission(db: web::Data<DatabaseConnection>, new_permission: web::Json<CreatePermission>, claims: ClaimsExtractor) -> impl Responder {
    if !claims.0.permissions.contains(&"manage_permissions".to_string()) {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }
    match PermissionsRepository::create(db.get_ref(), new_permission.into_inner()).await {
        Ok(permission) => HttpResponse::Ok().json(ApiResponse::new(permission)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create permission".to_string())),
    }
}

pub async fn update_permission(db: web::Data<DatabaseConnection>, path: web::Path<i32>, update_data: web::Json<UpdatePermission>, claims: ClaimsExtractor) -> impl Responder {
    if !claims.0.permissions.contains(&"manage_permissions".to_string()) {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }
    let id = path.into_inner();
    match PermissionsRepository::update(db.get_ref(), id, update_data.into_inner()).await {
        Ok(Some(permission)) => HttpResponse::Ok().json(ApiResponse::new(permission)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new(format!("Permission with ID {} not found", id))),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update permission".to_string())),
    }
}

pub async fn delete_permission(db: web::Data<DatabaseConnection>, path: web::Path<i32>, claims: ClaimsExtractor) -> impl Responder {
    if !claims.0.permissions.contains(&"manage_permissions".to_string()) {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }
    let id = path.into_inner();
    match PermissionsRepository::delete(db.get_ref(), id).await {
        Ok(rows_affected) if rows_affected > 0 => HttpResponse::Ok().json(ApiResponse::new("Permission deleted successfully".to_string())),
        Ok(_) => HttpResponse::NotFound().json(ApiError::new(format!("Permission with ID {} not found", id))),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete permission".to_string())),
    }
}
