use axum::extract::{Path, State};
use axum::Json;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::AppState;
use crate::errors::TaskApiError;
use crate::models::{CreateTask, CreateTaskRequest, TaskRecord, UpdateTaskRequest};
use crate::responses::TaskApiResponse;
use crate::schemas::tasks;
use crate::schemas::tasks::dsl::tasks as tasks_table;
use crate::schemas::tasks::task_id as task_record_id;

pub async fn get_task(
    State(state): State<AppState>,
) -> Result<TaskApiResponse, TaskApiError> {

    //TODO: handle error from db!
    let db_pool = &mut state.db_pool.get().expect("");

    //TODO: handle error
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

    //TODO: handle error from db!
    let db_pool = &mut state.db_pool.get().expect("");

    //TODO: handle error
    let task_id: i32 = diesel::insert_into(tasks::table)
        .values(task)
        .returning(task_record_id)
        .get_result(db_pool)
        .expect("Error save task in db!");

    Ok(TaskApiResponse::Created(CreateTask { task_id }))
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
    Json(task_request): Json<UpdateTaskRequest>,
) -> Result<TaskApiResponse, TaskApiError> {

    //TODO: handle error from db!
    let db_pool = &mut state.db_pool.get().expect("");

    //TODO: handle error
    //None fields skipped by defaults!
    //https://diesel.rs/guides/all-about-updates.html
    diesel::update(tasks_table.find(task_id))
        .set(task_request)
        .execute(db_pool)
        .expect("Error to update task in db!");

    Ok(TaskApiResponse::Ok)
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
) -> Result<TaskApiResponse, TaskApiError> {

    //TODO: handle error
    let db_pool = &mut state.db_pool.get().expect("");

    //TODO: handle error
    diesel::delete(tasks_table.find(task_id))
        .execute(db_pool)
        .expect("Error to delete task from db!");

    Ok(TaskApiResponse::Ok)
}

pub async fn handler_404() -> TaskApiError {
    TaskApiError::NotFound
}