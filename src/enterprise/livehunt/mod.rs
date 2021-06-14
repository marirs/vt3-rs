mod response;
use response::SearchRulesetRoot;
pub use response::SubmitRulesetRoot;

use crate::{
    error::VtError,
    utils::{http_body_post, http_delete, http_get, http_get_with_params, http_patch},
    VtClient,
};

impl<'a> VtClient<'a> {
    pub fn get_rulesets(
        self,
        limit: Option<&str>,
        order: Option<&str>,
        filter: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<SearchRulesetRoot, VtError> {
        //! Retrieve VT Hunting livehunt rulesets
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.get_rulesets(Some("10"), None, None, None);
        //! ```
        let url = format!("{}/intelligence/hunting_rulesets", self.endpoint);
        let mut query_params: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query_params.push(("limit", l))
        }
        if let Some(o) = order {
            query_params.push(("order", o))
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

        let res: SearchRulesetRoot = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(VtError::Json(e)),
        };

        Ok(res)
    }

    pub fn get_ruleset(self, ruleset_id: &str) -> Result<SearchRulesetRoot, VtError> {
        //! Retrieve a VT Hunting Livehunt ruleset given by a id
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.get_ruleset("1");
        //! ```
        let url = format!(
            "{}/intelligence/hunting_rulesets/{}",
            self.endpoint, ruleset_id
        );
        let text = match http_get(self.api_key, self.user_agent, &url) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        let res: SearchRulesetRoot = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(VtError::Json(e)),
        };

        Ok(res)
    }

    pub fn create_ruleset(self, data: &SubmitRulesetRoot) -> Result<SubmitRulesetRoot, VtError> {
        //! Create/Submit a new VT Hunting Livehunt ruleset
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::{VtClient, SubmitRulesetRoot};
        //!
        //! let vt = VtClient::new("Your API Key");
        //! let mut data = SubmitRulesetRoot::default();
        //! vt.create_ruleset(&data);
        //! ```
        let url = format!("{}/intelligence/hunting_rulesets", self.endpoint);
        let js = match serde_json::to_string(data) {
            Ok(j) => j,
            Err(e) => return Err(VtError::Json(e)),
        };
        let text = match http_body_post(self.api_key, self.user_agent, &url, js) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        let res: SubmitRulesetRoot = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(VtError::Json(e)),
        };

        Ok(res)
    }

    pub fn delete_ruleset(self, ruleset_id: &str) -> Result<String, VtError> {
        //! Delete/Remove Hunting ruleset by ID
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.delete_ruleset("1");
        //! ```
        let url = format!(
            "{}/intelligence/hunting_rulesets/{}",
            self.endpoint, ruleset_id
        );
        let text = match http_delete(self.api_key, self.user_agent, &url) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        Ok(text)
    }

    pub fn update_ruleset(
        self,
        ruleset_id: &str,
        data: &SubmitRulesetRoot,
    ) -> Result<SubmitRulesetRoot, VtError> {
        //! Update/Modify a VT Hunting Livehunt ruleset
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::{VtClient, SubmitRulesetRoot};
        //!
        //! let vt = VtClient::new("Your API Key");
        //! let mut data = SubmitRulesetRoot::default();
        //! vt.update_ruleset("1", &data);
        //! ```
        let url = format!(
            "{}/intelligence/hunting_rulesets/{}",
            self.endpoint, ruleset_id
        );
        let js = match serde_json::to_string(data) {
            Ok(j) => j,
            Err(e) => return Err(VtError::Json(e)),
        };
        let text = match http_patch(self.api_key, self.user_agent, &url, js) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        let res: SubmitRulesetRoot = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(VtError::Json(e)),
        };

        Ok(res)
    }
}
