use axum::{routing::get,Router};
use tokio::prelude::*;

async fn home() {
    let app = Router::new()
    .router("/", get(|| async {"Encurtator,Desencurtador"}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}