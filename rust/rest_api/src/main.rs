mod error;
mod models;
mod routes_posts;
mod routes_users;
mod store;

use axum::routing::get;
use axum::{Json, Router};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use store::Store;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(Mutex::new(Store::new()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let user_routes = Router::new()
        .route(
            "/",
            get(routes_users::list_users).post(routes_users::create_user),
        )
        .route(
            "/{user_id}",
            get(routes_users::get_user)
                .patch(routes_users::update_user)
                .delete(routes_users::delete_user),
        );

    let post_routes = Router::new()
        .route(
            "/",
            get(routes_posts::list_posts).post(routes_posts::create_post),
        )
        .route(
            "/{post_id}",
            get(routes_posts::get_post)
                .patch(routes_posts::update_post)
                .delete(routes_posts::delete_post),
        );

    let app = Router::new()
        .route("/health", get(health))
        .nest("/api/v1/users", user_routes)
        .nest("/api/v1/posts", post_routes)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok"}))
}
