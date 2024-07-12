use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub enum TaskApiError {
    // BadRequest,
    // Forbidden,
    // Unauthorized,
    InternalServerError,
    NotFound,
}

impl IntoResponse for TaskApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            TaskApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error!"),
            ),
            TaskApiError::NotFound => (
                StatusCode::NOT_FOUND,
                String::from("The requested resource was not found!"),
            ),
        };

        (
            status,
            Json(json!({"message": message, "time": chrono::Utc::now().to_string()})),
        ).into_response()
    }
}