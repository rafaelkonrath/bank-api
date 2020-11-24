use crate::{
    errors::AppError,
    models::trans::{Transactions}
};
use actix_web::{web::Data, FromRequest};
use color_eyre::Result;
use futures::future::{ready, Ready};
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
//use sqlx::types::Json;
use std::{ops::Deref, sync::Arc};
use tracing::{instrument};
use uuid::Uuid;
use serde_json::{Value};

pub struct TransRepository {
    pool: Arc<PgPool>,
}

impl TransRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    #[instrument(skip(self))]
    pub async fn save_trans(&self, user_id: Uuid, json_trans: String) -> Result<()> {

        let serialized: Value = serde_json::from_str(&json_trans).unwrap();
        
        sqlx::query_as::<_, Transactions>(
            "INSERT INTO transactions (user_id, results) VALUES($1, to_json($2))",
        )
            .bind(user_id)
            .bind(serialized)
            .fetch_optional(&*self.pool)
            .await?;
        
        Ok(())
    }
}

impl FromRequest for TransRepository {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();
    #[instrument(skip(req, payload))]
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let pool_result = Data::<PgPool>::from_request(req, payload).into_inner();

        match pool_result {
            Ok(pool) => ready(Ok(TransRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}
