use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchJobRoot {
    pub data: Option<Vec<Data>>,
    pub links: Option<Links>,
    pub meta: Option<Meta>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitJobRoot {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "type")]
    pub _type: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub links: Option<Links>,
    pub attributes: Option<Attributes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(rename = "creation_date")]
    pub creation_date: Option<i64>,
    pub corpus: Option<String>,
    #[serde(rename = "finish_date")]
    pub finish_date: Option<i64>,
    pub progress: Option<f64>,
    pub rules: String,
    #[serde(rename = "scanned_bytes")]
    pub scanned_bytes: Option<i64>,
    #[serde(rename = "start_date")]
    pub start_date: Option<i64>,
    pub status: Option<String>,
    pub notification_email: Option<String>,
    #[serde(rename = "total_matches")]
    pub total_matches: Option<i64>,
    #[serde(rename = "time_range")]
    pub time_range: Option<TimeRange>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub next: Option<String>,
    #[serde(rename = "self")]
    pub self_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub cursor: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeRange {
    pub start: i64,
    pub end: i64,
}
