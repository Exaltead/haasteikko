use axum::{Router, http::HeaderName, routing::get};
use tower_http::cors::{Any, CorsLayer};

mod auth;
mod challenge;
mod challenge_answers;
mod database;
mod library;
mod migrations;
mod solution;
mod utils;

#[derive(Clone)]
struct AppState {
    jwks: jsonwebtoken::jwk::JwkSet,
    required_audience: String,
    database_path: String,
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

    let migrations_path = std::env::var("MIGRATIONS_PATH").expect("MIGRATIONS_PATH must be set");

    let mut migrator = migrations::Migrator::new(&database_path, &migrations_path)
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
        .route("/api/ping", get(ping))
        .nest("/api", library::library_routes())
        .nest("/api", challenge::routes())
        .nest("/api", solution::routes())
        .nest("/api", challenge_answers::routes())
        .with_state(app_state)
        // For local development, disallow when deploying
        .layer(cors);

    let address = "0.0.0.0:3000";
    println!("Listening on http://{}", address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ping() -> String {
    format!("PONG")
}
