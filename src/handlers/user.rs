use super::{auth::AuthenticatedUser, AppResponse};
use crate::{
    db, 
    config::crypto::CryptoService,
    db::user::UserRepository,
    errors::AppError,
    models::user::{NewUser, User},
    config::params::{Params},
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use actix_web::{web};
use color_eyre::Result;
use sqlx::{error::DatabaseError, postgres::PgError};
use tracing::{debug, instrument};
use validator::Validate;
use serde::{Deserialize, Serialize};
use reqwest;
use std::fmt::Debug;

#[instrument(skip(user, repository, crypto_service))]
pub async fn create_user(
    user: Json<NewUser>,
    repository: UserRepository,
    crypto_service: Data<CryptoService>,
) -> AppResponse {
    match user.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_map = errors.field_errors();

            let message = if error_map.contains_key("username") {
                format!("Invalid username. \"{}\" is too short.", user.username)
            } else if error_map.contains_key("email") {
                format!("Invalid email address \"{}\"", user.email)
            } else if error_map.contains_key("password") {
                "Invalid password. Too short".to_string()
            } else {
                "Invalid input.".to_string()
            };

            Err(AppError::INVALID_INPUT.message(message))
        }
    }?;

    let result: Result<User> = repository.create(user.0, crypto_service.as_ref()).await;

    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(error) => {
            let pg_error: &PgError =
                error
                    .root_cause()
                    .downcast_ref::<PgError>()
                    .ok_or_else(|| {
                        debug!("Error creating user. {:?}", error);
                        AppError::INTERNAL_ERROR
                    })?;

            let error = match (pg_error.code(), pg_error.column_name()) {
                (Some(db::UNIQUE_VIOLATION_CODE), Some("email")) => {
                    AppError::INVALID_INPUT.message("Email address already exists.".to_string())
                },
                (Some(db::UNIQUE_VIOLATION_CODE), Some("username")) => {
                    AppError::INVALID_INPUT.message("Username already exists.".to_string())
                },
                (Some(db::UNIQUE_VIOLATION_CODE), None) => {
                    AppError::INVALID_INPUT.message("Username or email already exists.".to_string())
                },
                _ => {
                    debug!("Error creating user. {:?}", pg_error);
                    AppError::INTERNAL_ERROR.into()
                }
            };
            Err(error)
        }
    }
}


#[instrument[skip(repository)]]
pub async fn me(
    user: AuthenticatedUser, 
    repository: UserRepository
) -> AppResponse {
    let user = repository
        .find_by_id(user.0)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(user))
}

#[derive(Debug,Deserialize, Serialize)]
pub struct AuthRequest {
    code: String,
}

#[instrument(skip(repository))]
pub async fn callback_code(
    //user: AuthenticatedUser,
    repository: UserRepository,
    params: Data<Params>,
    web::Query(info): web::Query<AuthRequest>,
) -> AppResponse {

    repository.update_code(info.code.to_string()).await?;

    let access_token = exchange_token(params, info.code.to_string());

    repository.update_token(access_token.await).await?;
    
    Ok(HttpResponse::Ok().json(info))
}


#[derive(Debug,Deserialize, Serialize)]
pub struct Token {
    access_token: String,
}

#[instrument(skip(params))]
pub async fn exchange_token(
    params: Data<Params>, 
    code: String
) -> String {

    let res = reqwest::Client::new()
        .post(&params.token_uri.to_string())
        .form(&[
            ("grant_type", "authorization_code"),
            ("client_id", &params.client_id.to_string()),
            ("client_secret", &params.client_secret.to_string()),
            ("redirect_uri", &params.redirect_uri.to_string()),
            ("code", &code),
        ])
        .send()
        .await
        .expect("Getting token");

    let body = res.text().await.expect("Reading Body");
    let token: Token = serde_json::from_str(&body).unwrap();
    
    token.access_token.to_string()
}
