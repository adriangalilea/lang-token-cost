use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use crate::error::AppError;
use crate::models::*;
use crate::store::SharedState;

#[derive(serde::Deserialize)]
pub struct CreatePostQuery {
    pub author_id: u64,
}

pub async fn list_posts(
    State(state): State<SharedState>,
    Query(params): Query<PostListParams>,
) -> Result<Json<PaginatedResponse<Post>>, AppError> {
    let store = state.lock().unwrap();
    Ok(Json(store.list_posts(&params)))
}

pub async fn create_post(
    State(state): State<SharedState>,
    Query(query): Query<CreatePostQuery>,
    Json(body): Json<PostCreate>,
) -> Result<(StatusCode, Json<Post>), AppError> {
    let mut store = state.lock().unwrap();
    let post = store.create_post(query.author_id, body)?;
    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn get_post(
    State(state): State<SharedState>,
    Path(post_id): Path<u64>,
) -> Result<Json<Post>, AppError> {
    let store = state.lock().unwrap();
    let post = store.get_post(post_id)?;
    Ok(Json(post.clone()))
}

pub async fn update_post(
    State(state): State<SharedState>,
    Path(post_id): Path<u64>,
    Json(body): Json<PostUpdate>,
) -> Result<Json<Post>, AppError> {
    let mut store = state.lock().unwrap();
    let post = store.update_post(post_id, body)?;
    Ok(Json(post))
}

pub async fn delete_post(
    State(state): State<SharedState>,
    Path(post_id): Path<u64>,
) -> Result<StatusCode, AppError> {
    let mut store = state.lock().unwrap();
    store.delete_post(post_id)?;
    Ok(StatusCode::NO_CONTENT)
}
