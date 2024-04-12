mod model;
mod request;
mod response;

pub use super::comment::CommentAttributes;
use super::comment::{Comment, Comments};

use super::file::VtFiles;
use super::relationships::RelatedCollections;

pub use super::votes::VoteAttributes;
use super::votes::{Vote, Votes};
pub use model::{RelatedObjects, Relationships};
use request::{create_comment_req, create_vote_req};
use response::Root;

use crate::{
    utils::{http_body_post, http_get},
    VtClient, VtResult,
};

impl VtClient {
    pub fn ip_info(&self, ip_address: &str) -> VtResult<Root> {
        //! Get the report of a given IP Address
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! println!("{:?}", vt.ip_info("192.168.2.1"));
        //! ```
        let url = format!("{}/ip_addresses/{}", &self.endpoint, ip_address);
        http_get(&self.api_key, &self.user_agent, &url)
    }

    pub fn ip_comments(&self, ip_address: &str) -> VtResult<Comments> {
        //! Get the comments from an ip address
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("API Key");
        //! println!("{:?}", vt.ip_comments("192.168.2.1"));
        //! ```
        let url = format!("{}/ip_addresses/{}/comments", &self.endpoint, ip_address);
        http_get(
            self.api_key.as_str(),
            self.user_agent.as_str(),
            url.as_str(),
        )
    }

    pub fn add_ip_comment(&self, ip_address: &str, attrs: CommentAttributes) -> VtResult<Comment> {
        //! Get the comments from an ip address
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //! use vt3::public_api::ip::CommentAttributes;
        //!
        //! let vt = VtClient::new("API Key");
        //! let attrs = CommentAttributes::new(
        //!     None,
        //!     None,
        //!     None,
        //!     Some("This is an example".to_string()),
        //!     None,
        //! );
        //! println!("{:?}", vt.add_ip_comment("192.168.2.1", attrs));
        //! ```
        let url = format!("{}/ip_addresses/{}/comments", &self.endpoint, ip_address);
        let body = create_comment_req(attrs);
        http_body_post(
            self.api_key.as_str(),
            self.user_agent.as_str(),
            url.as_str(),
            body,
        )
    }

    pub fn list_ip_related_objects(
        &self,
        ip_address: &str,
        relationship: Relationships,
    ) -> VtResult<RelatedObjects> {
        //! List related objects of an ip address
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //! use vt3::public_api::ip::Relationships;
        //!
        //! let vt = VtClient::new("API Key");
        //! let res =  vt.list_ip_related_objects("192.168.1.1", Relationships::Comments);
        //! ```
        let url = format!(
            "{}/ip_addresses/{}/{}",
            self.endpoint.as_str(),
            ip_address,
            relationship.to_string(),
        );
        match relationship {
            Relationships::Comments | Relationships::RelatedComments => {
                let comments: Comments = http_get(
                    self.api_key.as_str(),
                    self.user_agent.as_str(),
                    url.as_str(),
                )?;
                Ok(RelatedObjects::Comments(comments))
            }
            Relationships::CommunicatingFiles
            | Relationships::DownloadedFiles
            | Relationships::ReferrerFiles => {
                let files: VtFiles = http_get(
                    self.api_key.as_str(),
                    self.user_agent.as_str(),
                    url.as_str(),
                )?;
                Ok(RelatedObjects::Files(files))
            }
        }
    }

    pub fn list_ip_related_ids(
        &self,
        ip_address: &str,
        relationship: Relationships,
    ) -> VtResult<RelatedCollections> {
        //! List related ids of an ip address
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //! use vt3::public_api::ip::Relationships;
        //!
        //! let vt = VtClient::new("API Key");
        //! let res =  vt.list_ip_related_ids("192.168.1.1", Relationships::Comments);
        //! ```
        let url = format!(
            "{}/ip_addresses/{}/relationships/{}",
            self.endpoint.as_str(),
            ip_address,
            relationship.to_string()
        );
        http_get(
            self.api_key.as_str(),
            self.user_agent.as_str(),
            url.as_str(),
        )
    }

    pub fn list_ip_votes(&self, ip_address: &str) -> VtResult<Votes> {
        //! List votes for an ip address
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //! use vt3::public_api::ip::Relationships;
        //!
        //! let vt = VtClient::new("API Key");
        //! let res = vt.list_ip_votes("192.168.1.1");
        //! ```
        let url = format!(
            "{}/ip_addresses/{}/votes",
            self.endpoint.as_str(),
            ip_address
        );
        http_get(
            self.api_key.as_str(),
            self.user_agent.as_str(),
            url.as_str(),
        )
    }

    pub fn create_ip_vote(&self, ip_address: &str, attrs: VoteAttributes) -> VtResult<Vote> {
        let url = format!(
            "{}/ip_addresses/{}/votes",
            self.endpoint.as_str(),
            ip_address
        );
        let req = create_vote_req(attrs);
        http_body_post(
            self.api_key.as_str(),
            self.user_agent.as_str(),
            url.as_str(),
            req,
        )
    }
}
