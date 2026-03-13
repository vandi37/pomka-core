use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::{Map, Number, Value};

pub enum AppError {
    InvalidToken,
    InvalidAdapterToken,
    InvalidCredentials,
    Internal,
    EmptyPatch,
    AdminUsernameTaken(String),
    AdminNotFound(i64),
    BotUsernameTaken(String),
    BotNotFound(i64),
    UserbotNotFound(i64),
    InvalidUserName(String),
    ExecutorNotFound(i64),
    UserNotFound(i64),
    ExecutorForbidden(i64),
    UserForbidden(i64),
}

#[derive(Serialize)]
pub struct ResponseError {
    pub code: u16,
    pub message: String,
    #[serde(flatten)]
    pub data: Map<String, Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                Json(ResponseError {
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                    message: "invalid authorization provided".into(),
                    data: Map::new(),
                }),
            )
                .into_response(),
            Self::InvalidAdapterToken => (
                StatusCode::UNAUTHORIZED,
                Json(ResponseError {
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                    message: "invalid adapter authorization provided".into(),
                    data: Map::new(),
                }),
            )
                .into_response(),
            Self::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                Json(ResponseError {
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                    message: "invalid credentials".into(),
                    data: Map::new(),
                }),
            )
                .into_response(),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseError {
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "internal server error".into(),
                    data: Map::new(),
                }),
            )
                .into_response(),
            Self::EmptyPatch => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ResponseError {
                    code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    message: "empty patch".into(),
                    data: Map::new(),
                }),
            )
                .into_response(),
            Self::AdminUsernameTaken(username) => (
                StatusCode::CONFLICT,
                Json(ResponseError {
                    code: StatusCode::CONFLICT.as_u16(),
                    message: "admin username is taken".into(),
                    data: Map::from_iter([("username".to_string(), Value::String(username))]),
                }),
            )
                .into_response(),
            Self::AdminNotFound(id) => (
                StatusCode::NOT_FOUND,
                Json(ResponseError {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message: "admin not found".into(),
                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))]),
                }),
            )
                .into_response(),
            Self::BotUsernameTaken(username) => (
                StatusCode::CONFLICT,
                Json(ResponseError {
                    code: StatusCode::CONFLICT.as_u16(),
                    message: "bot username is taken".into(),
                    data: Map::from_iter([("username".to_string(), Value::String(username))]),
                }),
            )
                .into_response(),
            Self::BotNotFound(id) => (
                StatusCode::NOT_FOUND,
                Json(ResponseError {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message: "bot not found".into(),
                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))]),
                }),
            )
                .into_response(),
            Self::UserbotNotFound(id) => (
                StatusCode::NOT_FOUND,
                Json(ResponseError {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message: "userbot not found".into(),
                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))]),
                }),
            )
                .into_response(),
            Self::InvalidUserName(name) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ResponseError {
                    code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    message: "invalid user name".into(),
                    data: Map::from_iter([("name".to_string(), Value::String(name))]),
                }),
            ).into_response(),
             Self::ExecutorNotFound(id) => (
                StatusCode::NOT_FOUND,
                Json(ResponseError {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message: "executor not found".into(),
                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))]),
                }),
            )
                .into_response(),
             Self::UserNotFound(id) => (
                StatusCode::NOT_FOUND,
                Json(ResponseError {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message: "user not found".into(),
                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))]),
                }),
            )
                .into_response(),
            Self::ExecutorForbidden(id) => (
                StatusCode::FORBIDDEN,
                Json(ResponseError {
                    code: StatusCode::FORBIDDEN.as_u16(),
                    message:"forbidden for executor".into(),
                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))])
                })
            )
                .into_response(),
            Self::UserForbidden(id) => (
                StatusCode::FORBIDDEN,
                Json(ResponseError {
                    code: StatusCode::FORBIDDEN.as_u16(),
                    message:"forbidden for user".into(),
                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))])
                })
            )
                .into_response()
        }
    }
}
