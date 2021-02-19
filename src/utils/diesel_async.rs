use diesel::{connection::TransactionManager, prelude::*, r2d2::ConnectionManager};
use futures::Future;
use r2d2::Pool;

pub async fn transaction<T, E, F>(
    pool: &Pool<ConnectionManager<MysqlConnection>>,
    f: F,
) -> Result<T, E>
where
    F: Future<Output = Result<T, E>>,
    E: From<diesel::result::Error> + From<r2d2::Error>,
{
    let c = pool.get()?;
    let transaction_manager = c.transaction_manager();
    transaction_manager.begin_transaction(&c)?;
    match f.await {
        Ok(value) => {
            transaction_manager.commit_transaction(&c)?;
            Ok(value)
        }
        Err(e) => {
            transaction_manager.rollback_transaction(&c)?;
            Err(e)
        }
    }
}
