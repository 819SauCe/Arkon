use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub id: i32,
    pub username: String,
    pub role: String,
    pub exp: usize,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub role: String,
    pub exp: usize,
}
