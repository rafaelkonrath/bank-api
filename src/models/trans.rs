use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, sqlx::FromRow, Serialize)]
//#[serde(rename_all = "camelCase")]
pub struct Transactions {
    results: String,
}


#[derive(Debug, sqlx::FromRow, Serialize)]
//#[serde(rename_all = "camelCase")]
pub struct TransactionsResults {
    results: Vec<TransactionsAccount>,
    status: String,
}

#[derive(Debug, Deserialize, Serialize)]
//#[serde(rename_all = "camelCase")]
pub struct TransactionsAccount {
    timestamp: String,
    description: String,
    transaction_type: String,
    transaction_category: String,
    transaction_classification: Vec<String>,
    amount: f32,
    currency: String,
    transaction_id: String,
    running_balance: Balance,
}

#[derive(Debug, Deserialize, Serialize)]
//#[serde(rename_all = "camelCase")]
pub struct Balance {
    currency: String,
    amount: f32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCode {
    pub code: String,
}