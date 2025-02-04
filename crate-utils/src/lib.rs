pub mod test {
    #![allow(clippy::unwrap_used)]

    use hyper_util::rt::TokioIo;
    use tokio::task::JoinHandle;
    use tonic::transport::{Channel, Endpoint, server::Router};
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
