use crate::errors::app_error::AppError;
use crate::models::todo::{CreateTodo, UpdateTodo};
use crate::models::todo_repo::DynTodoRepo;

use axum::{
    extract::{Extension, Path},
    http::{StatusCode, HeaderMap},
    Json,
    response::IntoResponse,
};
use uuid::Uuid;

pub async fn create_todo(
    Json(create_todo): Json<CreateTodo>,
    Extension(todo_repo): Extension<DynTodoRepo>,
) -> Result<impl IntoResponse, AppError> {
    let id = todo_repo.create_todo(create_todo).await?;
    let mut headers = HeaderMap::new();
    headers.insert("Location", format!("{}{}", "/todos/", id).parse().unwrap());
    Ok((StatusCode::CREATED, headers))
}

pub async fn delete_todo(
    Path(id): Path<Uuid>,
    Extension(todo_repo): Extension<DynTodoRepo>,
) -> Result<StatusCode, AppError> {
    let _ = todo_repo.delete_todo(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_todo(
    Path(id): Path<Uuid>,
    Json(update_todo): Json<UpdateTodo>,
    Extension(todo_repo): Extension<DynTodoRepo>,
) -> Result<StatusCode, AppError> {
    let _ = todo_repo.update_todo(id, update_todo).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_todo(
    Path(id): Path<Uuid>,
    Extension(todo_repo): Extension<DynTodoRepo>,
) -> Result<impl IntoResponse, AppError> {
    let todo_str = todo_repo.get_todo(id).await?;
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    Ok((StatusCode::OK, headers, todo_str))
}
