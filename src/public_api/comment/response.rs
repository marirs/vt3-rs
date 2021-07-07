use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentRoot {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsRoot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    pub data: Vec<Data>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub attributes: Option<Attributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub votes: Option<Votes>,
}

impl Attributes {
    pub fn new(
        date: Option<i64>,
        html: Option<String>,
        tags: Option<Vec<String>>,
        text: Option<String>,
        votes: Option<Votes>,
    ) -> Attributes {
        Self {
            date,
            html,
            tags,
            text,
            votes,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Votes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive: Option<i64>,
}

impl Votes {
    pub fn new(abuse: Option<i64>, negative: Option<i64>, positive: Option<i64>) -> Self {
        Self {
            abuse,
            negative,
            positive,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub _next: Option<String>,
}
