pub mod test {
    #![allow(clippy::unwrap_used)]

    use hyper_util::rt::TokioIo;
    use tokio::task::JoinHandle;
    use tonic::transport::{server::Router, Channel, Endpoint};
    use tower::service_fn;

    pub async fn start_server(router: Router) -> (JoinHandle<()>, Channel) {
        let (client, server) = tokio::io::duplex(1024);

        let handle = tokio::spawn(async move {
            router
                .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
                .await
                .unwrap();
        });

        let mut client = Some(client);

        let channel = Endpoint::try_from("http://[::]:50051")
            .unwrap()
            .connect_with_connector(service_fn(move |_| {
                let client = client.take();
                async move {
                    if let Some(client) = client {
                        Ok(TokioIo::new(client))
                    } else {
                        Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Client already taken",
                        ))
                    }
                }
            }))
            .await
            .unwrap();

        (handle, channel)
    }
}

pub mod db {
    use sqlx::{Postgres, Transaction};
    use tonic::metadata::MetadataMap;

    pub async fn setup_db<'c>(
        trx: &mut Transaction<'c, Postgres>,
        metadata: &MetadataMap,
    ) -> Result<(), sqlx::Error> {
        /*
           set "app.jwt.secret" to 'secret';
           set "app.jwt.audience" to 'audd';
           set "app.jwt.issuer" to 'web';
           set "app.jwt.expiry" to '3600';
        */

        sqlx::query!("set role web").execute(trx.as_mut()).await?;

        sqlx::query!("select set_config('app.jwt.issuer',current_user,true)")
            .fetch_one(trx.as_mut())
            .await?;

        if let Ok(secret) = std::env::var("RUST_POSTGRES_JWT_SECRET") {
            sqlx::query!("select set_config('app.jwt.secret',$1,true)", secret)
                .fetch_one(trx.as_mut())
                .await?;
        }
        if let Ok(audience) = std::env::var("RUST_POSTGRES_JWT_AUDIENCE") {
            sqlx::query!("select set_config('app.jwt.audience',$1,true)", audience)
                .fetch_one(trx.as_mut())
                .await?;
        }
        if let Ok(expiry) = std::env::var("RUST_POSTGRES_JWT_EXPIRY") {
            sqlx::query!("select set_config('app.jwt.expiry',$1,true)", expiry)
                .fetch_one(trx.as_mut())
                .await?;
        }

        if let Some(header) = metadata.get("authorization") {
            if let Ok(value) = header.to_str() {
                sqlx::query!("select set_config('request.jwt.token',$1,true)", value)
                    .fetch_one(trx.as_mut())
                    .await?;
            }
        }

        Ok(())
    }
}
