use utoipa::{
    Modify, OpenApi,
    openapi::security::{Http, SecurityScheme},
};

#[derive(OpenApi, Debug)]
#[openapi(modifiers(&SecurityAddon),info(description = "Logistics Management System"))]
pub struct OpenAPI;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer",
                SecurityScheme::Http(
                    Http::builder()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
