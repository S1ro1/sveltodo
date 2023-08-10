use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use sea_orm::Database;
use tower_http::cors::CorsLayer;

mod jwt_middleware;
mod login;
mod register;

use jwt_middleware::jwt_middleware;
use login::login;
use register::register;

pub async fn router() -> Router {
    dotenv().ok();
    let db_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let db = Database::connect(db_uri).await.unwrap();

    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(middleware::from_fn(jwt_middleware))
        .route("/login", post(login))
        .route("/register", post(register))
        .layer(CorsLayer::permissive())
        .layer(Extension(db));
    router
}
