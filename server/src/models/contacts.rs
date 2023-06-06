use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Postgres};

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "snake_case")]
pub struct Contacts {
    pub subscriber_id: u64,
    pub subscribing_id: u64,
    pub is_accept: bool,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}