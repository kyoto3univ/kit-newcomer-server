#[macro_use]
extern crate diesel;

mod config;
mod dto;
mod gql;
mod guard;
mod models;
mod mutations;
mod queries;
mod utils;
mod model_resolver;

use config::Config;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use envconfig::Envconfig;
use gql::start_graphql;
use r2d2::Pool;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();

    let config: Config = Config::init_from_env()?;

    let manager = ConnectionManager::<MysqlConnection>::new(&config.database_url);
    let pool = Pool::builder().build(manager)?;

    start_graphql(Arc::new(config), pool).await;

    Ok(())
}
