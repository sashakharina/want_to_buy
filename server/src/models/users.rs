use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Error, Executor, query::QueryAs, postgres::PgArguments};
use uuid::Uuid;

use super::List;

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub id: i64,
    pub nickname: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UserPublic {
    pub nickname: String,
    pub email: String,
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<DateTime<Utc>>,
}

impl From <User> for UserPublic {
    fn from(value: User) -> Self {
        Self { 
            nickname: (value.nickname), 
            email: (value.email), 
            phone_number: (value.phone_number), 
            photo: (value.photo), 
            date_of_birth: (value.date_of_birth) }
    }
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Session {
    pub session_id: String,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub async fn insert_session (
        user_id: i64,
        conn: impl Executor<'_, Database = Postgres>
    ) -> Result<Self, Error>{
        let query = sqlx::query_as(
            "INSERT INTO session (
                session_id, user_id
            ) VALUES ($1, $2)
            RETURNING *",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(user_id);
        
        let res = query.fetch_one(conn).await?;
        Ok(res)
    }

    pub async fn get_session (
        session_id: &str,
        conn: impl Executor<'_, Database = Postgres>
    ) -> Result<Option<Self>, Error>{
        let query = sqlx::query_as(
            "SELECT * FROM WHERE session_id = $1",
        )
        .bind(session_id);
        
        let res = query.fetch_optional(conn).await?;
        Ok(res)
    }

    pub async fn drop_session (
        session_id: &str,
        conn: impl Executor<'_, Database = Postgres>
    ) -> Result<(), Error>{
        let query: QueryAs<Postgres, Session, PgArguments> = sqlx::query_as(
            "DELETE FROM session WHERE session_id = $1",
        )
        .bind(session_id);
        
        conn.execute(query).await?;
        //query.fetch_optional(conn).await?;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserRequest {
    pub nickname: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<DateTime<Utc>>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoginResponse {
    pub user_info: User,
    pub session_id: String,
    pub lists: Vec<List>,
}

impl User {
    pub async fn insert_from_request (
        req: CreateUserRequest,
        conn: impl Executor<'_, Database = Postgres>
    ) -> Result<Option<Self>, Error>{
        let query = sqlx::query_as(
            "INSERT INTO users (
                nickname, email, password, phone_number, date_of_birth
            ) VALUES ($1, $2, $3, $4, $5)
            RETURNING *",
        )
        .bind(req.nickname)
        .bind(req.email)
        .bind(req.password)
        .bind(req.phone_number)
        .bind(req.date_of_birth);
        
        let res = query.fetch_optional(conn).await?;
        Ok(res)
    }

    pub async fn get_by_email (
        email: &str,
        conn: impl Executor<'_, Database = Postgres>
    ) -> Result<Option<Self>, Error>{
        let query = sqlx::query_as(
            "SELECT * FROM users WHERE email = $1",
        )
        .bind(email);
        
        let res = query.fetch_optional(conn).await?;
        Ok(res)
    }
}