use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use diesel::r2d2;
use diesel::result::Error as DieselError;
use serde_json::json;

#[derive(Debug)]
pub enum TaskApiError {
    // Forbidden,
    Unauthorized,
    InternalServerError,
    NotFoundPage,
    NotFoundData(i32),
    DBError(DBError),
}

impl IntoResponse for TaskApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            TaskApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                String::from("You are unauthorized!")
            ),
            TaskApiError::NotFoundPage => (
                StatusCode::NOT_FOUND,
                String::from("The requested resource was not found!"),
            ),
            TaskApiError::DBError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal database error: {:?}", db_error)
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error!"),
            ),
        };

        (
            status,
            Json(json!({"message": message, "time": chrono::Utc::now().to_string()})),
        ).into_response()
    }
}

#[derive(Debug)]
pub enum DBError {
    NotFound,
    DatabaseError,
}

pub trait DatabaseError {
    fn as_database_error(&self) -> DBError;
}

impl DatabaseError for DieselError {
    fn as_database_error(&self) -> DBError {
        match self {
            DieselError::NotFound => DBError::NotFound,
            _ => DBError::DatabaseError,
        }
    }
}

impl DatabaseError for r2d2::PoolError {
    fn as_database_error(&self) -> DBError {
        DBError::DatabaseError
    }
}

pub fn adapt_database_error<T: DatabaseError>(error: T) -> DBError {
    error.as_database_error()
}