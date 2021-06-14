mod domain;
/// VirusTotal API v3
/// Clean & Simple interface to access the VirusTotal v3 REST api's
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
mod file;
mod ip;
mod url;

#[cfg(feature = "enterprise")]
mod enterprise;
#[cfg(feature = "enterprise")]
pub use self::enterprise::retrohunt::SubmitJobRoot;

pub mod error;
mod utils;

#[derive(Copy, Clone)]
pub struct VtClient<'a> {
    api_key: &'a str,
    endpoint: &'a str,
    user_agent: &'a str,
}

impl<'a> VtClient<'a> {
    pub fn new(api_key: &'a str) -> Self {
        //! Creates a new VirusTotal API Client to access the VirusTotal REST API v3
        //!
        //! ## Example usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt_client = VtClient::new("YOUR API KEY");
        //! ```
        VtClient {
            api_key,
            endpoint: "https://www.virustotal.com/api/v3",
            user_agent: "rust-client/vt3-rs+https://github.com/marirs/vt3-rs",
        }
    }

    /// Sets a new user-agent that from the default
    pub fn user_agent(&'a mut self, user_agent: &'a str) -> &'a mut VtClient {
        self.user_agent = user_agent;
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
