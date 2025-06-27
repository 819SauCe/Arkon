mod handlers;
mod auth;
mod models;
mod mongo;

use axum::{
    Router,
    routing::{post, get, delete},
    http::{Method, HeaderName, Request},
    middleware::Next,
    response::Response,
    extract::connect_info::ConnectInfo,
};
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders};
use tower_http::services::ServeDir;
use std::{env, net::SocketAddr};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use mongodb::Client;
use crate::handlers::{s_product, listar_produtos, d_product};
use sqlx::postgres::PgPoolOptions;
use tracing::{info, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};
use tracing_subscriber::util::SubscriberInitExt;

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| env::var("JWT_SECRET").expect("JWT_SECRET não definida"));

async fn log_ip<B>(ConnectInfo(addr): ConnectInfo<SocketAddr>, req: Request<B>, next: Next<B>) -> Response {
    info!("IP: {}", addr);
    next.run(req).await
}

#[tokio::main]
async fn main() {
    dotenv().expect("Falha ao carregar .env");

    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "access.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(non_blocking))
        .with(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    let mongo_conection = env::var("MGDB").expect("MGDB não setada");
    let client = Client::with_uri_str(&mongo_conection).await.expect("Falha na conexão com o MongoDB");
    let db = client.database(&env::var("MGTB").unwrap_or_else(|_| "Arkon".to_string()));
    let db_clone = db.clone();
    let db_url = env::var("PGDB").expect("PGDB não definida");
    let pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    let static_files = Router::new().nest_service("/avatars", ServeDir::new("static/avatars"));
    
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS, Method::DELETE])
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
        .route("/s_product", post(s_product))
        .route("/d_product", delete(d_product))
        .with_state(db.clone())
        .route_layer(axum::middleware::from_fn(auth::require_auth));

    let public_routes = Router::new()
        .route("/products", get(listar_produtos))
        .with_state(db_clone);

    let app = Router::new()
        .merge(auth_routes)
        .merge(mongo_routes)
        .merge(public_routes)
        .merge(static_files)
        .layer(cors)
        .layer(axum::middleware::from_fn_with_state((), log_ip));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Servidor rodando em http://localhost:3000");
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
