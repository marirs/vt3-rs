mod response;
pub use response::{CommentRoot, CommentsRoot, Attributes};
mod model;
pub use model::{Comment, Comments, CommentAttributes, CommentVotes};

use crate::{
    utils::{http_delete, http_get, http_get_with_params},
    VtClient, VtResult,
};

impl VtClient {
    pub fn get_comments(
        self,
        limit: Option<&str>,
        filter: Option<&str>,
        cursor: Option<&str>,
    ) -> VtResult<CommentsRoot> {
        //! Retrieves information about the latest comments.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.get_comments(Some("10"), Some("tag:malware"), None);
        //! ```
        let url = format!("{}/comments", self.endpoint);
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

    pub fn get_comment(self, comment_id: &str) -> VtResult<CommentRoot> {
        //! Retrieve a public_api.comment information.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! let comment_id = "u-011915942db556bbab5137f761efe61fed2b00598fea900360b800b193a7bf31-d94d7c8a";
        //! vt.get_comment(comment_id);
        //! ```
        let url = format!("{}/comments/{}", self.endpoint, comment_id);
        http_get(&self.api_key, &self.user_agent, &url)
    }

    pub fn delete_comment(self, comment_id: &str) -> VtResult<CommentRoot> {
        //! Delete a public_api.comment.
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! let comment_id = "u-011915942db556bbab5137f761efe61fed2b00598fea900360b800b193a7bf31-d94d7c8a";
        //! vt.delete_comment(comment_id);
        //! ```
        let url = format!("{}/comments/{}", self.endpoint, comment_id);
        http_delete(&self.api_key, &self.user_agent, &url)
    }
}
