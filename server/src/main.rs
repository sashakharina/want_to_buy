use sqlx::{postgres::{PgConnection}, Connection};
use actix_web::{HttpServer, App, web::Data, middleware::Logger, get};
use futures_util::TryFutureExt;
use crate::models::Currency;

mod config;
mod context;
mod controllers;
mod models;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!(); // defaults to "./migrations"

#[tokio::main]
async fn main() -> Result<(), eyre::Report>{
    let config = config::Config::new()?;
    
    print!("{:?}", config);
    let mut conn = PgConnection::connect(config.database_url.as_str()).await?;
    MIGRATOR.run(&mut conn).await?;
    let context = context::Context::new_from_config(config.clone())?;

    for (c, d) in config.currencies {
        let conn = context.db_pool.acquire().await?;
        Currency {
            code: c,
            description: d,
        }.insert_with_description(conn).await?;
    }

    let app = HttpServer:: new ( move || {
        App::new()
        .wrap(Logger::default())
        .app_data(Data::new(context.clone()))
        .service(health)
        .configure(controllers::configure)
    })
    .bind(config.listen_addr)?;

    

    let http_app_task = tokio::spawn(app.run().map_err(eyre::Report::from));
    tokio::select! {
        res = http_app_task => res??
    }
    Ok(())
}

#[get("/health")]
async fn health() -> &'static str {
    "Status OK"
}
