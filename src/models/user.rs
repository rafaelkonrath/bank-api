use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: Option<String>,
    #[serde(skip_serializing)]
    pub active: bool,
    pub code: Option<String>,
    //#[serde(skip_serializing)]
    pub access_token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(length(min = 3))]
    pub password: String,
    #[validate(email)]
    pub email: String,
    
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCode {
    pub code: String,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Validate)]
pub struct GetToken {
    pub access_token: String,
}

