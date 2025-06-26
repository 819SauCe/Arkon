use axum::{http::Request, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::Claims;
use crate::JWT_SECRET;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn require_auth<B>(req: Request<B>, next: Next<B>) -> Result<Response, Response> {
    if let Some(auth) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                match decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET.as_bytes()), &Validation::default()) {
                    Ok(_) => return Ok(next.run(req).await),
                    Err(e) => {
                        eprintln!("Erro ao decodificar token JWT: {:?}", e);
                        return Err(StatusCode::UNAUTHORIZED.into_response());
                    }
                }
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED.into_response())
}