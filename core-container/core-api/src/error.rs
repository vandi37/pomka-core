// The Pomka Ecosystem Core Source Code
// Copyright (C) 2026 Lev (Leo) Kondukov (aka DiceBarrel, Barrel, Vandi)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::{Map, Number, Value};

pub enum AppError {
    // 500 1**
    Internal,

    // 401 2**
    InvalidToken,
    InvalidAdapterToken,
    InvalidCredentials,

    // 422 3**
    EmptyPatch,
    InvalidUserName(String),
    InvalidUserHandle(String),
    InsufficientFunds,

    // 409 4**
    AdminUsernameTaken(String),
    BotUsernameTaken(String),
    DailyRewardAlreadyClaimed(i64),

    // 404 5**
    AdminNotFound(i64),
    BotNotFound(i64),
    UserbotNotFound(i64),
    ExecutorNotFound(i64),
    UserNotFound(i64),
    UserNotFoundByHandle(String),

    // 403 6**
    ExecutorForbidden(i64),
    UserForbidden(i64),
}

impl AppError {
    pub fn into_u16(&self) -> u16 {
        match self {
            AppError::Internal => 100,
            AppError::InvalidToken => 200,
            AppError::InvalidAdapterToken => 201,
            AppError::InvalidCredentials => 202,
            AppError::EmptyPatch => 300,
            AppError::InvalidUserName(_) => 301,
            AppError::InvalidUserHandle(_) => 302,
            AppError::InsufficientFunds => 303,
            AppError::AdminUsernameTaken(_) => 400,
            AppError::BotUsernameTaken(_) => 401,
            AppError::DailyRewardAlreadyClaimed(_) => 402,
            AppError::AdminNotFound(_) => 500,
            AppError::BotNotFound(_) => 501,
            AppError::UserbotNotFound(_) => 502,
            AppError::ExecutorNotFound(_) => 503,
            AppError::UserNotFound(_) => 504,
            AppError::UserNotFoundByHandle(_) => 505,
            AppError::ExecutorForbidden(_) => 600,
            AppError::UserForbidden(_) => 601,
        }
    }
}

#[derive(Serialize)]
pub struct ResponseError {
    pub code: u16,
    pub message: String,
    pub app_code: u16,
    #[serde(flatten)]
    pub data: Map<String, Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let app_code = self.into_u16();
        match self {
            Self::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                Json(ResponseError {
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                    message: "invalid authorization provided".into(),
                    app_code,
                    data: Map::new(),
                }),
            )
                .into_response(),
            Self::InvalidAdapterToken => (
                StatusCode::UNAUTHORIZED,
                Json(ResponseError {
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                    message: "invalid adapter authorization provided".into(),
                    app_code,

                    data: Map::new(),
                }),
            )
                .into_response(),
            Self::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                Json(ResponseError {
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                    message: "invalid credentials".into(),
                    app_code,

                    data: Map::new(),
                }),
            )
                .into_response(),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseError {
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "internal server error".into(),
                    app_code,

                    data: Map::new(),
                }),
            )
                .into_response(),
            Self::EmptyPatch => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ResponseError {
                    code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    message: "empty patch".into(),
                    app_code,

                    data: Map::new(),
                }),
            )
                .into_response(),
            Self::AdminUsernameTaken(username) => (
                StatusCode::CONFLICT,
                Json(ResponseError {
                    code: StatusCode::CONFLICT.as_u16(),
                    message: "admin username is taken".into(),
                    app_code,

                    data: Map::from_iter([("username".to_string(), Value::String(username))]),
                }),
            )
                .into_response(),
            Self::AdminNotFound(id) => (
                StatusCode::NOT_FOUND,
                Json(ResponseError {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message: "admin not found".into(),
                    app_code,

                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))]),
                }),
            )
                .into_response(),
            Self::BotUsernameTaken(username) => (
                StatusCode::CONFLICT,
                Json(ResponseError {
                    code: StatusCode::CONFLICT.as_u16(),
                    message: "bot username is taken".into(),
                    app_code,

                    data: Map::from_iter([("username".to_string(), Value::String(username))]),
                }),
            )
                .into_response(),
            Self::BotNotFound(id) => (
                StatusCode::NOT_FOUND,
                Json(ResponseError {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message: "bot not found".into(),
                    app_code,

                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))]),
                }),
            )
                .into_response(),
            Self::UserbotNotFound(id) => (
                StatusCode::NOT_FOUND,
                Json(ResponseError {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message: "userbot not found".into(),
                    app_code,

                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))]),
                }),
            )
                .into_response(),
            Self::InvalidUserName(name) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ResponseError {
                    code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    message: "invalid user name".into(),
                    app_code,
                    data: Map::from_iter([("name".to_string(), Value::String(name))]),
                }),
            )
                .into_response(),
            Self::InvalidUserHandle(handle) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ResponseError {
                    code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    message: "invalid user handle".into(),
                    app_code,
                    data: Map::from_iter([("userhandle".to_string(), Value::String(handle))]),
                }),
            )
                .into_response(),
            Self::ExecutorNotFound(id) => (
                StatusCode::NOT_FOUND,
                Json(ResponseError {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message: "executor not found".into(),
                    app_code,
                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))]),
                }),
            )
                .into_response(),
            Self::UserNotFound(id) => (
                StatusCode::NOT_FOUND,
                Json(ResponseError {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message: "user not found".into(),
                    app_code,
                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))]),
                }),
            )
                .into_response(),
            Self::UserNotFoundByHandle(userhandle) => (
                StatusCode::NOT_FOUND,
                Json(ResponseError {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message: "user not found by handle".into(),
                    app_code,
                    data: Map::from_iter([("userhandle".to_string(), Value::String(userhandle))]),
                }),
            )
                .into_response(),
            Self::ExecutorForbidden(id) => (
                StatusCode::FORBIDDEN,
                Json(ResponseError {
                    code: StatusCode::FORBIDDEN.as_u16(),
                    message: "forbidden for executor".into(),
                    app_code,
                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))]),
                }),
            )
                .into_response(),
            Self::UserForbidden(id) => (
                StatusCode::FORBIDDEN,
                Json(ResponseError {
                    code: StatusCode::FORBIDDEN.as_u16(),
                    message: "forbidden for user".into(),
                    app_code,
                    data: Map::from_iter([("id".to_string(), Value::Number(Number::from(id)))]),
                }),
            )
                .into_response(),
            Self::DailyRewardAlreadyClaimed(next) => (
                StatusCode::CONFLICT,
                Json(ResponseError {
                    code: StatusCode::CONFLICT.as_u16(),
                    message: "daily reward already claimed".into(),
                    app_code,
                    data: Map::from_iter([(
                        "next_available_at".to_string(),
                        Value::Number(Number::from(next)),
                    )]),
                }),
            )
                .into_response(),
            Self::InsufficientFunds => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ResponseError {
                    code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    message: "daily reward already claimed".into(),
                    app_code,
                    data: Map::new(),
                }),
            )
                .into_response(),
        }
    }
}
