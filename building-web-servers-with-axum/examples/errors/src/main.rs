use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Serialize;

pub enum AppError {
    ServerError(String),
    ClientError(Vec<String>),
}

#[derive(Serialize)]
struct ErrorResponse {
    data: Vec<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {

            AppError::ServerError(msg) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        data: vec!(msg),
                    }),
                ).into_response();
            }
            AppError::ClientError(messages) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        data: messages,
                    }),
                ).into_response();
            }
        };
    }
}

async fn no_response() {
    panic!("Some error");
}

async fn ok_response() -> StatusCode {
    StatusCode::OK
}

async fn internal_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn bad_request() -> StatusCode {
    StatusCode::BAD_REQUEST
}

async fn with_result() -> Result<String, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

async fn custom_server_error() -> Result<String, AppError> {
    Err(AppError::ServerError("Our system is down".to_string()))
}

async fn custom_client_error() -> Result<String, AppError> {
    Err(AppError::ClientError(
        vec!(
            "Field tacos must be set".to_string(),
            "Field email is invalid".to_string()
        )
    ))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/no-response", get(no_response))
        .route("/ok", get(ok_response))
        .route("/internal-error", get(internal_error))
        .route("/bad-request", get(bad_request))
        .route("/with-result", get(with_result))
        .route("/custom-client-error", get(custom_client_error))
        .route("/custom-server-error", get(custom_server_error));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
