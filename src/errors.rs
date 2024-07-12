use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::json;

pub enum TaskApiError {
    // BadRequest,
    // Forbidden,
    // Unauthorized,
    InternalServerError
}

impl IntoResponse for TaskApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            TaskApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            )
        };

        (
            status,
            Json(json!({"message": message, "time": chrono::Utc::now().to_string()})),
        ).into_response()
    }
}