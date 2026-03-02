use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::{Map, Value};

pub enum AppError {
    InvalidToken,
    InvalidCredentials,
    Internal,
}

#[derive(Serialize)]
pub struct ResponseError {
    pub code: u16,
    pub message: String,
    #[serde(flatten)]
    pub data: Map<String, Value>
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                Json(ResponseError{
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                    message: "invalid authorization provided".into(),
                    data: Map::new(),
                })
            ).into_response(),
            Self::InvalidCredentials => (
                  StatusCode::UNAUTHORIZED,
                Json(ResponseError{
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                    message: "invalid credentials".into(),
                    data: Map::new(),
                })
            ).into_response(),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseError{
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "internal server error".into(),
                    data: Map::new()
                })
            ).into_response()
        }
    }
}
