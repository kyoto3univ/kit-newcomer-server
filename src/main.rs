#[macro_use]
extern crate diesel;

mod asset_serve;
mod config;
mod dto;
mod gql;
mod guard;
mod model_resolver;
mod models;
mod mutations;
mod queries;
mod utils;

use config::Config;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use envconfig::Envconfig;
use r2d2::Pool;
use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();

    let config: Arc<Config> = Arc::new(Config::init_from_env()?);

    let manager = ConnectionManager::<MysqlConnection>::new(&config.database_url);
    let pool = Pool::builder().build(manager)?;

    let options_request = warp::options().map(warp::reply).with(
        warp::cors()
            .allow_any_origin()
            .allow_header("Authorization")
            .allow_header("content-type")
            .allow_methods(vec!["GET", "POST", "OPTIONS"])
            .build(),
    );
    let asset_request = asset_serve::asset_resize_handler(config.clone());
    let gql_filter = gql::gql_handler(config.clone(), pool);

    let filter = gql_filter.or(options_request).or(asset_request);

    warp::serve(filter).run(([0, 0, 0, 0], config.port)).await;

    Ok(())
}
