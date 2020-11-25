use crate::{
    config::crypto::CryptoService,
    errors::AppError,
    models::trans::{CheckCache, Transactions},
    models::user::{NewUser, User},
};
use actix_web::{web::Data, FromRequest};
use color_eyre::Result;
use futures::future::{ready, Ready};
use serde_json::Value;
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};
use tracing::instrument;
use uuid::Uuid;

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    #[instrument(skip(self, new_user, hashing))]
    pub async fn create(&self, new_user: NewUser, hashing: &CryptoService) -> Result<User> {
        let password_hash = hashing.hash_password(new_user.password).await?;

        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(new_user.username)
        .bind(new_user.email)
        .bind(password_hash)
        .fetch_one(&*self.pool)
        .await?;
        Ok(user)
    }

    #[instrument(skip(self))]
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let maybe_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(maybe_user)
    }

    #[instrument(skip(self))]
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let maybe_user = sqlx::query_as::<_, User>("select * from users where id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(maybe_user)
    }

    pub async fn update_code(&self, code: String) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("update users set code = $1 returning *")
            .bind(code)
            .fetch_optional(&*self.pool)
            .await?;
        Ok(user)
    }

    pub async fn update_token(&self, access_token: String) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("update users set access_token = $1 returning *")
            .bind(access_token)
            .fetch_optional(&*self.pool)
            .await?;
        Ok(user)
    }

    #[instrument(skip(self))]
    pub async fn get_token(&self, id: Uuid) -> Result<Option<User>> {
        let maybe_token = sqlx::query_as::<_, User>("select * from users where id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(maybe_token)
    }

    #[instrument(skip(self))]
    pub async fn check_cache(&self, user_id: Uuid) -> Result<Option<CheckCache>> {
        let maybe_cached = sqlx::query_as::<_, CheckCache>(
            r#"select count(*) as results from transactions where user_id=$1"#,
        )
        .bind(user_id)
        .fetch_optional(&*self.pool)
        .await?;
        Ok(maybe_cached)
    }

    #[instrument(skip(self))]
    pub async fn get_cache(&self, user_id: Uuid) -> Result<Transactions> {
        let maybe_cached = sqlx::query_as::<_, Transactions>
        (r#"select user_id, created_at, (results #>> '{}')::jsonb->>'results' as results from transactions where user_id=$1"#)
            .bind(user_id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(maybe_cached)
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

impl FromRequest for UserRepository {
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
            Ok(pool) => ready(Ok(UserRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}
