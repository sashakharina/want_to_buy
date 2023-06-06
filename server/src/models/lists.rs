use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Postgres};

#[derive(sqlx::Type, Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "address_type_code")]
pub enum ListAccesslevel {
    Private,
    Public,
    ByLink,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "snake_case")]
pub struct List {
    pub id: u64,
    pub user_id: u64,
    pub name: String,
    pub access_level: ListAccesslevel,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}