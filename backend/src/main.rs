mod entity;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    let router = routes::router().await;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
