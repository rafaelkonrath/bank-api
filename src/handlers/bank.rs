use super::{auth::AuthenticatedUser, AppResponse};
use crate::{
//    db, 
//    config::crypto::CryptoService,
    db::user::UserRepository,
    db::trans::TransRepository,
    errors::AppError,
    //models::user::{User},
    //models::trans::{Transactions},
    config::params::{Params},
};
use actix_web::{
    web::{Data},
    HttpResponse,
};
use reqwest;
use tracing::{instrument};
//use serde::{Deserialize, Serialize};
use serde_json::{Value};


#[instrument[skip(user_repository, trans_repository)]]
pub async fn transactions(
    user: AuthenticatedUser, 
    user_repository: UserRepository,
    trans_repository: TransRepository,
    params: Data<Params>,
) -> AppResponse {
    
    let user_token = user_repository
        .get_token(user.0)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    let access_token = format!("{}", user_token.access_token);
    let _host = format!("{}/data/v1/accounts", params.api_uri);
    let host_temp = "https://api.truelayer-sandbox.com/data/v1/accounts/56c7b029e0f8ec5a2334fb0ffc2fface/transactions";
    let res = reqwest::Client::new()
        .get(host_temp)
        .bearer_auth(access_token)
        .send()
        .await
        .expect("Getting Transactions");
    
    if res.status().is_success() {
        let body = res.text().await.expect("Reading Body");
        let serialized: Value = serde_json::from_str(&body).unwrap();
        trans_repository
            .save_trans(user.0, body.to_string())
            .await?;

        Ok(HttpResponse::Ok().json(serialized))
    } 
    else {
        let body = res.text().await.expect("Reading Body");
        let serialized: Value = serde_json::from_str(&body).unwrap();
        Ok(HttpResponse::Ok().json(serialized))
    }
    

}
