use axum::extract::State;
use axum::Json;
use server_core::{AppError, AppState, Result};
use server_database::UserMapper;
use server_model::User;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub async fn find_by_id(State(state): State<Arc<AppState>>) -> Result<Json<User>> {
    let redis = state.redis_conn().await?;
    let mut tx = state.pg_tx().await?;
    server_cache::set(redis).await?;
    let user = UserMapper::query_by_id(
        Uuid::from_str("3551ba72-9fb6-0c72-796e-21376c8146e7")?,
        &mut *tx,
    )
    .await?
    .ok_or(AppError::no_data())?;
    Ok(Json(user))
}
