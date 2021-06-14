mod response;
use response::SearchJobRoot;
pub use response::SubmitJobRoot;

use crate::{
    error::VtError,
    utils::{http_body_post, http_delete, http_get, http_get_with_params, http_post},
    VtClient,
};

impl<'a> VtClient<'a> {
    pub fn get_jobs(
        self,
        limit: Option<&str>,
        filter: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<SearchJobRoot, VtError> {
        //! Get all or limited RetroHunt jobs
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.get_jobs(Some("10"), None, None);
        //! ```
        let url = format!("{}/intelligence/retrohunt_jobs", self.endpoint);
        let mut query_params: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query_params.push(("limit", l))
        }
        if let Some(f) = filter {
            query_params.push(("filter", f))
        }
        if let Some(c) = cursor {
            query_params.push(("cursor", c))
        }

        let text = match http_get_with_params(
            self.api_key,
            self.user_agent,
            &url,
            &query_params.as_slice(),
        ) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        let res: SearchJobRoot = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(VtError::Json(e)),
        };

        Ok(res)
    }

    pub fn get_job(self, job_id: &str) -> Result<SearchJobRoot, VtError> {
        //! Get RetroHunt job by ID
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.get_job("1");
        //! ```
        let url = format!("{}/intelligence/retrohunt_jobs/{}", self.endpoint, job_id);

        let text = match http_get(self.api_key, self.user_agent, &url) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        let res: SearchJobRoot = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(VtError::Json(e)),
        };

        Ok(res)
    }

    pub fn create_job(self, data: &SubmitJobRoot) -> Result<SubmitJobRoot, VtError> {
        //! Create/Submit a RetroHunt job
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::{VtClient, SubmitJobRoot};
        //!
        //! let vt = VtClient::new("Your API Key");
        //! let mut data = SubmitJobRoot::default();
        //! vt.create_job(&data);
        //! ```
        let url = format!("{}/intelligence/retrohunt_jobs", self.endpoint);
        let js = serde_json::to_string(data).unwrap();
        let text = match http_body_post(self.api_key, self.user_agent, &url, js) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        let res: SubmitJobRoot = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(VtError::Json(e)),
        };

        Ok(res)
    }

    pub fn delete_job(self, job_id: &str) -> Result<String, VtError> {
        //! Delete RetroHunt job
        //!
        //! # Example
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.delete_job("1");
        //! ```
        let url = format!("{}/intelligence/retrohunt_jobs/{}", self.endpoint, job_id);
        let text = match http_delete(self.api_key, self.user_agent, &url) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        Ok(text)
    }

    pub fn abort_job(self, job_id: &str) -> Result<String, VtError> {
        //! Abort a RetroHunt job
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.abort_job("1");
        //! ```
        let url = format!(
            "{}/intelligence/retrohunt_jobs/{}/abort",
            self.endpoint, job_id
        );
        let form_data = &[("id", job_id)];

        let text = match http_post(self.api_key, self.user_agent, &url, form_data) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        Ok(text)
    }
}
