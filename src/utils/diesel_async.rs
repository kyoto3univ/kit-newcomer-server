use diesel::{r2d2::ConnectionManager, MysqlConnection};
use r2d2::{Pool, PooledConnection};

pub async fn query<F, T>(
    pool: &Pool<ConnectionManager<MysqlConnection>>,
    f: F,
) -> Result<T, anyhow::Error>
where
    F: FnOnce(&PooledConnection<ConnectionManager<MysqlConnection>>) -> Result<T, anyhow::Error>
        + Send
        + 'static,
    T: Send + 'static,
{
    let conn = pool.get()?;
    let res = tokio::task::spawn(async {
        let conn = conn;
        f(&conn)
    })
    .await?;

    res
}
