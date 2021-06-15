mod response;
use response::Root;

use crate::{utils::http_get_bz, VtClient, VtResult};

impl VtClient {
    pub fn file_feed(self, time: &str) -> VtResult<Vec<Root>> {
        //! Get a file feed batch.
        //!
        //! ## Example Usage
        //! ```
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // A string in format YYYYMMDDhhmm
        //! vt.file_feed("202106131355");
        //! ```
        let url = format!("{}/feeds/files/{}", self.endpoint, time);
        http_get_bz(&self.api_key, &self.user_agent, &url)
    }
}
