mod response;
use response::Root;

use crate::{utils::http_get_bz, VtClient, VtResult};

impl VtClient {
    pub fn url_feed(&self, time: &str) -> VtResult<Vec<Root>> {
        //! Get a URL feed batch.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // A string in format YYYYMMDDhhmm
        //! vt.url_feed("202106131355");
        //! ```
        let url = format!("{}/feeds/urls/{}", self.endpoint, time);
        http_get_bz(&self.api_key, &self.user_agent, &url)
    }

    pub fn url_feed_hourly(&self, time: &str) -> VtResult<Vec<Root>> {
        //! Hourly file feed batch.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // A string in format YYYYMMDDhh
        //! vt.url_feed("2021061313");
        //! ```
        let url = format!("{}/feeds/urls/hourly/{}", self.endpoint, time);
        http_get_bz(&self.api_key, &self.user_agent, &url)
    }
}
