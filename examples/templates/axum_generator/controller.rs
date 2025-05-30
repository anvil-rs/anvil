// Generated Controller for {{ name }}
// This file is automatically generated

use axum::{
    http::StatusCode,
    extract::{Path, Json},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

pub async fn list_{{ snake_name }}s() -> impl IntoResponse {
    // Implementation for listing all {{ snake_name }}s
    Json(vec![])
}

pub async fn get_{{ snake_name }}(Path(id): Path<String>) -> impl IntoResponse {
    // Implementation for getting a specific {{ snake_name }}
    Json(())
}

pub async fn create_{{ snake_name }}(
    Json(payload): Json<Create{{ name }}Request>,
) -> impl IntoResponse {
    // Implementation for creating a new {{ snake_name }}
    (StatusCode::CREATED, Json(()))
}

pub async fn update_{{ snake_name }}(
    Path(id): Path<String>,
    Json(payload): Json<Update{{ name }}Request>,
) -> impl IntoResponse {
    // Implementation for updating a {{ snake_name }}
    Json(())
}

pub async fn delete_{{ snake_name }}(Path(id): Path<String>) -> impl IntoResponse {
    // Implementation for deleting a {{ snake_name }}
    StatusCode::NO_CONTENT
}

#[derive(Deserialize)]
pub struct Create{{ name }}Request {
    // Request fields
}

#[derive(Deserialize)]
pub struct Update{{ name }}Request {
    // Request fields
}
