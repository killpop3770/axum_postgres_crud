use axum::extract::{Path, State};
use axum::Json;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::AppState;
use crate::errors::TaskApiError;
use crate::models::{CreateTask, CreateTaskRequest, TaskRecord, UpdateTaskRequest};
use crate::responses::TaskApiResponse;
use crate::schemas::tasks;
use crate::schemas::tasks::dsl::tasks as tasks_table;

pub async fn get_task(
    State(state): State<AppState>,
) -> Result<TaskApiResponse, TaskApiError> {
    //TODO: handle error from db!
    let db_pool = &mut state.db_pool.get().expect("");

    let rows = tasks_table
        .select(TaskRecord::as_select())
        .load::<TaskRecord>(db_pool)
        .expect("Error to load tasks from db!");

    Ok(TaskApiResponse::Data(rows))
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(task): Json<CreateTaskRequest>,
) -> Result<TaskApiResponse, TaskApiError> {
    // let db_pool = &mut state.db_pool.get().expect("");
    //
    // let row = diesel::insert_into(tasks::table)
    //     .values(task)
    //     .returning(CreateTask::as_returning())
    //     .get_result(db_pool)
    //     .expect("Error save task in db!");
    //
    // Ok(TaskApiResponse::Created(row))
    Ok(TaskApiResponse::Ok)
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
    Json(update_task_request): Json<UpdateTaskRequest>,
) -> Result<TaskApiResponse, TaskApiError> {
//     let mut query = String::from("UPDATE tasks SET task_id = $1");
//
//     let mut variable_number = 2;
//
//     if update_task_request.name.is_some() {
//         query.push_str(format!(", name = ${}", variable_number).as_str());
//         variable_number += 1;
//     }
//
//     if update_task_request.priority.is_some() {
//         query.push_str(format!(", priority = ${}", variable_number).as_str());
//     }
//
//     query.push_str(" WHERE task_id = $1");
//
//     let mut main_query = sqlx::query(&state).bind(task_id);
//
//     if update_task_request.name.is_some() {
//         main_query = main_query.bind(update_task_request.name);
//     }
//
//     if update_task_request.priority.is_some() {
//         main_query = main_query.bind(update_task_request.priority);
//     }
//
//     main_query
//         .execute(&state)
//         .await
//         .map_err(|_| TaskApiError::InternalServerError)?;

    Ok(TaskApiResponse::Ok)
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
) -> Result<TaskApiResponse, TaskApiError> {

//     sqlx::query!(
//         "DELETE FROM tasks WHERE task_id = $1",
//         task_id
//     )
//         .execute(&state)
//         .await
//         .map_err(|_| TaskApiError::InternalServerError)?;

    Ok(TaskApiResponse::Ok)
}

pub async fn handler_404() -> TaskApiError {
    TaskApiError::NotFound
}