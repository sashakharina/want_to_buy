use sqlx::{postgres::{PgConnection}, Connection};

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!(); // defaults to "./migrations"
#[tokio::main]
async fn main() -> Result<(), eyre::Report>{
    let mut conn = PgConnection::connect("postgres://postgres:password@wtb-postgres/want_to_buy").await?;
    MIGRATOR.run(&mut conn).await?;
    Ok(())
}
