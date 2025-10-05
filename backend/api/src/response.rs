use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub enum ApiResponse<T: IntoSuccess> {
    Success(T),
    Failure(ApiError),
}

pub trait IntoSuccess: Serialize + Sized {
    const STATUS_CODE: StatusCode;
    fn into_json(self) -> Json<Self> {
        Json(self)
    }
}

pub trait IntoFailure: std::error::Error + std::fmt::Debug + Sized {
    const STATUS_CODE: StatusCode;
    const CODE: String;
    fn into_api_error(self) -> ApiError {
        ApiError {
            status_code: Self::STATUS_CODE,
            code: Self::CODE,
            message: self.to_string(),
        }
    }
}

pub trait ApiResponseExt {
    type T: IntoSuccess;
    fn into_response(self) -> ApiResponse<Self::T>;
}

#[derive(Serialize)]
pub struct ApiError {
    #[serde(skip)]
    status_code: StatusCode,
    code: String,
    message: String,
}

impl<T: IntoSuccess> ApiResponseExt for Result<T, ApiError> {
    type T = T;
    fn into_response(self) -> ApiResponse<Self::T> {
        match self {
            Ok(ok) => ApiResponse::Success(ok),
            Err(err) => ApiResponse::Failure(err),
        }
    }
}

impl<T: IntoSuccess> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Success(ok) => (T::STATUS_CODE, ok.into_json()).into_response(),
            ApiResponse::Failure(err) => (err.status_code, Json(err)).into_response(),
        }
    }
}
