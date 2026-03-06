use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::{Map, Number, Value};

pub enum AppError {
    InvalidToken,
    InvalidCredentials,
    Internal,
    AdminUsernameTaken(String),
    AminNotFound(i64),
    BotNotFound(i64),
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
            ).into_response(),
            Self::AdminUsernameTaken(username) => (
                StatusCode::CONFLICT,
                Json(ResponseError{
                    code: StatusCode::CONFLICT.as_u16(),
                    message: "admin username is taken".into(),
                    data: Map::from_iter([
                        ("username".to_string(), Value::String(username))
                    ])
                })
            ).into_response(),
            Self::AminNotFound(id) => (
                StatusCode::NOT_FOUND,
                Json(ResponseError{
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message:"admin not found".into(),
                    data: Map::from_iter([
                        ("id".to_string(), Value::Number(Number::from(id)))
                    ]),
                })
            ).into_response(),
             Self::BotNotFound(id) => (
                StatusCode::NOT_FOUND,
                Json(ResponseError{
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message:"bot not found".into(),
                    data: Map::from_iter([
                        ("id".to_string(), Value::Number(Number::from(id)))
                    ]),
                })
            ).into_response()
        }
    }
}
