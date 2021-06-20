mod response;
use response::SearchRulesetRoot;
pub use response::SubmitLivehuntRuleset;

use crate::{
    utils::{http_body_post, http_delete, http_get, http_get_with_params, http_patch},
    VtClient, VtResult,
};

impl VtClient {
    pub fn get_rulesets(
        &self,
        limit: Option<&str>,
        order: Option<&str>,
        filter: Option<&str>,
        cursor: Option<&str>,
    ) -> VtResult<SearchRulesetRoot> {
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

        http_get_with_params(
            &self.api_key,
            &self.user_agent,
            &url,
            &query_params.as_slice(),
        )
    }

    pub fn get_ruleset(&self, ruleset_id: &str) -> VtResult<SearchRulesetRoot> {
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
        http_get(&self.api_key, &self.user_agent, &url)
    }

    pub fn create_ruleset(&self, data: &SubmitLivehuntRuleset) -> VtResult<SubmitLivehuntRuleset> {
        //! Create/Submit a new VT Hunting Livehunt ruleset
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::{VtClient, SubmitLivehuntRuleset};
        //!
        //! let vt = VtClient::new("Your API Key");
        //! let mut data = SubmitLivehuntRuleset::default();
        //! vt.create_ruleset(&data);
        //! ```
        let url = format!("{}/intelligence/hunting_rulesets", self.endpoint);
        http_body_post(&self.api_key, &self.user_agent, &url, data)
    }

    pub fn delete_ruleset(&self, ruleset_id: &str) -> VtResult<String> {
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
        http_delete(&self.api_key, &self.user_agent, &url)
    }

    pub fn update_ruleset(
        &self,
        ruleset_id: &str,
        data: &SubmitLivehuntRuleset,
    ) -> VtResult<SubmitLivehuntRuleset> {
        //! Update/Modify a VT Hunting Livehunt ruleset
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::{VtClient, SubmitLivehuntRuleset};
        //!
        //! let vt = VtClient::new("Your API Key");
        //! let mut data = SubmitLivehuntRuleset::default();
        //! vt.update_ruleset("1", &data);
        //! ```
        let url = format!(
            "{}/intelligence/hunting_rulesets/{}",
            self.endpoint, ruleset_id
        );
        http_patch(&self.api_key, &self.user_agent, &url, data)
    }
}
