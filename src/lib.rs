/// VirusTotal API v3
/// Clean & Simple interface to access the VirusTotal v3 Public & Enterprise REST api's
/// ## Usage
/// ```toml
/// [dependencies]
/// vt3 = "0.1.0"
/// ```
/// ```rust
/// use vt3;
///
/// let vt_client = vt3::VtClient::new("YOUR API KEY");
/// ```
///
mod domain;
mod file;
mod ip;
mod url;

#[cfg(feature = "enterprise")]
mod enterprise;

#[cfg(feature = "enterprise")]
pub use self::enterprise::{livehunt::SubmitRulesetRoot, retrohunt::SubmitJobRoot};

static DEFAULT_USER_AGENT: &str = "rust-client/vt3-rs+https://github.com/marirs/vt3-rs";

pub mod error;
mod utils;

use error::VtError;
pub type VtResult<T> = Result<T, VtError>;

#[derive(Clone)]
pub struct VtClient {
    api_key: String,
    endpoint: String,
    user_agent: String,
}

impl VtClient {
    pub fn new(api_key: &str) -> Self {
        //! Creates a new VirusTotal API Client to access the VirusTotal REST API v3
        //!
        //! ## Example usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt_client = VtClient::new("YOUR API KEY");
        //! ```
        VtClient {
            api_key: api_key.into(),
            endpoint: "https://www.virustotal.com/api/v3".into(),
            user_agent: DEFAULT_USER_AGENT.into(),
        }
    }

    /// Sets a new user-agent that from the default
    pub fn user_agent(&mut self, user_agent: &str) -> &mut VtClient {
        self.user_agent = user_agent.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::VtClient;

    #[test]
    fn test_vtclient() {
        let vt_client = VtClient::new("someapikey");
        assert_eq!(vt_client.api_key, "someapikey");
        assert_eq!(vt_client.endpoint, "https://www.virustotal.com/api/v3")
    }
}
