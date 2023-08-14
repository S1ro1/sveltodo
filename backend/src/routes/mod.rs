use axum::{
    middleware,
    routing::{delete, get, post, put},
    Extension, Router,
};
use dotenv::dotenv;
use sea_orm::Database;
use tower_http::cors::CorsLayer;

mod create_task;
mod delete_task;
mod get_user_tasks;
mod jwt_middleware;
mod login;
mod register;
mod update_task;
mod update_task_status;

use create_task::create_task;
use delete_task::delete_task;
use get_user_tasks::get_user_tasks;
use jwt_middleware::jwt_middleware;
use login::login;
use register::register;
use update_task::update_task;
use update_task_status::update_task_status;

pub async fn router() -> Router {
    dotenv().ok();
    let db_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let db = Database::connect(db_uri).await.unwrap();

    let router = Router::new()
        .route("/get_user_tasks", get(get_user_tasks))
        .route("/create_task", post(create_task))
        .route("/update_task/:id", put(update_task))
        .route("/update_task_status/:id", put(update_task_status))
        .route("/delete_task/:id", delete(delete_task))
        .layer(middleware::from_fn(jwt_middleware))
        .route("/login", post(login))
        .route("/register", post(register))
        .layer(CorsLayer::very_permissive())
        .layer(Extension(db));
    router
}
