use std::process::exit;

use axum::Router;
use clap::{Arg, Command};
use tokio::net::TcpListener;

pub struct CLI {
    command: Command,
    app: Router,
}

impl CLI {
    pub fn new() -> Self {
        Self {
            command: Command::new("CLI Management"),
            app: Router::new(),
        }
    }

    fn setup_address_and_port() -> (String, String) {
        let app_address = std::env::var("APP_ADDRESS").unwrap_or("127.0.0.1".into());
        let app_port = std::env::var("APP_PORT").unwrap_or("8080".into());
        (app_address, app_port)
    }

    pub async fn start(mut self) {
        self = self.version();
        let matches = self.command.get_matches();

        // serve command
        if let Some(serve) = matches.subcommand_matches("serve") {
            let port = match serve.get_one::<String>("port") {
                Some(port) => port,
                None => {
                    tracing::error!("Unable to get port");
                    exit(1);
                }
            };
            let address = match serve.get_one::<String>("address") {
                Some(port) => port,
                None => {
                    tracing::error!("Unable to get port");
                    exit(1);
                }
            };

            let listener = match TcpListener::bind(format!("{}:{}", address, port)).await {
                Ok(listener) => listener,
                Err(err) => {
                    tracing::error!("{}", err);
                    panic!("Unable to bind to address {}:{}", address, port);
                }
            };

            tracing::info!("Listening on {}:{}", address, port);

            match axum::serve(listener, self.app).await {
                Ok(_) => {
                    tracing::info!("Running")
                }
                Err(_) => {
                    tracing::error!("Error occured when starting the application")
                }
            };
        }

        let serve = matches.subcommand_matches("serve");
        println!("{:#?}", serve);
    }

    fn version(mut self) -> Self {
        self.command = self.command.version(env!("CARGO_PKG_VERSION"));
        self
    }

    pub fn serve(mut self, app: Router) -> Self {
        let (address, port) = CLI::setup_address_and_port();
        self.app = app;
        self.command = self.command.subcommand(
            Command::new("serve")
                .about("Serve the application")
                .arg(
                    Arg::new("port")
                        .long("port")
                        .help("port of the application")
                        .default_value(&port),
                )
                .arg(
                    Arg::new("address")
                        .long("address")
                        .help("address of the application")
                        .default_value(&address),
                ),
        );

        self
    }

    pub fn about(mut self, about: impl Into<String>) -> Self {
        self.command = self.command.about(about.into());
        self
    }
}
