use std::pin::Pin;
use tokio::pin;

use tokio_stream::{Stream, StreamExt, wrappers::ReceiverStream};
use tonic::Status;

// stream sqlx
pub async fn stream_sqlx<T: Send + Sync + 'static>(
    rows: impl Stream<Item = sqlx::Result<T>>,
) -> ReceiverStream<Result<T, Status>> {
    pin!(rows);
    let (tx, rx) = tokio::sync::mpsc::channel(1024 * 64);
    while let Some(row) = rows.try_next().await.transpose() {
        if let Ok(row) = row {
            let _ = tx
                .send(Ok(row))
                .await
                .map_err(|err| crate::error::Error::Custom(Box::new(err)));
        }
    }
    ReceiverStream::new(rx)
}

pub async fn stream_iterator<T: Send + Sync + 'static>(
    items: impl Iterator<Item = T> + Send + Sync + 'static,
) -> ReceiverStream<Result<T, Status>> {
    let (tx, rx) = tokio::sync::mpsc::channel(1024 * 64);
    for item in items {
        let _ = tx
            .send(Ok(item))
            .await
            .map_err(|err| crate::error::Error::Custom(Box::new(err)));
    }

    ReceiverStream::new(rx)
}
