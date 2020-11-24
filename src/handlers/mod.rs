mod auth;
mod user;
mod bank;

use crate::errors::AppError;
use actix_web::{web, HttpResponse};
use auth::auth;
use user::{create_user, me, callback_code};
use bank::{{transactions}};

type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;

pub fn app_config(config: &mut web::ServiceConfig) {
    let signup = web::resource("/signup").route(web::post().to(create_user));

    let auth = web::resource("/auth").route(web::post().to(auth));

    let me = web::resource("/me")
        .route(web::get().to(me));

    let health_resource = web::resource("/").route(web::get().to(health));

    let callback_code = web::resource("/callback").route(web::get().to(callback_code));

    let transactions = web::resource("/transactions").route(web::get().to(transactions));

    config
        .service(signup)
        .service(auth)
        .service(me)
        .service(health_resource)
        .service(callback_code)
        .service(transactions);
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

