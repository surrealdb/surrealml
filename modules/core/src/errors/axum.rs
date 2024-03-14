//! Implements the `IntoResponse` trait for the `SurrealError` type for the `axum` web framework.
use axum::response::{IntoResponse, Response};
use axum::body::Body;
pub use crate::errors::error::{SurrealErrorStatus, SurrealError};


impl IntoResponse for SurrealError {
    
    /// Constructs a HTTP response for the error.
    /// 
    /// # Returns
    /// * `Response` - The HTTP response for the error.
    fn into_response(self) -> Response {
        let status_code = match self.status {
            SurrealErrorStatus::NotFound  => axum::http::StatusCode::NOT_FOUND,
            SurrealErrorStatus::Forbidden => axum::http::StatusCode::FORBIDDEN,
            SurrealErrorStatus::Unknown => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            SurrealErrorStatus::BadRequest => axum::http::StatusCode::BAD_REQUEST,
            SurrealErrorStatus::Conflict => axum::http::StatusCode::CONFLICT,
            SurrealErrorStatus::Unauthorized => axum::http::StatusCode::UNAUTHORIZED
        };
        axum::http::Response::builder()
            .status(status_code)
            .body(Body::new(self.message))
            .unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn test_into_response() {
        let error = SurrealError {
            message: "Test".to_string(),
            status: SurrealErrorStatus::NotFound
        };
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
