use crate::dto::auth::*;
use crate::error::{AppError, ErrorResponse};
use crate::handlers;
use crate::utils::jwt::AuthClaims;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    info(
        version = "v0.1.0",
        title = "Letters API"
    ),
    paths(
        handlers::auth::authorize,
    ),
    components(
        schemas(
            AuthClaims,
            AuthRequest,
            AuthResponse,
            AppError,
            ErrorResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "handlers::auth", description = "server auth endpoints"),
    ),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
            )
        }
    }
}
