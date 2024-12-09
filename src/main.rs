use axum::{routing::get, Router};
use deaddrop::api::{api_get, api_insert};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let db = Arc::new(sled::open("db").unwrap());
    let app = Router::new()
        .route("/", get(api_get).post(api_insert))
        .with_state(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9998").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
