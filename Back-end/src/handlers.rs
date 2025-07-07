use axum::extract::{State, Json, Extension};
use axum::http::StatusCode;
use sqlx::{PgPool, Row};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use std::net::SocketAddr;
use crate::{models::*, JWT_SECRET};

pub async fn register(State(pool): State<PgPool>, Json(payload): Json<RegisterRequest>) -> Result<Json<ApiResponse>, StatusCode> {
    let existing: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE email = $1 OR username = $2")
    .bind(&payload.email)
    .bind(&payload.username)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.0 > 0 {
        return Err(StatusCode::CONFLICT);
    }

    let hash = hash(&payload.password, DEFAULT_COST)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    println!("Register user {}", payload.username);

    sqlx::query("INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)")
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&hash).execute(&pool)
    .await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse { message: "Usuário registrado com sucesso".into() }))
}

pub async fn login(State(pool): State<PgPool>,Extension(ip): Extension<SocketAddr>,Json(payload): Json<LoginRequest>,) -> Result<Json<LoginResponse>, StatusCode> {
    let ip_str = ip.ip().to_string();
    let failures: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM login_attempts
         WHERE ip_address = $1 AND success = false
         AND attempted_at > NOW() - INTERVAL '5 minutes'"
    )
    .bind(&ip_str)
    .fetch_one(&pool)
    .await
    .unwrap_or((0,));

    if failures.0 >= 30 {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
        println!("tentativa de login do ip {}", ip.ip());
    let row = sqlx::query("SELECT id, username, password_hash, role FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_one(&pool)
        .await;

    if let Ok(row) = row {
        let user_id: i32 = row.get("id");
        let stored_hash: String = row.get("password_hash");
        let username: String = row.get("username");
        let role: String = row.try_get("role").unwrap_or("user".into());
        let is_valid = verify(&payload.password, &stored_hash).unwrap_or(false);

        sqlx::query("INSERT INTO login_attempts (ip_address, username, success) VALUES ($1, $2, $3)")
            .bind(&ip_str)
            .bind(&payload.username)
            .bind(is_valid)
            .execute(&pool)
            .await
            .ok();

        if is_valid {
            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(24))
                .unwrap()
                .timestamp() as usize;

            let claims = Claims {
                sub: payload.username.clone(),
                username: username.clone(),
                role: role.clone(),
                exp: expiration,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
            ).unwrap();

            return Ok(Json(LoginResponse {
                token,
                id: user_id,
                exp: expiration,
                username,
                role,
            }));
        }
    }

    sqlx::query("INSERT INTO login_attempts (ip_address, username, success) VALUES ($1, $2, $3)")
        .bind(&ip_str)
        .bind(&payload.username)
        .bind(false)
        .execute(&pool)
        .await
        .ok();

    Err(StatusCode::UNAUTHORIZED)
}