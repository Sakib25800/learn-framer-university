use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_axum::router::OpenApiRouter;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "api.learn.framer.university",
        description = "API documentation for [learn.framer.university](https://learn.framer.university)",
        contact(name = "Sakibul Islam", email = "sakibulislam25800@gmail.com"),
        version = "0.0.0",
    ),
    modifiers(&SecurityAddon),
    servers(
        (url = "https://learn.framer.university"),
        (url = "https://staging.learn.framer.university"),
        (url = "http://localhost:8080"),
    ),
)]
pub struct BaseOpenApi;

impl BaseOpenApi {
    pub fn router<S>() -> OpenApiRouter<S>
    where
        S: Send + Sync + Clone + 'static,
    {
        OpenApiRouter::with_openapi(Self::openapi())
    }

    pub fn openapi() -> utoipa::openapi::OpenApi {
        <Self as OpenApi>::openapi()
    }
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_default();

        let jwt_scheme = HttpBuilder::new()
            .scheme(HttpAuthScheme::Bearer)
            .bearer_format("JWT")
            .build();

        components.add_security_scheme("bearer", SecurityScheme::Http(jwt_scheme));
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::util::{test_app::TestApp, RequestHelper};

    #[tokio::test(flavor = "multi_thread")]
    async fn test_openapi_snapshot() {
        let (_app, anon) = TestApp::init().empty().await;

        let response = anon.get("/private/openapi.json").await;

        response.assert_status_ok();

        let json_response = response.json::<serde_json::Value>();

        match json_response {
            serde_json::Value::Object(map) => {
                assert!(map.contains_key("openapi"));
            }
            _ => panic!("Expected JSON object"),
        }
    }
}
