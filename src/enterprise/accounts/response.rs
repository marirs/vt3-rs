use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRoot {
    pub data: Option<Data>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiUsage {
    pub data: Option<ApiUsageData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverallQuotaRoot {
    pub data: Option<OverallQuota>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub attributes: Option<Attributes>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub id: Option<String>,
    pub links: Option<Links>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub _self: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub apikey: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "has_2fa")]
    pub has_2fa: Option<bool>,
    #[serde(rename = "first_name")]
    pub profile_phrase: Option<String>,
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    #[serde(rename = "last_login")]
    pub last_login: Option<i64>,
    pub status: Option<String>,
    pub reputation: Option<i64>,
    #[serde(rename = "user_since")]
    pub user_since: Option<i64>,
    pub privileges: Option<HashMap<String, Privilege>>,
    pub quotas: Option<Quotas>,
    pub preferences: HashMap<String, Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Privilege {
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<i64>,
    pub granted: Option<bool>,
    #[serde(rename = "inherited_from")]
    pub inherited_from: Option<String>,
    #[serde(rename = "inherited_via")]
    pub inherited_via: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quotas {
    #[serde(rename = "api_requests_daily")]
    pub api_requests_daily: Option<Quota>,
    #[serde(rename = "api_requests_hourly")]
    pub api_requests_hourly: Option<Quota>,
    #[serde(rename = "api_requests_monthly")]
    pub api_requests_monthly: Option<Quota>,
    #[serde(rename = "cases_creation_monthly")]
    pub cases_creation_monthly: Option<Quota>,
    #[serde(rename = "intelligence_downloads_monthly")]
    pub intelligence_downloads_monthly: Option<Quota>,
    #[serde(rename = "intelligence_retrohunt_jobs_monthly")]
    pub intelligence_retrohunt_jobs_monthly: Option<Quota>,
    #[serde(rename = "intelligence_hunting_rules")]
    pub intelligence_hunting_rules: Option<Quota>,
    #[serde(rename = "intelligence_searches_monthly")]
    pub intelligence_searches_monthly: Option<Quota>,
    #[serde(rename = "intelligence_graphs_private")]
    pub intelligence_graphs_private: Option<Quota>,
    #[serde(rename = "intelligence_vtdiff_creation_monthly")]
    pub intelligence_vtdiff_creation_monthly: Option<Quota>,
    #[serde(rename = "monitor_storage_bytes")]
    pub monitor_storage_bytes: Option<Quota>,
    #[serde(rename = "monitor_storage_files")]
    pub monitor_storage_files: Option<Quota>,
    #[serde(rename = "monitor_uploaded_bytes")]
    pub monitor_uploaded_bytes: Option<Quota>,
    #[serde(rename = "monitor_uploaded_files")]
    pub monitor_uploaded_files: Option<Quota>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverallQuota {
    #[serde(rename = "api_requests_daily")]
    pub api_requests_daily: Option<OverallQuotaObject>,
    #[serde(rename = "api_requests_hourly")]
    pub api_requests_hourly: Option<OverallQuotaObject>,
    #[serde(rename = "api_requests_monthly")]
    pub api_requests_monthly: Option<OverallQuotaObject>,
    #[serde(rename = "cases_creation_monthly")]
    pub cases_creation_monthly: Option<OverallQuotaObject>,
    #[serde(rename = "intelligence_downloads_monthly")]
    pub intelligence_downloads_monthly: Option<OverallQuotaObject>,
    #[serde(rename = "intelligence_retrohunt_jobs_monthly")]
    pub intelligence_retrohunt_jobs_monthly: Option<OverallQuotaObject>,
    #[serde(rename = "intelligence_hunting_rules")]
    pub intelligence_hunting_rules: Option<OverallQuotaObject>,
    #[serde(rename = "intelligence_searches_monthly")]
    pub intelligence_searches_monthly: Option<OverallQuotaObject>,
    #[serde(rename = "intelligence_graphs_private")]
    pub intelligence_graphs_private: Option<OverallQuotaObject>,
    #[serde(rename = "intelligence_vtdiff_creation_monthly")]
    pub intelligence_vtdiff_creation_monthly: Option<OverallQuotaObject>,
    #[serde(rename = "monitor_storage_bytes")]
    pub monitor_storage_bytes: Option<OverallQuotaObject>,
    #[serde(rename = "monitor_storage_files")]
    pub monitor_storage_files: Option<OverallQuotaObject>,
    #[serde(rename = "monitor_uploaded_bytes")]
    pub monitor_uploaded_bytes: Option<OverallQuotaObject>,
    #[serde(rename = "monitor_uploaded_files")]
    pub monitor_uploaded_files: Option<OverallQuotaObject>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverallQuotaObject {
    pub group: Option<Quota>,
    pub user: Option<Quota>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quota {
    pub allowed: Option<i64>,
    pub used: Option<i64>,
    pub inherited_from: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiUsageData {
    pub daily: Option<HashMap<String, Value>>,
    pub total: Option<HashMap<String, i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRoot {
    pub data: Option<GroupData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupData {
    pub attributes: Option<GroupAttributes>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub id: Option<String>,
    pub links: Option<Links>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupAttributes {
    pub organization: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "country_iso")]
    pub country_iso: Option<String>,
    #[serde(rename = "organization_legal_name")]
    pub organization_legal_name: Option<String>,
    pub industry: Option<String>,
    #[serde(rename = "lock_users_api_quota_group")]
    pub lock_users_api_quota_group: Option<bool>,
    pub privileges: Option<HashMap<String, Value>>,
    pub quotas: Option<Quotas>,
    #[serde(rename = "domain_name")]
    pub domain_name: Option<String>,
    #[serde(rename = "billing_emails")]
    pub billing_emails: Option<Vec<String>>,
    #[serde(rename = "auto_add_users")]
    pub auto_add_users: Option<Vec<String>>,
    #[serde(rename = "contact_emails")]
    pub contact_emails: Option<Vec<String>>,
    #[serde(rename = "quota_usage_by_user")]
    pub quota_usage_by_user: Option<HashMap<String, Value>>,
}
