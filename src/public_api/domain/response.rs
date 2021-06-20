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
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub id: Option<String>,
    pub links: Option<Links>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(rename = "last_dns_records")]
    pub last_dns_records: Option<Vec<LastDnsRecord>>,
    pub jarm: Option<String>,
    pub whois: Option<String>,
    #[serde(rename = "last_https_certificate_date")]
    pub last_https_certificate_date: Option<i64>,
    pub tags: Option<Vec<::serde_json::Value>>,
    #[serde(rename = "popularity_ranks")]
    pub popularity_ranks: Option<HashMap<String, Rank>>,
    #[serde(rename = "last_dns_records_date")]
    pub last_dns_records_date: Option<i64>,
    #[serde(rename = "last_analysis_stats")]
    pub last_analysis_stats: Option<LastAnalysisStats>,
    #[serde(rename = "creation_date")]
    pub creation_date: Option<i64>,
    #[serde(rename = "whois_date")]
    pub whois_date: Option<i64>,
    pub reputation: Option<i64>,
    pub registrar: Option<String>,
    #[serde(rename = "last_analysis_results")]
    pub last_analysis_results: Option<HashMap<String, Engine>>,
    #[serde(rename = "last_update_date")]
    pub last_update_date: Option<i64>,
    #[serde(rename = "last_modification_date")]
    pub last_modification_date: Option<i64>,
    #[serde(rename = "last_https_certificate")]
    pub last_https_certificate: Option<LastHttpsCertificate>,
    pub categories: Option<HashMap<String, String>>,
    #[serde(rename = "total_votes")]
    pub total_votes: Option<TotalVotes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastDnsRecord {
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub value: Option<String>,
    pub ttl: Option<i64>,
    pub flag: Option<i64>,
    pub tag: Option<String>,
    pub priority: Option<i64>,
    pub rname: Option<String>,
    pub retry: Option<i64>,
    pub minimum: Option<i64>,
    pub refresh: Option<i64>,
    pub expire: Option<i64>,
    pub serial: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub timestamp: Option<i64>,
    pub rank: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastAnalysisStats {
    pub harmless: Option<i64>,
    pub malicious: Option<i64>,
    pub suspicious: Option<i64>,
    pub undetected: Option<i64>,
    pub timeout: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Engine {
    pub category: Option<String>,
    pub result: Option<String>,
    pub method: Option<String>,
    #[serde(rename = "engine_name")]
    pub engine_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastHttpsCertificate {
    pub size: Option<i64>,
    #[serde(rename = "public_key")]
    pub public_key: Option<PublicKey>,
    #[serde(rename = "thumbprint_sha256")]
    pub thumbprint_sha256: Option<String>,
    pub tags: Option<Vec<::serde_json::Value>>,
    #[serde(rename = "cert_signature")]
    pub cert_signature: Option<CertSignature>,
    pub validity: Option<Validity>,
    pub version: Option<String>,
    pub extensions: Option<Extensions>,
    #[serde(rename = "signature_algorithm")]
    pub signature_algorithm: Option<String>,
    #[serde(rename = "serial_number")]
    pub serial_number: Option<String>,
    #[serde(rename = "first_seen_date")]
    pub first_seen_date: Option<u32>,
    pub thumbprint: Option<String>,
    pub issuer: Option<Issuer>,
    pub subject: Option<Subject>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub ec: Option<Ec>,
    pub algorithm: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ec {
    pub oid: Option<String>,
    #[serde(rename = "pub")]
    pub pub_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertSignature {
    pub signature: Option<String>,
    #[serde(rename = "signature_algorithm")]
    pub signature_algorithm: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Validity {
    #[serde(rename = "not_after")]
    pub not_after: Option<String>,
    #[serde(rename = "not_before")]
    pub not_before: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    #[serde(rename = "certificate_policies")]
    pub certificate_policies: Option<Vec<String>>,
    #[serde(rename = "extended_key_usage")]
    pub extended_key_usage: Option<Vec<String>>,
    #[serde(rename = "authority_key_identifier")]
    pub authority_key_identifier: Option<AuthorityKeyIdentifier>,
    #[serde(rename = "subject_alternative_name")]
    pub subject_alternative_name: Option<Vec<String>>,
    // TODO: remove serde_json dependency
    pub tags: Option<Vec<::serde_json::Value>>,
    #[serde(rename = "subject_key_identifier")]
    pub subject_key_identifier: Option<String>,
    #[serde(rename = "crl_distribution_points")]
    pub crl_distribution_points: Option<Vec<String>>,
    #[serde(rename = "key_usage")]
    pub key_usage: Option<Vec<String>>,
    #[serde(rename = "1.3.6.1.4.1.11129.2.4.2")]
    pub n13614111129242: Option<String>,
    #[serde(rename = "CA")]
    pub ca: Option<bool>,
    #[serde(rename = "ca_information_access")]
    pub ca_information_access: Option<CaInformationAccess>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorityKeyIdentifier {
    pub keyid: Option<String>,
    pub serial_number: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaInformationAccess {
    #[serde(rename = "CA Issuers")]
    pub ca_issuers: Option<String>,
    #[serde(rename = "OCSP")]
    pub ocsp: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issuer {
    #[serde(rename = "C")]
    pub c: Option<String>,
    #[serde(rename = "CN")]
    pub cn: Option<String>,
    #[serde(rename = "O")]
    pub o: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    #[serde(rename = "C")]
    pub c: Option<String>,
    #[serde(rename = "ST")]
    pub st: Option<String>,
    #[serde(rename = "L")]
    pub l: Option<String>,
    #[serde(rename = "O")]
    pub o: Option<String>,
    #[serde(rename = "CN")]
    pub cn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalVotes {
    pub harmless: Option<i64>,
    pub malicious: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: Option<String>,
}
