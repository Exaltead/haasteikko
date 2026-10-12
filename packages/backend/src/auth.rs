use jsonwebtoken::{DecodingKey, Validation, decode, jwk::JwkSet};
use reqwest::Error;
use rusqlite::{OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::{AppState, database::Database};

pub async fn fetch_jwks(url: &str) -> Result<JwkSet, Error> {
    let resp = reqwest::get(url).await?;
    let jwks: JwkSet = resp.json().await?;

    Ok(jwks)
}

#[derive(Debug, Serialize, Deserialize)]
struct UserClaims {
    pub sub: String,
    pub aud: Vec<String>,
}

pub fn validate_jwt(
    token: &str,
    jwks: &JwkSet,
    audience: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let header = jsonwebtoken::decode_header(token)?;
    let kid = header.kid.ok_or("No kid found in token header")?;
    let jwk = jwks.find(&kid).ok_or("No matching JWK found for kid")?;
    let decoding_key = DecodingKey::from_jwk(jwk)?;

    let alg = header.alg;

    let mut validation = Validation::new(alg);

    validation.set_audience(&[audience]);

    let token = decode::<UserClaims>(token, &decoding_key, &validation)?;

    Ok(token.claims.sub)
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

        return match validate_jwt(token, &state.jwks, &state.required_audience) {
            Ok(sub) => {
                return match convert_claim_to_user_id(&sub, &state) {
                    Ok(user_id) => Ok(User::new(user_id)),
                    Err(err) => {
                        println!("{}", err);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Some error happened".to_string(),
                        )
                            .into_response());
                    }
                };
            }
            Err(e) => Err((StatusCode::UNAUTHORIZED, e.to_string()).into_response()),
        };
    }
}

pub fn convert_claim_to_user_id(
    sub: &str,
    state: &AppState,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut db = Database::new(&state.database_path)?;

    if let Some(user_id) = find_user_id(&db.conn, sub)? {
        return Ok(user_id);
    }

    // A new user's first requests often arrive concurrently. Take the write lock up front
    // (IMMEDIATE) and re-check inside it, so concurrent requests queue up and only the first one
    // creates the user; the rest find its row. Without this, each could insert its own row, and
    // the duplicates make every later lookup fail.
    let tx = db
        .conn
        .transaction_with_behavior(TransactionBehavior::Immediate)?;
    if let Some(user_id) = find_user_id(&tx, sub)? {
        return Ok(user_id);
    }

    let new_id = Uuid::new_v4().to_string();
    tx.execute(
        "INSERT INTO user(id, id_claim) VALUES(?,?)",
        &[&new_id, sub],
    )?;
    tx.commit()?;
    Ok(new_id)
}

fn find_user_id(conn: &rusqlite::Connection, sub: &str) -> rusqlite::Result<Option<String>> {
    conn.query_one("SELECT id FROM user WHERE id_claim = ?", &[sub], |f| f.get(0))
        .optional()
}

pub struct User {
    pub id: String,
}
impl User {
    pub fn new(id: String) -> Self {
        User { id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migrations::Migrator;
    use std::path::PathBuf;
    use std::sync::{Arc, Barrier};
    use tempfile::tempdir;

    #[test]
    fn concurrent_first_requests_create_a_single_user() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let migrations_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations");
        Migrator::new(&db_path, &migrations_path)
            .unwrap()
            .run_migrations()
            .unwrap();
        let state = Arc::new(AppState {
            jwks: JwkSet { keys: vec![] },
            required_audience: String::new(),
            database_path: db_path.to_str().unwrap().to_string(),
        });

        const THREADS: usize = 8;
        for claim in 0..20 {
            let sub = format!("claim-{claim}");
            let barrier = Arc::new(Barrier::new(THREADS));
            let handles: Vec<_> = (0..THREADS)
                .map(|_| {
                    let (state, barrier, sub) = (state.clone(), barrier.clone(), sub.clone());
                    std::thread::spawn(move || {
                        barrier.wait();
                        convert_claim_to_user_id(&sub, &state).map_err(|e| e.to_string())
                    })
                })
                .collect();
            let ids: Vec<String> = handles
                .into_iter()
                .map(|h| h.join().unwrap().expect("user lookup failed"))
                .collect();

            assert!(ids.iter().all(|id| id == &ids[0]), "{sub} got different user ids");
            // The user keeps working after the burst
            assert_eq!(convert_claim_to_user_id(&sub, &state).unwrap(), ids[0]);
        }

        let db = Database::new(&state.database_path).unwrap();
        let rows: i64 = db
            .conn
            .query_row("SELECT COUNT(*) FROM user", [], |r| r.get(0))
            .unwrap();
        assert_eq!(rows, 20);
    }
}
