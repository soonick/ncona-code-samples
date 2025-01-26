use axum::{
    routing::post,
    Router,
};

async fn post_handler(body: String) -> String {
    format!("Request body was: {}", body)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(post_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
