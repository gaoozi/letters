use axum::Router;
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::Any;
use tower_http::{
    cors::CorsLayer,
    trace::{self, TraceLayer},
};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

use crate::{api, conf::Conf, handlers::openapi::ApiDoc};

pub struct AppState {
    pub dbc: Arc<DatabaseConnection>,
    pub conf: Arc<Conf>,
}

pub async fn serve(port: u16, conf: &Conf) {
    let db_url = conf.database.url.clone().unwrap_or("".to_string());
    let dbc = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let state = Arc::new(AppState {
        dbc: Arc::new(dbc),
        conf: Arc::new(conf.clone()),
    });

    let cors_layer = CorsLayer::new().allow_origin(Any).allow_methods(Any);

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .nest("/api", api::router())
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                        .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(cors_layer),
        )
        .with_state(Arc::clone(&state));

    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::debug!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}
