use axum::{extract::State, Json};
use axum::http::StatusCode;
use sqlx::{PgPool, Row};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use crate::models::*;
use mongodb::Database;
use crate::mongo::{insert_product, buscar_produtos_ativos};

pub async fn register(State(pool): State<PgPool>, Json(payload): Json<RegisterRequest>) -> Result<Json<ApiResponse>, StatusCode> {
    let email_verify: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE email = $1 OR username = $2")
        .bind(&payload.email)
        .bind(&payload.username)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if email_verify.0 > 0 {
        return Err(StatusCode::CONFLICT);
    }

    let hash = hash(&payload.password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query("INSERT INTO users (username, password, email) VALUES ($1, $2, $3)")
        .bind(&payload.username)
        .bind(&hash)
        .bind(&payload.email)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse { message: "Usuário registrado com sucesso".into() }))
}

pub async fn login(State(pool): State<PgPool>, Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    let result = sqlx::query("SELECT username, password, role FROM users WHERE username = $1")
        .bind(&payload.email)
        .fetch_one(&pool)
        .await;

    if let Ok(row) = result {
        let stored_hash: String = row.get("password");
        let username: String = row.get("username");
        let role: String = row.try_get("role").unwrap_or_else(|_| "user".into());

        if verify(&payload.password, &stored_hash).unwrap_or(false) {
            let expiration = Utc::now().checked_add_signed(Duration::hours(24)).unwrap().timestamp() as usize;
            let claims = Claims { sub: payload.email.clone(), username: username.clone(), role, exp: expiration };
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(crate::JWT_SECRET.as_bytes())).unwrap();
            return Ok(Json(LoginResponse { token, username }));
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn save_product(axum::extract::State(db): axum::extract::State<mongodb::Database>, Json(payload): Json<Product>) -> Json<String> {
    match insert_product(&db, payload).await {
        Ok(_) => {
            println!("Produto salvo com sucesso!");
            Json("Produto salvo com sucesso".into())
        },
        Err(err) => {
            eprintln!("Erro ao salvar produto no MongoDB: {:?}", err);
            Json("Erro ao salvar produto".into())
        }
    }
}

pub async fn listar_produtos(State(db): State<Database>) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let resultado = buscar_produtos_ativos(&db).await;
    match resultado {
        Ok(produtos) => {
            let produtos_json: Vec<serde_json::Value> = produtos.into_iter()
                .map(|doc| serde_json::to_value(doc).unwrap_or_default())
                .collect();
            Ok(Json(produtos_json))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}