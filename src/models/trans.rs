use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize)]
//#[serde(rename_all = "camelCase")]
pub struct Transactions {
    pub results: String,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
//#[serde(rename_all = "camelCase")]
pub struct CheckCache {
    pub results: i64,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct TransactionsResults {
    pub results: Vec<TransactionsAccount>,
    //status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionsAccount {
    pub timestamp: String,
    pub description: String,
    pub transaction_type: String,
    pub transaction_category: String,
    pub amount: f32,
    pub currency: String,
    pub transaction_id: String,
}
