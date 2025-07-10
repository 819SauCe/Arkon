use axum::extract::{State, Json, Extension};
use axum::http::StatusCode;
use sqlx::{PgPool, Row};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use std::net::SocketAddr;
use chrono::NaiveDateTime;
use crate::{models::*, JWT_SECRET};
use crate::models::{WebstoreListResponse, WebstoreView};


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

    Ok(Json(ApiResponse {message: "Usuário registrado com sucesso".into(),success: true,}))
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

pub async fn create_webstore(State(pool): State<PgPool>,Json(payload): Json<WebstoreRequest>,) -> Result<Json<ApiResponse>, StatusCode> {
    let updated_at = NaiveDateTime::parse_from_str(&payload.updated_at, "%Y-%m-%d %H:%M:%S").map_err(|_| StatusCode::BAD_REQUEST)?;
    let result = sqlx::query!(r#"INSERT INTO webstores (
            name, url, image, description, category, creator_name,
            theme, creator_id, companies, updated_at, user_accept_terms
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        payload.name,
        payload.url,
        payload.image,
        payload.description,
        payload.category,
        payload.creator_name,
        payload.theme,
        payload.creator_id,
        payload.companies,
        updated_at,
        payload.user_accept_terms
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(Json(ApiResponse {
            message: "Webstore criada com sucesso.".into(),
            success: true,
        })),
        Err(e) => {
            eprintln!("Database insert error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_product(State(pool): State<PgPool>,Json(payload): Json<Create_product>,) -> Result<Json<ApiResponse>, StatusCode> {
    let parsed_updated_at = match NaiveDateTime::parse_from_str(&payload.updated_at, "%Y-%m-%d %H:%M:%S") {
        Ok(dt) => dt,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let query = r#"
        INSERT INTO create_product (
            id, webstore_id, name, description, old_price, price, currency,
            show_old_price, shipping, updated_at
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7,
            $8, $9, $10
        )
    "#;

    let result = sqlx::query(query)
        .bind(payload.id)
        .bind(payload.webstore_id)
        .bind(payload.name)
        .bind(payload.description)
        .bind(payload.old_price)
        .bind(payload.price)
        .bind(payload.currency)
        .bind(payload.show_old_price)
        .bind(payload.shipping)
        .bind(parsed_updated_at)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: "Produto criado com sucesso.".to_string(),
        })),
        Err(err) => {
            eprintln!("Erro ao inserir produto: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn view_webstores(State(pool): State<PgPool>) -> Result<Json<WebstoreListResponse>, StatusCode> {
    let result = sqlx::query_as::<_, WebstoreView>(r#"
        SELECT
            w.id,
            w.name,
            w.url,
            w.description,
            w.image,
            w.category,
            w.creator_name,
            w.theme,
            c.name AS company_name,
            w.is_active,
            w.updated_at
        FROM webstores w
        LEFT JOIN companies c ON w.companies = c.id
    "#)
    .fetch_all(&pool)
    .await;

    match result {
        Ok(webstores) => Ok(Json(WebstoreListResponse {
            success: true,
            message: format!("{} lojas encontradas", webstores.len()),
            data: webstores,
        })),
        Err(e) => {
            eprintln!("Erro ao buscar webstores: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
