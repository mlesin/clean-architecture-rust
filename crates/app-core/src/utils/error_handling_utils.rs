use app_domain::error::AppError;
use std::error::Error;

pub struct ErrorHandlingUtils {}

impl ErrorHandlingUtils {
    pub fn business_error(error_message: &str, error: Option<Box<dyn Error>>) -> AppError {
        ErrorHandlingUtils::log_error(error_message, &error);
        AppError {
            code: 400,
            message: String::from(error_message),
            error,
        }
    }
    pub fn unauthorized_error() -> AppError {
        let unauthorized_message = "Error: not authenticated or token expired";
        ErrorHandlingUtils::log_error(unauthorized_message, &None);
        AppError {
            code: 401,
            message: String::from(unauthorized_message),
            error: None,
        }
    }
    pub fn forbidden_error() -> AppError {
        let forbdden_message = "Error: resource not allowed";
        ErrorHandlingUtils::log_error(forbdden_message, &None);
        AppError {
            code: 403,
            message: String::from(forbdden_message),
            error: None,
        }
    }

    fn log_error(message: &str, err: &Option<Box<dyn Error>>) {
        println!("Error: {}", message);
        if let Some(error) = err {
            println!("Stack: {}", error.to_string());
        }
    }
}
