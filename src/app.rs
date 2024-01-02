use std::sync::Arc;

use axum::Router;
use secrecy::Secret;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    conf::Conf,
    repositories::{create_repositories, RepoImpls},
    routes,
};
use utoipa::{
    openapi::{
        self,
        security::{ApiKey, ApiKeyValue, SecurityScheme},
    },
    Modify, OpenApi,
};

pub struct AppState {
    pub secret: Secret<String>,
    pub repo: RepoImpls,
}

pub async fn serve(conf: Conf) {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            routes::user::create_user,
            routes::user::get_current_user,
            routes::user::login_user,
            routes::user::update_user,
        ),
        components(schemas())
    )]
    struct ApiDoc;
    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "api_key",
                    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("letters_apikey"))),
                )
            }
        }
    }

    let state = Arc::new(AppState {
        repo: create_repositories().await,
        secret: Secret::new(conf.auth.secret),
    });

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .nest("/api", routes::api_router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(Arc::clone(&state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}
