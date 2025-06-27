use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Buscar_id {
    pub id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub sku: String,
    pub active: Option<bool>,
    pub name: String,
    pub desc: Option<String>,
    pub short_desc: Option<String>,
    pub price: Option<f64>,
    pub old_price: Option<f64>,
    pub discount: Option<f64>,
    pub free_shipping: Option<bool>,
    pub category: Vec<String>,
    pub stock: Option<i32>,
    pub unit: String,
    pub created_at: String,
    pub img: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Produto_vitrine {
    id: String,
    name: String,
    price: f64,
    discount: f64,
    old_price: f64,
    img: String,
    stock: i32,
    store: String,
    category: String
}