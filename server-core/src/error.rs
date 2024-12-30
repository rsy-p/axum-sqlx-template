use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::Value;
use thiserror::Error;

/// 错误类型
#[derive(Debug, Error)]
enum MyError {
    #[error("No Data")]
    NoData,
}

/// 自定义错误
pub struct AppError(anyhow::Error);

impl AppError {
    pub fn new(msg: &str) -> AppError {
        AppError(anyhow::anyhow!("{}", msg))
    }

    pub fn no_data() -> AppError {
        AppError(anyhow::Error::from(MyError::NoData))
    }
}

/// axum返回结果
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error = self.0;
        let mut result = (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Value::String(format!("server error: {:?}", error))),
        );
        if let Some(sqlx::Error::RowNotFound) = error.downcast_ref() {
            result = (StatusCode::OK, Json(Value::Null));
        }
        if let Some(MyError::NoData) = error.downcast_ref() {
            result = (StatusCode::OK, Json(Value::Null));
        }
        result.into_response()
    }
}

/// anyhow::Error ==> AppError
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
