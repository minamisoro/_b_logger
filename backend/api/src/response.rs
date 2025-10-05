use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

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
    const CODE: &str;
    fn into_api_error(self) -> ApiError {
        ApiError {
            status_code: Self::STATUS_CODE,
            code: Self::CODE.to_string(),
            message: self.to_string(),
        }
    }
}

pub trait ApiResponseExt {
    type T: IntoSuccess;
    fn into_response(self) -> ApiResponse<Self::T>;
}

#[derive(Serialize, ToSchema)]
pub struct ApiError {
    #[serde(skip)]
    #[schema(ignore)]
    status_code: StatusCode,
    /// Error code identifier
    code: String,
    /// Human-readable error message
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
            ApiResponse::Failure(err) => err.into_response(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status_code, Json(self)).into_response()
    }
}
