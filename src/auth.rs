use axum::{async_trait, extract::{FromRequestParts}, http::request::Parts};
// ! use axum::State;
use axum_extra::{TypedHeader, headers::{Authorization, authorization::Bearer}};
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::{error::AppError, state::AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,
}

pub fn sign(sub: &str, secret: &str, ttl_minutes: i64) -> anyhow::Result<String> {
    let exp = (Utc::now() + Duration::minutes(ttl_minutes)).timestamp() as usize;
    let claims = Claims { sub: sub.into(), exp };
    Ok(encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(secret.as_bytes()))?)
}

pub fn verify(token: &str, secret: &str) -> anyhow::Result<Claims> {
    let data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS256))?;
    Ok(data.claims)
}

pub struct AuthUser { pub user_id: String }

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, app: &AppState) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, &())
                .await
                .map_err(|_| AppError::Unauthorized)?;
        let claims = verify(bearer.token(), &app.jwt_secret).map_err(|_| AppError::Unauthorized)?;
        Ok(AuthUser { user_id: claims.sub })
    }
}