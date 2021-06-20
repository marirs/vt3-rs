use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Option<Data>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub attributes: Option<Attributes>,
    pub id: Option<String>,
    pub links: Option<Links>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub categories: Option<HashMap<String, String>>,
    pub favicon: Option<Favicon>,
    #[serde(rename = "first_submission_date")]
    pub first_submission_date: Option<i64>,
    #[serde(rename = "has_content")]
    pub has_content: Option<bool>,
    #[serde(rename = "html_meta")]
    pub html_meta: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "last_analysis_date")]
    pub last_analysis_date: Option<i64>,
    #[serde(rename = "last_analysis_results")]
    pub last_analysis_results: Option<HashMap<String, Engine>>,
    #[serde(rename = "last_analysis_stats")]
    pub last_analysis_stats: Option<LastAnalysisStats>,
    #[serde(rename = "last_final_url")]
    pub last_final_url: Option<String>,
    #[serde(rename = "last_http_response_code")]
    pub last_http_response_code: Option<i64>,
    #[serde(rename = "last_http_response_content_length")]
    pub last_http_response_content_length: Option<i64>,
    #[serde(rename = "last_http_response_content_sha256")]
    pub last_http_response_content_sha256: Option<String>,
    #[serde(rename = "last_http_response_cookies")]
    pub last_http_response_cookies: Option<HashMap<String, String>>,
    #[serde(rename = "last_http_response_headers")]
    pub last_http_response_headers: Option<HashMap<String, String>>,
    #[serde(rename = "last_modification_date")]
    pub last_modification_date: Option<i64>,
    #[serde(rename = "last_submission_date")]
    pub last_submission_date: Option<i64>,
    #[serde(rename = "outgoing_links")]
    pub outgoing_links: Option<Vec<String>>,
    pub reputation: Option<i64>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "targeted_brand")]
    pub targeted_brand: Option<HashMap<String, String>>,
    #[serde(rename = "times_submitted")]
    pub times_submitted: Option<i64>,
    pub title: Option<String>,
    #[serde(rename = "total_votes")]
    pub total_votes: Option<TotalVotes>,
    pub trackers: Option<HashMap<String, Vec<Tracker>>>,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Favicon {
    pub dhash: Option<String>,
    #[serde(rename = "raw_md5")]
    pub raw_md5: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlMeta {
    pub description: Option<Vec<String>>,
    pub sessid: Option<Vec<String>>,
    pub viewport: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Engine {
    pub category: Option<String>,
    #[serde(rename = "engine_name")]
    pub engine_name: Option<String>,
    pub method: Option<String>,
    pub result: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastAnalysisStats {
    pub harmless: Option<i64>,
    pub malicious: Option<i64>,
    pub suspicious: Option<i64>,
    pub timeout: Option<i64>,
    pub undetected: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalVotes {
    pub harmless: Option<i64>,
    pub malicious: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracker {
    pub id: Option<String>,
    pub timestamp: Option<i64>,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanRoot {
    pub data: ScanData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanData {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
}
