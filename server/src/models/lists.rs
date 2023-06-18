use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Error, Postgres};
use crate::models::Purchase;

#[derive(sqlx::Type, Deserialize, Serialize, Clone, Eq, PartialEq, Debug, Default)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "address_type_code")]

pub enum ListAccessLevel {
    #[default]
    Private,
    Public,
    ByLink,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "snake_case")]
pub struct List {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub access_level: ListAccessLevel,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl List {
    pub async fn insert_from_request (
        req: CreateListRequest,
        conn: impl Executor<'_, Database = Postgres>
    ) -> Result<Option<Self>, Error>{
        let query = sqlx::query_as(
            "INSERT INTO lists (
                user_id, name, access_level
            ) VALUES ($1, $2, $3)
            ON CONFLICT (user_id, name)
            DO NOTHING
            RETURNING *",
        )
        .bind(req.user_id)
        .bind(req.name)
        .bind(req.access_level);
        
        let res = query.fetch_optional(conn).await?;
        Ok(res)
    }
    pub async fn get_all_by_user (
        user_id: i64,
        conn: impl Executor<'_, Database = Postgres>
    ) -> Result<Vec<Self>, Error>{
        let query = sqlx::query_as(
            "SELECT * FROM lists WHERE user_id = $1",
        )
        .bind(user_id);
        
        let res = query.fetch_all(conn).await?;
        Ok(res)
    }

    pub async fn get_by_id (
        id: i64,
        conn: impl Executor<'_, Database = Postgres>
    ) -> Result<Option<Self>, Error>{
        let query = sqlx::query_as(
            "SELECT * FROM lists WHERE id = $1",
        )
        .bind(id);
        
        let res = query.fetch_optional(conn).await?;
        Ok(res)
    }
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GetListResponse {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub access_level: ListAccessLevel,
    pub created_at: DateTime<Utc>,
    pub purchases: Vec<Purchase>,
}

impl GetListResponse {
    pub fn from_list (list: List, purchases: Vec<Purchase>,) -> Self {
        Self { 
            id: list.id, 
            user_id: list.user_id, 
            name: list.name, 
            access_level: list.access_level, 
            created_at: list.created_at, 
            purchases: purchases, 
        }
    }
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateListRequest {
    pub user_id: i64,
    pub name: String,
    pub access_level: ListAccessLevel,
}