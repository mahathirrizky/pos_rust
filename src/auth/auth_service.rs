use argon2::{Argon2, PasswordHasher, PasswordVerifier, PasswordHash};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use sea_orm::DbErr;
use crate::entities::employees;
use password_hash::{SaltString, rand_core::OsRng};

const SECRET_KEY: &[u8] = b"your_secret_key_here"; // TODO: Load from environment variable

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32, // Employee ID
    pub email: String,
    pub role: String,
    pub store_id: i32,
    pub exp: usize, // Expiration time
}

pub fn hash_password(password: &str) -> Result<String, DbErr> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(OsRng);
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).map_err(|e| DbErr::Custom(e.to_string()))?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, DbErr> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| DbErr::Custom(e.to_string()))?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).map_err(|e| DbErr::Custom(e.to_string()))?;
    Ok(true)
}

pub fn create_jwt(employee: &employees::Model) -> Result<String, DbErr> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24)) // Token valid for 24 hours
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: employee.id,
        email: employee.email.clone(),
        role: employee.role.clone(),
        store_id: employee.store_id,
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
        .map_err(|e| DbErr::Custom(e.to_string()))
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, DbErr> {
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &Validation::default())
        .map_err(|e| DbErr::Custom(e.to_string()))
}
