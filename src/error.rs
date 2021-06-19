use reqwest::StatusCode;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum VtError {
    #[error("You have exceeded one of your quotas (minute, daily or monthly). Daily quotas are reset every day at 00:00 UTC. You may have run out of disk space and/or number of files on your VirusTotal Monitor accounts.")]
    QuotaExceededError,
    #[error("Too many requests.")]
    TooManyRequestsError,
    #[error("Transient server error. Retry might work.")]
    TransientError,
    #[error("The operation took too long to complete.")]
    DeadlineExceededError,
    #[error("The request depended on another request and that request failed.")]
    FailedDependencyError,
    #[error("The resource already exists.")]
    AlreadyExistsError,
    #[error("The requested resource was not found.")]
    NotFoundError,
    #[error("You are not allowed to perform the requested operation.")]
    ForbiddenError,
    #[error("The provided API key is incorrect.")]
    WrongCredentialsError,
    #[error("The user accounts is not active. Make sure you properly activated your accounts by following the link sent to your email.")]
    UserNotActiveError,
    #[error(
        "The operation requires an authenticated user. Verify that you have provided your API key."
    )]
    AuthenticationRequiredError,
    #[error("Unsupported content search query.")]
    UnsupportedContentQueryError,
    #[error("Content search query is not selective enough.")]
    UnselectiveContentQueryError,
    #[error("The resource is not available yet, but will become available later.")]
    NotAvailableYet,
    #[error("Some of the provided arguments are incorrect.")]
    InvalidArgumentError,
    #[error("The API request is invalid or malformed. The message usually provides details about why the request is not valid.")]
    BadRequestError,
    #[error("Unknown error.")]
    Unknown,
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Json(serde_json::Error),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
}

impl From<serde_json::Error> for VtError {
    fn from(err: serde_json::Error) -> VtError {
        use serde_json::error::Category;
        match err.classify() {
            Category::Io => VtError::Io(err.into()),
            Category::Syntax | Category::Data | Category::Eof => VtError::Json(err),
        }
    }
}

/// Return the VtError based on the http status code
impl From<(StatusCode, String)> for VtError {
    fn from(pair: (StatusCode, String)) -> VtError {
        let (status_code, resp_text) = pair;
        match status_code {
            StatusCode::BAD_REQUEST => {
                if resp_text.contains("BadRequestError") {
                    VtError::BadRequestError
                } else if resp_text.contains("InvalidArgumentError") {
                    VtError::InvalidArgumentError
                } else if resp_text.contains("NotAvailableYet") {
                    VtError::NotAvailableYet
                } else if resp_text.contains("UnselectiveContentQueryError") {
                    VtError::UnselectiveContentQueryError
                } else {
                    VtError::UnsupportedContentQueryError
                }
            } // 400
            StatusCode::UNAUTHORIZED => {
                if resp_text.contains("AuthenticationRequiredError") {
                    VtError::AuthenticationRequiredError
                } else if resp_text.contains("UserNotActiveError") {
                    VtError::UserNotActiveError
                } else {
                    VtError::WrongCredentialsError
                }
            } // 401
            StatusCode::FORBIDDEN => VtError::ForbiddenError, // 403
            StatusCode::NOT_FOUND => VtError::NotFoundError,  // 404
            StatusCode::CONFLICT => VtError::AlreadyExistsError, // 409
            StatusCode::FAILED_DEPENDENCY => VtError::FailedDependencyError, // 424
            StatusCode::TOO_MANY_REQUESTS => {
                if resp_text.contains("QuotaExceededError") {
                    VtError::QuotaExceededError
                } else {
                    VtError::TooManyRequestsError
                }
            } // 429
            StatusCode::SERVICE_UNAVAILABLE => VtError::TransientError, // 503
            StatusCode::GATEWAY_TIMEOUT => VtError::DeadlineExceededError, // 504
            _ => VtError::Unknown,
        }
    }
}
