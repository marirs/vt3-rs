pub use super::response::{Attributes as CommentAttributes, Votes as CommentVotes};
use crate::public_api::model::{Collection, Object};

pub type Comment = Object<CommentAttributes>;
pub type Comments = Collection<CommentAttributes>;
