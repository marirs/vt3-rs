#[derive(Debug)]
pub enum VtError {
    QuotaExceededError,
    TooManyRequestsError,
    TransientError,
    DeadlineExceededError,
    FailedDependencyError,
    AlreadyExistsError,
    NotFoundError,
    ForbiddenError,
    WrongCredentialsError,
    UserNotActiveError,
    AuthenticationRequiredError,
    UnsupportedContentQueryError,
    UnselectiveContentQueryError,
    NotAvailableYet,
    InvalidArgumentError,
    BadRequestError,
    Unknown,
    Json(serde_json::Error),
    Io(std::io::Error),
    Reqwest(reqwest::Error),
}

impl std::error::Error for VtError {}

impl VtError {
    fn as_str(&self) -> &str {
        match *self {
            VtError::QuotaExceededError => "You have exceeded one of your quotas (minute, daily or monthly). Daily quotas are reset every day at 00:00 UTC. You may have run out of disk space and/or number of files on your VirusTotal Monitor account.",
            VtError::TooManyRequestsError => "Too many requests.",
            VtError::TransientError => "Transient server error. Retry might work.",
            VtError::DeadlineExceededError => "The operation took too long to complete.",
            VtError::FailedDependencyError => "The request depended on another request and that request failed.",
            VtError::AlreadyExistsError => "The resource already exists.",
            VtError::NotFoundError => "The requested resource was not found.",
            VtError::ForbiddenError => "You are not allowed to perform the requested operation.",
            VtError::WrongCredentialsError => "The provided API key is incorrect.",
            VtError::UserNotActiveError => "The user account is not active. Make sure you properly activated your account by following the link sent to your email.",
            VtError::AuthenticationRequiredError => "The operation requires an authenticated user. Verify that you have provided your API key.",
            VtError::UnsupportedContentQueryError => "Unsupported content search query.",
            VtError::UnselectiveContentQueryError => "Content search query is not selective enough.",
            VtError::NotAvailableYet => "The resource is not available yet, but will become available later.",
            VtError::InvalidArgumentError => "Some of the provided arguments are incorrect.",
            VtError::BadRequestError => "The API request is invalid or malformed. The message usually provides details about why the request is not valid.",
            VtError::Unknown => "Unknown error.",
            _ => "Custom",
        }
    }
}

impl std::fmt::Display for VtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            VtError::QuotaExceededError => f.write_str(VtError::QuotaExceededError.as_str()),
            VtError::TooManyRequestsError => f.write_str(VtError::TooManyRequestsError.as_str()),
            VtError::TransientError => f.write_str(VtError::TransientError.as_str()),
            VtError::DeadlineExceededError => f.write_str(VtError::DeadlineExceededError.as_str()),
            VtError::FailedDependencyError => f.write_str(VtError::FailedDependencyError.as_str()),
            VtError::AlreadyExistsError => f.write_str(VtError::AlreadyExistsError.as_str()),
            VtError::NotFoundError => f.write_str(VtError::NotFoundError.as_str()),
            VtError::ForbiddenError => f.write_str(VtError::ForbiddenError.as_str()),
            VtError::WrongCredentialsError => f.write_str(VtError::WrongCredentialsError.as_str()),
            VtError::UserNotActiveError => f.write_str(VtError::UserNotActiveError.as_str()),
            VtError::AuthenticationRequiredError => {
                f.write_str(VtError::AuthenticationRequiredError.as_str())
            }
            VtError::UnsupportedContentQueryError => {
                f.write_str(VtError::UnsupportedContentQueryError.as_str())
            }
            VtError::UnselectiveContentQueryError => {
                f.write_str(VtError::UnselectiveContentQueryError.as_str())
            }
            VtError::NotAvailableYet => f.write_str(VtError::NotAvailableYet.as_str()),
            VtError::InvalidArgumentError => f.write_str(VtError::InvalidArgumentError.as_str()),
            VtError::BadRequestError => f.write_str(VtError::BadRequestError.as_str()),
            VtError::Unknown => f.write_str(VtError::Unknown.as_str()),
            VtError::Io(ref err) => f.write_str(err.to_string().as_str()),
            VtError::Json(ref err) => f.write_str(err.to_string().as_str()),
            VtError::Reqwest(ref err) => f.write_str(err.to_string().as_str()),
        }
    }
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

impl From<reqwest::Error> for VtError {
    fn from(err: reqwest::Error) -> VtError {
        VtError::Reqwest(err)
    }
}
