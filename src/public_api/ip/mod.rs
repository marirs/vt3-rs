mod response;
use response::Root;

use crate::{utils::http_get, VtClient, VtResult};

impl VtClient {
    pub fn ip_info(&self, ip_address: &str) -> VtResult<Root> {
        //! Get the report of a given IP Address
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! println!("{:?}", vt.ip_info("192.168.2.1"));
        //! ```
        let url = format!("{}/ip_addresses/{}", &self.endpoint, ip_address);
        http_get(&self.api_key, &self.user_agent, &url)
    }
}
