mod response;
use response::Root;

use crate::{utils::http_get_bz, VtClient, VtResult};

impl VtClient {
    pub fn file_feed(&self, time: &str) -> VtResult<Vec<Root>> {
        //! Get a file feed batch.
        //!
        //! ## Example Usage
        //! ```rust
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

    pub fn file_feed_behaviours(&self, time: &str) -> VtResult<Vec<Root>> {
        //! Get a file sandbox behaviour feed batch.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // A string in format YYYYMMDDhhmm
        //! vt.file_feed("202106131355");
        //! ```
        let url = format!("{}/feeds/file-behaviours/{}", self.endpoint, time);
        http_get_bz(&self.api_key, &self.user_agent, &url)
    }

    pub fn file_feed_behaviours_hourly(&self, time: &str) -> VtResult<Vec<Root>> {
        //! Hourly file feed sandbox behaviour feed batch.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // A string in format YYYYMMDDhh
        //! vt.file_feed("2021061313");
        //! ```
        let url = format!("{}/feeds/file-behaviours/hourly/{}", self.endpoint, time);
        http_get_bz(&self.api_key, &self.user_agent, &url)
    }

    pub fn file_feed_hourly(&self, time: &str) -> VtResult<Vec<Root>> {
        //! Hourly file feed packages.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // A string in format YYYYMMDDhh
        //! vt.file_feed("2021061313");
        //! ```
        let url = format!("{}/feeds/files/hourly/{}", self.endpoint, time);
        http_get_bz(&self.api_key, &self.user_agent, &url)
    }
}
