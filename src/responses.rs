use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use crate::models::{CreateTask, TaskRecord};

pub enum TaskApiResponse {
    Data(Vec<TaskRecord>),
    Created(CreateTask),
    Ok,
}

impl IntoResponse for TaskApiResponse {
    fn into_response(self) -> Response {
        match self {
            TaskApiResponse::Data(data) => (StatusCode::OK, Json(data)).into_response(),
            TaskApiResponse::Created(task_id) => (StatusCode::CREATED, Json(task_id)).into_response(),
            TaskApiResponse::Ok => StatusCode::OK.into_response(),
        }
    }
}