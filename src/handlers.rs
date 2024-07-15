use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};

use crate::AppState;
use crate::errors::{DBError, TaskApiError};
use crate::models::{CreateTask, CreateTaskRequest, TokenClaims, UpdateTaskRequest, UserLoginInfo};
use crate::responses::TaskApiResponse;
use crate::task_repository::{delete, get_all, insert, update};

pub async fn login_handler(
    Json(user_login_info): Json<UserLoginInfo>
) -> Result<TaskApiResponse, TaskApiError> {
    let login = &user_login_info.login;
    let password = &user_login_info.password;

    let is_valid = !login.trim().is_empty() && !password.trim().is_empty();

    println!("is_valid: {}", is_valid);

    if is_valid {
        let claims = TokenClaims {
            sub: login.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
        };

        let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Error occurred with creating token: {} !", e);
                return Err(TaskApiError::InternalServerError);
            }
        };

        Ok(TaskApiResponse::Authorized(token))
    } else {
        Err(TaskApiError::Unauthorized)
    }
}

pub async fn get_info_after_login_handler
(
    header_map: HeaderMap
) -> Result<TaskApiResponse, TaskApiError> {
    if let Some(auth_header) = header_map.get("Authorization") {
        if let Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.starts_with("Bearer ") {
                let token = auth_header_str.trim_start_matches("Bearer ").to_string();

                return match decode::<TokenClaims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()) {
                    Ok(_) => {
                        println!("You are valid!");
                        Ok(TaskApiResponse::Ok)
                    }
                    Err(e) => {
                        eprintln!("Error occurred with decoding token: {} !", e);
                        Err(TaskApiError::Unauthorized)
                    }
                };
            }
        }
    }
    Err(TaskApiError::Unauthorized)
}

pub async fn get_task(
    State(state): State<AppState>,
) -> Result<TaskApiResponse, TaskApiError> {
    let tasks = get_all(&state.db_pool).map_err(TaskApiError::DBError)?;
    Ok(TaskApiResponse::Data(tasks))
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(task): Json<CreateTaskRequest>,
) -> Result<TaskApiResponse, TaskApiError> {
    let task_id = insert(&state.db_pool, task).map_err(TaskApiError::DBError)?;
    Ok(TaskApiResponse::Created(CreateTask { task_id }))
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
    Json(task_request): Json<UpdateTaskRequest>,
) -> Result<TaskApiResponse, TaskApiError> {
    update(&state.db_pool, task_id, task_request).map_err(|db_error| match db_error {
        DBError::NotFound => TaskApiError::NotFoundData(task_id),
        DBError::DatabaseError => TaskApiError::InternalServerError,
    })?;
    Ok(TaskApiResponse::Ok)
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
) -> Result<TaskApiResponse, TaskApiError> {
    delete(&state.db_pool, task_id).map_err(|db_error| match db_error {
        DBError::NotFound => TaskApiError::NotFoundData(task_id),
        DBError::DatabaseError => TaskApiError::InternalServerError,
    })?;
    Ok(TaskApiResponse::Ok)
}

pub async fn handler_404() -> TaskApiError {
    TaskApiError::NotFoundPage
}