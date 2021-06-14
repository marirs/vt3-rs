mod response;
use response::SearchJobRoot;
pub use response::SubmitJobRoot;

use crate::utils::http_post;
use crate::{
    utils::{http_body_post, http_delete, http_get, http_get_with_params},
    VtClient, VtResult,
};

impl VtClient {
    pub fn get_jobs(
        &self,
        limit: Option<&str>,
        filter: Option<&str>,
        cursor: Option<&str>,
    ) -> VtResult<SearchJobRoot> {
        //! Get all or limited RetroHunt jobs
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.get_jobs(Some("10"), None, None);
        //! ```
        let url = format!("{}/intelligence/retrohunt_jobs", &self.endpoint);
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

        http_get_with_params(
            &self.api_key,
            &self.user_agent,
            &url,
            &query_params.as_slice(),
        )
    }

    pub fn get_job_by_id(&self, job_id: i32) -> VtResult<SearchJobRoot> {
        //! Get RetroHunt job by ID
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.get_job_by_id(1);
        //! ```
        let url = format!("{}/intelligence/retrohunt_jobs/{}", &self.endpoint, job_id);
        http_get(&self.api_key, &self.user_agent, &url)
    }

    pub fn create_job(&self, data: &SubmitJobRoot) -> VtResult<SubmitJobRoot> {
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
        let url = format!("{}/intelligence/retrohunt_jobs", &self.endpoint);
        http_body_post(&self.api_key, &self.user_agent, &url, data)
    }

    pub fn delete_job(&self, job_id: i32) -> VtResult<String> {
        //! Delete RetroHunt job
        //!
        //! # Example
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.delete_job(1);
        //! ```
        let url = format!("{}/intelligence/retrohunt_jobs/{}", &self.endpoint, job_id);
        http_delete(&self.api_key, &self.user_agent, &url)
    }

    pub fn abort_job(&self, job_id: i32) -> VtResult<String> {
        //! Abort a RetroHunt job
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.abort_job(1);
        //! ```
        let job_id = job_id.to_string();
        let url = format!(
            "{}/intelligence/retrohunt_jobs/{}/abort",
            &self.endpoint, job_id
        );
        let form_data = &[("id", job_id.as_str())];
        http_post(&self.api_key, &self.user_agent, &url, form_data)
    }
}
