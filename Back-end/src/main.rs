mod handlers;
mod models;

use axum::{
    Router,
    routing::{post, get},
    http::{Method, HeaderName, Request},
    middleware::Next,
    extract::connect_info::ConnectInfo,
    response::Response,
};
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders};
use std::{env, net::SocketAddr};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use tracing::{info, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};
use tracing_subscriber::util::SubscriberInitExt;

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| env::var("JWT_SECRET").expect("JWT_SECRET não definida"));

async fn inject_ip<B>(ConnectInfo(ip): ConnectInfo<SocketAddr>,mut req: Request<B>,next: Next<B>,) -> Response {
    req.extensions_mut().insert(ip);
    next.run(req).await
}

#[tokio::main]
async fn main() {
    dotenv().expect("Erro ao carregar .env");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL não definida");
    let pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "access.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let pool_clone = pool.clone();

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(non_blocking))
        .with(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(vec![Method::POST])
        .allow_headers(AllowHeaders::from(vec![
            HeaderName::from_static("authorization"),
            HeaderName::from_static("content-type")
        ]));

    let auth_routes = Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .with_state(pool_clone);

    let app = Router::new()
    .merge(auth_routes)
    .nest("/api", Router::new()
        .route("/webstores", post(handlers::create_webstore))
        .route("/products", post(handlers::create_product))
        .route("/webstores", get(handlers::view_webstores))
    )
    .with_state(pool)
    .layer(cors)
    .layer(axum::middleware::from_fn(inject_ip));


    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Servidor rodando em http://localhost:3000");
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
