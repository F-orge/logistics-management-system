use sqlx::{Acquire, Pool, Postgres, Transaction, pool::PoolConnection};

use crate::error::{Error, Result};

pub async fn aquire_connection(pool: &Pool<Postgres>) -> Result<PoolConnection<Postgres>> {
    pool.acquire().await.map_err(Error::Database)
}

pub async fn start_transaction(
    connection: &mut PoolConnection<Postgres>,
) -> Result<Transaction<'_, Postgres>> {
    connection.begin().await.map_err(Error::Database)
}

pub async fn commit_transaction(transaction: Transaction<'_, Postgres>) -> Result<()> {
    transaction.commit().await.map_err(Error::Database)
}
