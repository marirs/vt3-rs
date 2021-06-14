mod response;
use response::Root;

use crate::{error::VtError, utils::http_get, VtClient};

impl<'a> VtClient<'a> {
    pub fn ip_info(self, ip_address: &str) -> Result<Root, VtError> {
        //! Get the report of a given IP Address
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! println!("{:?}", vt.ip_info("192.168.2.1"));
        //! ```
        let url = format!("{}/ip_addresses/{}", self.endpoint, ip_address);
        let text = match http_get(self.api_key, self.user_agent, &url) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        let res: Root = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(VtError::Json(e)),
        };

        Ok(res)
    }
}
