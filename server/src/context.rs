use sqlx::{postgres::PgPool, error::Error};

use crate::config;

#[derive(Clone, Debug)]
pub struct Context {
    pub db_pool: PgPool,
    pub config: config::Config,
}

impl Context {
    pub fn new_from_config(config: config::Config) -> Result<Self, Error> {
        let db_pool = PgPool::connect_lazy(config.database_url.as_str())?;
        Ok(Self {
            config,
            db_pool,
        })
    }
}