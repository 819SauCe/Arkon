mod handlers;
mod auth;
mod models;
mod mongo;

use axum::{Router, routing::{post, get}, Server};
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders};
use tower_http::services::ServeDir;
use axum::http::{Method, HeaderName};
use std::{env, net::SocketAddr};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use mongodb::Client;
use crate::handlers::save_product;
use sqlx::postgres::PgPoolOptions;

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| env::var("JWT_SECRET").expect("JWT_SECRET não definida"));

#[tokio::main]
async fn main() {
    dotenv().expect("Falha ao carregar .env");

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI não setada");
    let client = Client::with_uri_str(&mongo_uri).await.expect("Falha na conexão com o MongoDB");
    let db = client.database(&env::var("MONGO_DB").unwrap_or_else(|_| "Arkon".to_string()));
    let db_url = env::var("DATABASE_URL_FOR_WEB").expect("DATABASE_URL_FOR_WEB não definida");
    let pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    let static_files = Router::new().nest_service("/avatars", ServeDir::new("static/avatars"));
    
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(AllowHeaders::from(vec![
            HeaderName::from_static("authorization"),
            HeaderName::from_static("content-type"),
            HeaderName::from_static("accept")
        ]));


    let auth_routes = Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .with_state(pool);

    let mongo_routes = Router::new()
        .route("/product", post(save_product))
        .with_state(db)
        .layer(axum::middleware::from_fn(auth::require_auth));

    let app = Router::new()
        .merge(auth_routes)
        .merge(mongo_routes)
        .merge(static_files)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Servidor rodando em http://localhost:3000");
    Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
