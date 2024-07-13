use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::r2d2::{ConnectionManager, Pool};

use crate::errors::{adapt_database_error, DBError};
use crate::models::{CreateTaskRequest, TaskRecord, UpdateTaskRequest};
use crate::schemas::tasks;

pub fn get_all(
    db_pool: &Pool<ConnectionManager<PgConnection>>
) -> Result<Vec<TaskRecord>, DBError> {
    let connection = &mut db_pool
        .get()
        .map_err(adapt_database_error)?;

    let rows = tasks::table
        .select(TaskRecord::as_select())
        .load::<TaskRecord>(connection)
        .map_err(adapt_database_error)?;

    Ok(rows)
}

pub fn insert(
    db_pool: &Pool<ConnectionManager<PgConnection>>,
    task_request: CreateTaskRequest,
) -> Result<i32, DBError>
{
    let connection = &mut db_pool
        .get()
        .map_err(adapt_database_error)?;

    let task_id: i32 = diesel::insert_into(tasks::table)
        .values(task_request)
        .returning(tasks::columns::task_id)
        .get_result(connection)
        .map_err(adapt_database_error)?;

    Ok(task_id)
}

pub fn update(
    db_pool: &Pool<ConnectionManager<PgConnection>>,
    task_id: i32,
    task_request: UpdateTaskRequest,
) -> Result<(), DBError> {
    let connection = &mut db_pool
        .get()
        .map_err(adapt_database_error)?;

    //None fields skipped by defaults!
    //https://diesel.rs/guides/all-about-updates.html
    diesel::update(tasks::table.find(task_id))
        .set(task_request)
        .execute(connection)
        .map_err(adapt_database_error)?;

    Ok(())
}

pub fn delete(
    db_pool: &Pool<ConnectionManager<PgConnection>>,
    task_id: i32,
) -> Result<(), DBError> {
    let connection = &mut db_pool
        .get()
        .map_err(adapt_database_error)?;

    diesel::delete(tasks::table.find(task_id))
        .execute(connection)
        .map_err(adapt_database_error)?;

    Ok(())
}