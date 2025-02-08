use axum::routing::{get, post};
use tonic::transport::Channel;
pub mod login;

#[derive(Clone)]
pub struct AuthenticationExtension {
    pub grpc_client: crate_proto::auth::auth_service_client::AuthServiceClient<Channel>,
    pub destination_url: String,
    pub action_url: String,
}

impl base::Extension for AuthenticationExtension {
    fn name(&self) -> String {
        "Authentication service".into()
    }
    fn navigation(&self) -> base::ExtensionNavigation {
        base::ExtensionNavigation {
            name: "Authentication service".into(),
            items: vec![base::ExtensionNavigationItem {
                name: "login".into(),
                path: "/login".into(),
                children: None,
            }],
        }
    }
    fn router(&self) -> axum::Router {
        axum::Router::new()
            .route("/login", get(login::login))
            .route("/login", post(login::login_action))
            .with_state(self.clone())
    }
}
