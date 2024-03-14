//! Implements the `ResponseError` trait for the `SurrealError` type for the `actix_web` web framework.
use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
pub use crate::errors::error::{SurrealErrorStatus, SurrealError};


impl ResponseError for SurrealError {
    
    /// Yields the status code for the error.
    /// 
    /// # Returns
    /// * `StatusCode` - The status code for the error.
    fn status_code(&self) -> StatusCode {
        match self.status {
            SurrealErrorStatus::NotFound  => StatusCode::NOT_FOUND,
            SurrealErrorStatus::Forbidden => StatusCode::FORBIDDEN,
            SurrealErrorStatus::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            SurrealErrorStatus::BadRequest => StatusCode::BAD_REQUEST,
            SurrealErrorStatus::Conflict => StatusCode::CONFLICT,
            SurrealErrorStatus::Unauthorized => StatusCode::UNAUTHORIZED
        }
    }

    /// Constructs a HTTP response for the error.
    /// 
    /// # Returns
    /// * `HttpResponse` - The HTTP response for the error.
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(self.message.clone())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;

    #[test]
    fn test_status_code() {
        let error = SurrealError {
            message: "Test".to_string(),
            status: SurrealErrorStatus::NotFound
        };
        assert_eq!(error.status_code(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_error_response() {
        let error = SurrealError {
            message: "Test".to_string(),
            status: SurrealErrorStatus::NotFound
        };
        let response = error.error_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
