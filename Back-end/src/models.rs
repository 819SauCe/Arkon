use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
    pub success: bool,
}

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

#[derive(Serialize, Deserialize)]
pub struct WebstoreRequest {
    pub name: String,
    pub url: String,
    pub image: String,
    pub description: String,
    pub category: String,
    pub creator_name: String,
    pub theme: String,
    pub creator_id: i32,
    pub companies: i32,
    pub updated_at: String,
    pub user_accept_terms: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Create_product {
    pub id: i32,
    pub webstore_id: i32,
    pub name: String,
    pub description: String,
    pub old_price: i32,
    pub price: i32,
    pub currency: String,
    pub show_old_price: bool,
    pub shipping: bool,
    pub updated_at: String,
}
#[derive(Serialize, Deserialize)]
pub struct Product_img {
    pub id: i32,
    pub product_id: i32,
    pub image: String
}
#[derive(Serialize, Deserialize)]
pub struct Product_create_for {
    pub id: i32,
    pub product_id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub creat_at: String,
}

#[derive(Serialize, FromRow)]
pub struct WebstoreView {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub category: Option<String>,
    pub creator_name: Option<String>,
    pub theme: Option<String>,
    pub company_name: Option<String>,
    pub is_active: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct WebstoreListResponse {
    pub success: bool,
    pub message: String,
    pub data: Vec<WebstoreView>,
}