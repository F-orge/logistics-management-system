use sqlx::{Acquire, Pool, Postgres, Transaction, pool::PoolConnection};
use tokio_stream::wrappers::ReceiverStream;
use tonic::Status;

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

pub async fn stream_response<T: Send + Sync + 'static>(
    rows: impl Iterator<Item = T> + Send + Sync + 'static,
) -> ReceiverStream<std::result::Result<T, Status>> {
    let (tx, rx) = tokio::sync::mpsc::channel(1024 * 64);

    tokio::spawn(async move {
        for row in rows {
            let _ = tx
                .send(Ok(row))
                .await
                .map_err(|err| Error::Custom(Box::new(err)));
        }
    });

    ReceiverStream::new(rx)
}
