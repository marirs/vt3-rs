mod response;
use response::Root;

use crate::{utils::http_get, VtClient, VtResult};

impl VtClient {
    pub fn domain_info(&self, domain: &str) -> VtResult<Root> {
        //! Get the report of a given Domain
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! println!("{:?}", vt.domain_info("google.com"))
        //! ```
        let url = format!("{}/domains/{}", &self.endpoint, domain);
        http_get(&self.api_key, &self.user_agent, &url)
    }
}
