extern crate r2d2;
#[macro_use]
extern crate diesel;
extern crate anyhow;
extern crate dotenv;

use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use gql::start_graphql;
use r2d2::Pool;

mod gql;
mod models;
mod mutations;
mod queries;
mod utils;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<MysqlConnection>::new(&db_url);
    let pool = Pool::builder().build(manager)?;

    start_graphql(pool).await;

    Ok(())
}
