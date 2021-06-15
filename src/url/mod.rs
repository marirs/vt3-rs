mod response;
use response::{Root, ScanRoot};

use crate::{
    utils::{http_get, http_post},
    VtClient, VtResult,
};

impl VtClient {
    pub fn url_scan(&self, resource_url: &str) -> VtResult<ScanRoot> {
        //! Scan an URL
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! let url = "https://example.com";
        //! println!("{:?}", vt.url_scan(url));
        //! ```
        let url = format!("{}/urls", &self.endpoint);
        let form_data = &[("url", resource_url)];
        http_post(&self.api_key, &self.user_agent, &url, form_data)
    }

    pub fn url_rescan(&self, resource_id: &str) -> VtResult<ScanRoot> {
        //! Re-analyse/Re-Scan an URL
        //!
        //! ## Example Usage
        //!
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! let url = "https://example.com";
        //! println!("{:?}", vt.url_scan(url));
        //! ```
        let url_id = resource_id.split('-').nth(1).unwrap_or(resource_id);
        let url = format!("{}/urls/{}/analyse", &self.endpoint, url_id);
        let form_data = &[("url", resource_id)];
        http_post(&self.api_key, &self.user_agent, &url, form_data)
    }

    pub fn url_info(&self, resource_url: &str) -> VtResult<Root> {
        //! Get the report of a given Url
        //!
        //! ## Example Usage
        //!
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! let resource = "https://www.example.com";
        //! println!("{:?}", vt.url_info(resource));
        //! ```
        let url = format!(
            "{}/urls/{}",
            &self.endpoint,
            base64::encode(resource_url).replace('=', "")
        );
        http_get(&self.api_key, &self.user_agent, &url)
    }

    pub fn url_info_by_id(&self, resource_id: &str) -> VtResult<Root> {
        //! Get the report of a given Url by its resource id. Generally you can first
        //! submit a url for scanning, and then, get the resource_id (`data.id`)
        //! and then url_info_by_id(data.id)
        //!
        //! ## Example Usage
        //!
        //! ```ignore
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! let resource = "https://www.example.com";
        //! let resource_id = vt.url_scan(resource).unwrap();
        //! println!("{:?}", vt.url_info_by_id(&resource_id.data.id))
        //! ```
        let url_id = resource_id.split('-').nth(1).unwrap_or(resource_id);
        let url = format!("{}/urls/{}", &self.endpoint, url_id);
        http_get(&self.api_key, &self.user_agent, &url)
    }
}
