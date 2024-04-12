use super::model::{Links, Meta};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct RelatedCollections {
    data: Vec<Data>,
    meta: Meta,
    links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "type")]
    pub object_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
