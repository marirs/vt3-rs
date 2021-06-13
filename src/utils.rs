use crate::error::VtError;
use reqwest::{blocking::Client, StatusCode};

/// GET from a URL
pub(crate) fn http_get(api_key: &str, user_agent: &str, url: &str) -> Result<String, VtError> {
    let client = Client::builder().user_agent(user_agent).build().unwrap();
    let resp = client
        .get(url)
        .header("x-apikey", api_key)
        .header("Accept", "application/json")
        .send()
        .unwrap();
    let status = resp.status();
    let text = resp.text().unwrap();

    match status {
        StatusCode::OK => Ok(text), // 200
        _ => Err(error_from_status(status, &text)),
    }
}

/// POST to a URL
pub(crate) fn http_post(
    api_key: &str,
    user_agent: &str,
    url: &str,
    form_data: &[(&str, &str)],
) -> Result<String, VtError> {
    let client = Client::builder().user_agent(user_agent).build().unwrap();
    let resp = client
        .post(url)
        .header("x-apikey", api_key)
        .header("Accept", "application/json")
        .form(form_data)
        .send()
        .unwrap();
    let status = resp.status();
    let text = resp.text().unwrap();

    match status {
        StatusCode::OK => Ok(text), // 200
        _ => Err(error_from_status(status, &text)),
    }
}

/// Return the VtError based on the http status code
fn error_from_status(status_code: StatusCode, resp_text: &str) -> VtError {
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