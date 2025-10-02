use axum::{
    Router,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
    routing::get,
};
/*
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};*/

use crate::auth::User;

mod auth;

#[derive(Clone)]
struct AppState {
    jwks: jsonwebtoken::jwk::JwkSet,
    required_audience: String,
}

impl FromRequestParts<AppState> for User {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing or invalid Authorization header",
                )
                    .into_response()
            })?;

        return match auth::validate_jwt(token, &state.jwks, &state.required_audience) {
            Ok(c) => Ok(c),
            Err(e) => Err((StatusCode::UNAUTHORIZED, e.to_string()).into_response()),
        };
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let jwks_url = std::env::var("JWKS_URL").expect("JWKS_URL must be set");
    let required_audience =
        std::env::var("REQUIRED_AUDIENCE").expect("REQUIRED_AUDIENCE must be set");

    let jwks = auth::fetch_jwks(&jwks_url)
        .await
        .expect("Failed to fetch JWKS");

    let app_state = AppState {
        jwks,
        required_audience,
    };

    let app = Router::new()
        .route("/protected", get(protected))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn protected(user: User) -> String {
    format!("Hello, {}!", user.id)
}
