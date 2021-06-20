mod response;
use response::{ApiUsage, GroupMembers, GroupRoot, OverallQuotaRoot, UserRoot};

use crate::{
    utils::{http_delete, http_get, http_get_with_params},
    VtClient, VtResult,
};

impl VtClient {
    pub fn user_info(&self, id: &str) -> VtResult<UserRoot> {
        //! Retrieve user information.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.user_info("user_id");
        //! ```
        let url = format!("{}/users/{}", self.endpoint, id);
        http_get(&self.api_key, &self.user_agent, &url)
    }

    pub fn delete_user(&self, id: &str) -> VtResult<UserRoot> {
        //! Delete a user.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.delete_user("user_id");
        //! ```
        let url = format!("{}/users/{}", self.endpoint, id);
        http_delete(&self.api_key, &self.user_agent, &url)
    }

    pub fn api_usage(
        &self,
        id: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> VtResult<ApiUsage> {
        //! Retrieve user's API usage.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // start_date & end_date fomrat is: YYYYMMDD
        //! vt.api_usage("user_id", None, None);
        //! ```
        let url = format!("{}/users/{}/api_usage", self.endpoint, id);
        let mut query_params: Vec<(&str, &str)> = Vec::new();
        if let Some(s) = start_date {
            query_params.push(("start_date", s))
        }
        if let Some(e) = end_date {
            query_params.push(("end_date", e))
        }
        http_get_with_params(
            &self.api_key,
            &self.user_agent,
            &url,
            &query_params.as_slice(),
        )
    }

    pub fn overall_quotas(&self, id: &str) -> VtResult<OverallQuotaRoot> {
        //! User's overall quotas.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.overall_quotas("user_id");
        //! ```
        let url = format!("{}/users/{}/overall_quotas", self.endpoint, id);
        http_get(&self.api_key, &self.user_agent, &url)
    }

    pub fn group_info(&self, id: &str) -> VtResult<GroupRoot> {
        //! Retrieve group information.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.overall_quotas("user_id");
        //! ```
        let url = format!("{}/groups/{}", self.endpoint, id);
        http_get(&self.api_key, &self.user_agent, &url)
    }

    pub fn group_members(&self, id: &str) -> VtResult<GroupMembers> {
        //! Retrieve group members.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.group_members("user_id");
        //! ```
        let url = format!("{}/groups/{}/relationships/users", self.endpoint, id);
        http_get(&self.api_key, &self.user_agent, &url)
    }

    pub fn group_api_usage(
        &self,
        id: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> VtResult<ApiUsage> {
        //! Retrieve group's API usage.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // start_date & end_date fomrat is: YYYYMMDD
        //! vt.group_api_usage("user_id", None, None);
        //! ```
        let url = format!("{}/groups/{}/api_usage", self.endpoint, id);
        let mut query_params: Vec<(&str, &str)> = Vec::new();
        if let Some(s) = start_date {
            query_params.push(("start_date", s))
        }
        if let Some(e) = end_date {
            query_params.push(("end_date", e))
        }
        http_get_with_params(
            &self.api_key,
            &self.user_agent,
            &url,
            &query_params.as_slice(),
        )
    }
}
