use jsonwebtoken::{DecodingKey, Validation, decode, jwk::JwkSet};
use reqwest::Error;
use serde::{Deserialize, Serialize};

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
) -> Result<User, Box<dyn std::error::Error>> {
    let header = jsonwebtoken::decode_header(token)?;
    let kid = header.kid.ok_or("No kid found in token header")?;
    let jwk = jwks.find(&kid).ok_or("No matching JWK found for kid")?;
    let decoding_key = DecodingKey::from_jwk(jwk)?;

    let alg = header.alg;

    let mut validation = Validation::new(alg);

    validation.set_audience(&[audience]);

    let token = decode::<UserClaims>(token, &decoding_key, &validation)?;

    Ok(User::new(token.claims.sub))
}

pub struct User {
    pub id: String,
}
impl User {
    pub fn new(id: String) -> Self {
        User { id }
    }
}
