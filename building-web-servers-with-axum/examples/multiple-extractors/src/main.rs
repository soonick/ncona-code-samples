use axum::{
    http::{Method, HeaderMap},
    routing::post,
    Router,
};

async fn post_handler(
    method: Method,
    headers: HeaderMap,
    body: String) -> String {
    format!("Body: {}, Method: {:?}, Headers: {:?}", body, method, headers)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(post_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
