use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use crate::error::AppError;
use crate::models::*;
use crate::store::SharedState;

pub async fn list_users(
    State(state): State<SharedState>,
    Query(params): Query<UserListParams>,
) -> Result<Json<PaginatedResponse<User>>, AppError> {
    let store = state.lock().unwrap();
    Ok(Json(store.list_users(&params)))
}

pub async fn create_user(
    State(state): State<SharedState>,
    Json(body): Json<UserCreate>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let mut store = state.lock().unwrap();
    let user = store.create_user(body);
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user(
    State(state): State<SharedState>,
    Path(user_id): Path<u64>,
) -> Result<Json<User>, AppError> {
    let store = state.lock().unwrap();
    let user = store.get_user(user_id)?;
    Ok(Json(user.clone()))
}

pub async fn update_user(
    State(state): State<SharedState>,
    Path(user_id): Path<u64>,
    Json(body): Json<UserUpdate>,
) -> Result<Json<User>, AppError> {
    let mut store = state.lock().unwrap();
    let user = store.update_user(user_id, body)?;
    Ok(Json(user))
}

pub async fn delete_user(
    State(state): State<SharedState>,
    Path(user_id): Path<u64>,
) -> Result<StatusCode, AppError> {
    let mut store = state.lock().unwrap();
    store.delete_user(user_id)?;
    Ok(StatusCode::NO_CONTENT)
}
