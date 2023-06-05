use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Postgres};

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "snake_case")]
pub struct Currency {
    pub code: String,
    pub description: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "snake_case")]
pub struct Purchase {
    pub id: u64,
    pub list_id: u64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_currency: Option<String>,
    pub priority: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<String>,
    pub is_done: bool,
    pub created_at: DateTime<UTC>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}