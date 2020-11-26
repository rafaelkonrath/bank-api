mod auth;
mod user;

use crate::errors::AppError;
use actix_web::{web, HttpResponse};
use auth::auth;
use user::{create_user, me, callback_code, 
    transactions, weekly_transactions, total_week_transactions, daily_transactions, 
    monthly_transactions, total_month_transactions, credit, debit};

type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;

pub fn app_config(config: &mut web::ServiceConfig) {
    let signup = web::resource("/signup").route(web::post().to(create_user));

    let auth = web::resource("/auth").route(web::post().to(auth));

    let me = web::resource("/me")
        .route(web::get().to(me));

    let health_resource = web::resource("/").route(web::get().to(health));

    let callback_code = web::resource("/callback").route(web::get().to(callback_code));

    let transactions = web::resource("/v1/transactions").route(web::get().to(transactions));
    let daily_transactions = web::resource("/v1/transactions/daily").route(web::get().to(daily_transactions));
    let weekly_transactions = web::resource("/v1/transactions/weekly").route(web::get().to(weekly_transactions));
    let total_week_transactions = web::resource("/v1/transactions/weekly/total").route(web::get().to(total_week_transactions));
    let monthly_transactions = web::resource("/v1/transactions/monthly").route(web::get().to(monthly_transactions));
    let total_month_transactions = web::resource("/v1/transactions/monthly/total").route(web::get().to(total_month_transactions));
    let credit = web::resource("/v1/transactions/credit").route(web::get().to(credit));
    let debit = web::resource("/v1/transactions/debit").route(web::get().to(debit));
    
    config
        .service(signup)
        .service(auth)
        .service(me)
        .service(health_resource)
        .service(callback_code)
        .service(transactions)
        .service(daily_transactions)
        .service(weekly_transactions)
        .service(monthly_transactions)
        .service(credit)
        .service(debit)
        .service(total_week_transactions)
        .service(total_month_transactions);
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}


