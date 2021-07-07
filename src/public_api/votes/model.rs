use crate::public_api::model::{Collection, Object};
use serde::{Deserialize, Serialize};

pub type Vote = Object<VoteAttributes>;
pub type Votes = Collection<VoteAttributes>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteAttributes {
	date: Option<i64>,
	value: Option<i64>,
	verdict: Option<String>,
}

impl VoteAttributes {
	pub fn new(date: Option<i64>, value: Option<i64>, verdict: Option<String>) -> Self {
		Self {
			date,
			value,
			verdict,
		}
	}
}
