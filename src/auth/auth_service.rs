use argon2::{self, Argon2, PasswordHash, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use lazy_static::lazy_static;
use password_hash::{PasswordHasher, SaltString};
use rand::rngs::OsRng;
use sea_orm::{DbErr, DatabaseConnection};
use serde::{Deserialize, Serialize};
use actix_web::{FromRequest, HttpRequest, dev::Payload, http, error::ErrorUnauthorized};
use std::future::{Future};
use std::pin::Pin;

use crate::entities::employees;
use crate::repository::permissions_repository::PermissionsRepository;

lazy_static! {
    static ref JWT_SECRET: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32, // Employee ID
    pub email: String,
    pub role: String,
    pub store_id: Option<i32>,
    pub permissions: Vec<String>, // User's permissions
    pub exp: usize, // Expiration time
}

pub fn hash_password(password: &str) -> Result<String, DbErr> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|e| DbErr::Custom(e.to_string()))?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, DbErr> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| DbErr::Custom(e.to_string()))?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

pub async fn create_jwt(db: &DatabaseConnection, employee: &employees::Model) -> Result<String, DbErr> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24)) // Token valid for 24 hours
        .expect("valid timestamp")
        .timestamp() as usize;

    let permissions = PermissionsRepository::find_permissions_for_role(db, employee.role_id).await?;

    let claims = Claims {
        sub: employee.id,
        email: employee.email.clone(),
        role: employee.role.clone(),
        store_id: employee.store_id,
        permissions,
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
        .map_err(|e| DbErr::Custom(e.to_string()))
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, DbErr> {
    decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET.as_bytes()), &Validation::default())
        .map_err(|e| DbErr::Custom(e.to_string()))
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            if let Some(auth_header) = req.headers().get(http::header::AUTHORIZATION) {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str["Bearer ".len()..];
                        return match decode_jwt(token) {
                            Ok(token_data) => Ok(token_data.claims),
                            Err(_) => Err(ErrorUnauthorized("Invalid token")),
                        };
                    }
                }
            }
            Err(ErrorUnauthorized("Missing or invalid token"))
        })
    }
}
