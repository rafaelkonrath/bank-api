use super::{auth::AuthenticatedUser, AppResponse};
use crate::{
    config::crypto::CryptoService,
    config::params::Params,
    db,
    db::user::UserRepository,
    errors::AppError,
    models::user::{NewUser, User},
};
use actix_web::web;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use color_eyre::Result;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{error::DatabaseError, postgres::PgError};
use std::fmt::Debug;
use tracing::{debug, instrument};
use validator::Validate;

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
                }
                (Some(db::UNIQUE_VIOLATION_CODE), Some("username")) => {
                    AppError::INVALID_INPUT.message("Username already exists.".to_string())
                }
                (Some(db::UNIQUE_VIOLATION_CODE), None) => {
                    AppError::INVALID_INPUT.message("Username or email already exists.".to_string())
                }
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
pub async fn me(user: AuthenticatedUser, repository: UserRepository) -> AppResponse {
    let user = repository
        .find_by_id(user.0)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(user))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthRequest {
    code: String,
}

#[instrument(skip(repository))]
pub async fn callback_code(
    //user: AuthenticatedUser, #remove auth for the callback •͡˘㇁•͡˘
    repository: UserRepository,
    params: Data<Params>,
    web::Query(info): web::Query<AuthRequest>,
) -> AppResponse {
    repository.update_code(info.code.to_string()).await?;

    let access_token = exchange_token(params, info.code.to_string());

    repository.update_token(access_token.await).await?;
    Ok(HttpResponse::Ok().json(info))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    access_token: String,
}

#[instrument(skip(params))]
pub async fn exchange_token(params: Data<Params>, code: String) -> String {
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

#[instrument[skip(repository)]]
pub async fn transactions(
    user: AuthenticatedUser,
    repository: UserRepository,
    params: Data<Params>,
) -> AppResponse {
    let user_account: uuid::Uuid = user.0.clone();
    //Check if the transactions already exist in the database
    let user = repository
        .check_cache(user.0)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;
    // If not request a new transaction and save locally
    if user.results as i64 == 0 {
        let user_token = repository
            .get_token(user_account)
            .await?
            .ok_or(AppError::INTERNAL_ERROR)?;
        let _host = format!("{}/data/v1/accounts", params.api_uri);
        /*
            Todo: request all the accounts from the user and save locally
        */
        let host_temp = "https://api.truelayer-sandbox.com/data/v1/accounts/56c7b029e0f8ec5a2334fb0ffc2fface/transactions";
        let res = reqwest::Client::new()
            .get(host_temp)
            .bearer_auth(user_token.access_token.unwrap())
            .send()
            .await
            .expect("Getting Transactions");
        // Check for 200 status
        // and save resquest in the database
        if res.status().is_success() {
            let body = res.text().await.expect("Reading Body");
            let serialized: Value = serde_json::from_str(&body).unwrap();
            repository
                .save_trans(user_account, serialized.to_string())
                .await?;

            Ok(HttpResponse::Ok().json(serialized))
        } else {
            // If any error just forward
            let body = res.text().await.expect("Reading Body");
            let serialized: Value = serde_json::from_str(&body).unwrap();
            Ok(HttpResponse::Ok().json(serialized))
        }
    } else {
        // retrieve data from local database
        let user_cached = repository.get_cache(user_account).await?;
        let serialized: Value = serde_json::from_str(&user_cached.results as &str).unwrap();
        Ok(HttpResponse::Ok().json(serialized))
    }
}
