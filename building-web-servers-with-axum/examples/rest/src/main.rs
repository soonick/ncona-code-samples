use axum::{
    extract::Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct TheRequest {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct TheResponse {
    greeting: String,
}

#[axum::debug_handler]
async fn post_handler(Json(req): Json<TheRequest>) -> Json<TheResponse> {
    Json(TheResponse {
        greeting: format!("Hello {}", req.name)
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(post_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
