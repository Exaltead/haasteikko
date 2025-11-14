use axum::http::StatusCode;

pub fn map_to_internal_error(err: Box<dyn std::error::Error>) -> StatusCode {
    println!("Internal error: {}", err);
    StatusCode::INTERNAL_SERVER_ERROR
}
