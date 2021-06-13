pub mod error;
mod ip;
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
            user_agent: "rust-client/vt3-rs",
        }
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
