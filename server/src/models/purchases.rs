use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, error::Error, pool::PoolConnection, Executor};

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "snake_case")]
pub struct Currency {
    pub code: String,
    pub description: String,
}

impl Currency {
    pub async fn insert_with_description(
        self,
        mut conn: PoolConnection<Postgres>,
    ) -> Result<Option<Self>, Error>{
        let query = sqlx::query_as(
            "INSERT INTO currencies (
                code, description
            ) VALUES ($1, $2)
            ON CONFLICT (code)
            DO NOTHING
            RETURNING *",
        )
        .bind(self.code)
        .bind(self.description);
        
        let res = query.fetch_optional(&mut conn).await?;
        Ok(res)
    }

    pub async fn get_all(
        conn: impl Executor<'_, Database = Postgres>,
    ) -> Result<Vec<Self>, Error>{
        let query = sqlx::query_as(
            "SELECT * FROM currencies",
        );
        
        let res = query.fetch_all(conn).await?;
        Ok(res)
    }
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
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}