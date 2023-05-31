use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use app_core::usecases::UseCaseError;
use derive_more::Display;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct PresenterError {
    pub code: u16,
    pub error: String,
    pub message: String,
}

#[derive(Error, Debug, Display)]
#[display(fmt = "{:?}", error)]
pub struct ErrorReponse {
    status_code: StatusCode,
    error: String,
}

impl ResponseError for ErrorReponse {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = PresenterError {
            code: status_code.as_u16(),
            message: status_code.to_string(),
            error: self.error.clone(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

impl From<UseCaseError> for ErrorReponse {
    fn from(value: UseCaseError) -> Self {
        match value {
            UseCaseError::Repository(e) => ErrorReponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: e,
            },
            UseCaseError::Business(e) => ErrorReponse {
                status_code: StatusCode::BAD_REQUEST,
                error: e,
            },
            UseCaseError::Unauthorized(e) => ErrorReponse {
                status_code: StatusCode::UNAUTHORIZED,
                error: e,
            },
            UseCaseError::Forbidden(e) => ErrorReponse {
                status_code: StatusCode::FORBIDDEN,
                error: e,
            },
        }
    }
}
