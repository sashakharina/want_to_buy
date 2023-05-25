use chrono::{Date, DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Postgres};

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub id: u64,
    pub nickname: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<Date<Utc>>,
    pub created_at: DateTime<UTC>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}