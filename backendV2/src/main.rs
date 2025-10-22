use axum::{
    Router,
    extract::FromRequestParts,
    http::{HeaderName, StatusCode, request::Parts},
    response::{IntoResponse, Response},
    routing::get,
};
use tower_http::cors::{Any, CorsLayer};

use crate::auth::User;

mod auth;
mod challenge;
mod database;
mod library;
mod migrations;

#[derive(Clone)]
struct AppState {
    jwks: jsonwebtoken::jwk::JwkSet,
    required_audience: String,
    database_path: String,
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
    let database_path =
        std::env::var("DATABASE_PATH").unwrap_or_else(|_| "database.sqlite".to_string());

    let jwks_url = std::env::var("JWKS_URL").expect("JWKS_URL must be set");
    let required_audience =
        std::env::var("REQUIRED_AUDIENCE").expect("REQUIRED_AUDIENCE must be set");

    let jwks = auth::fetch_jwks(&jwks_url)
        .await
        .expect("Failed to fetch JWKS");

    let mut migrator = migrations::Migrator::new(&database_path, &"migrations".to_string())
        .expect("Failed to create migrator");
    migrator.run_migrations().expect("Failed to run migrations");

    let app_state = AppState {
        jwks,
        required_audience,
        database_path: database_path,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers([
            HeaderName::from_static("authorization"),
            HeaderName::from_static("content-type"),
        ]);

    let app = Router::new()
        .route("/protected", get(protected))
        .nest("/api", library::library_routes())
        .nest("/api", challenge::routes())
        .with_state(app_state)
        // For local development, disallow when deploying
        .layer(cors);

    let address = "0.0.0.0:3000";
    println!("Listening on http://{}", address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn protected(user: User) -> String {
    format!("Hello, {}!", user.id)
}
