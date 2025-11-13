use jsonwebtoken::{DecodingKey, Validation, decode, jwk::JwkSet};
use reqwest::Error;
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};

use axum::{
    Router,
    extract::FromRequestParts,
    http::{HeaderName, StatusCode, request::Parts},
    response::{IntoResponse, Response},
    routing::get,
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
    let user: Option<String> = db
        .conn
        .query_one("SELECT id FROM user WHERE id_claim = ?", &[sub], |f| {
            f.get(0)
        })
        .optional()?;

    if let Some(user_id) = user {
        return Ok(user_id);
    }

    let new_id = Uuid::new_v4().to_string();

    let tx = db.conn.transaction()?;
    let insert = tx.execute(
        "REPLACE INTO user(id, id_claim) VALUES(?,?)",
        &[&new_id, sub],
    )?;

    if insert == 1 {
        tx.commit()?;
        return Ok(new_id);
    }
    Err("Failed to insert new user".into())
}

pub struct User {
    pub id: String,
}
impl User {
    pub fn new(id: String) -> Self {
        User { id }
    }
}
