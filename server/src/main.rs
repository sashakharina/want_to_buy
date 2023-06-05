use sqlx::{postgres::{PgConnection}, Connection};

mod config;
mod context;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!(); // defaults to "./migrations"

#[tokio::main]
async fn main() -> Result<(), eyre::Report>{
    let config = config::Config::new()?;
    print!("{:?}", config);
    let mut conn = PgConnection::connect(config.database_url.as_str()).await?;
    MIGRATOR.run(&mut conn).await?;
    let context = context::Context::new_from_config(config);
    Ok(())
}
