use crate::public_api::{
	comment::{CommentAttributes, CommentVotes},
	model::Object,
	votes::VoteAttributes,
};
use serde::Serialize;

type CreateCommentReq = Object<CommentAttributes>;

pub fn create_comment_req(attrs: CommentAttributes) -> CreateCommentReq {
	CreateCommentReq::build(Some("comment".to_string()), Some(attrs), None)
}

type CreateVoteReq = Object<VoteAttributes>;

pub fn create_vote_req(attrs: VoteAttributes) -> CreateVoteReq {
	CreateVoteReq::build(Some("vote".to_string()), Some(attrs), None)
}
