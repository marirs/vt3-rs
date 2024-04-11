use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
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
    #[serde(rename = "as_owner")]
    pub as_owner: Option<String>,
    pub asn: Option<i64>,
    pub continent: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "last_analysis_results")]
    pub last_analysis_results: Option<HashMap<String, Engine>>,
    #[serde(rename = "last_analysis_stats")]
    pub last_analysis_stats: Option<LastAnalysisStats>,
    #[serde(rename = "last_https_certificate")]
    pub last_https_certificate: Option<LastHttpsCertificate>,
    #[serde(rename = "last_https_certificate_date")]
    pub last_https_certificate_date: Option<i64>,
    #[serde(rename = "last_modification_date")]
    pub last_modification_date: Option<i64>,
    #[serde(rename = "last_analysis_date")]
    pub last_analysis_date: Option<i64>,
    pub network: Option<String>,
    #[serde(rename = "regional_internet_registry")]
    pub regional_internet_registry: Option<String>,
    pub reputation: Option<i64>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "total_votes")]
    pub total_votes: Option<TotalVotes>,
    pub whois: Option<String>,
    #[serde(rename = "whois_date")]
    pub whois_date: Option<i64>,
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
pub struct LastHttpsCertificate {
    #[serde(rename = "cert_signature")]
    pub cert_signature: Option<CertSignature>,
    pub extensions: Option<Extensions>,
    pub issuer: Option<Issuer>,
    #[serde(rename = "public_key")]
    pub public_key: Option<PublicKey>,
    #[serde(rename = "serial_number")]
    pub serial_number: Option<String>,
    #[serde(rename = "signature_algorithm")]
    pub signature_algorithm: Option<String>,
    pub size: Option<i64>,
    pub subject: Option<Subject>,
    pub thumbprint: Option<String>,
    #[serde(rename = "thumbprint_sha256")]
    pub thumbprint_sha256: Option<String>,
    pub validity: Option<Validity>,
    pub version: Option<String>,
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
pub struct Extensions {
    #[serde(rename = "1.3.6.1.4.1.11129.2.4.2")]
    pub n13614111129242: Option<String>,
    #[serde(rename = "CA")]
    pub ca: Option<bool>,
    #[serde(rename = "authority_key_identifier")]
    pub authority_key_identifier: Option<AuthorityKeyIdentifier>,
    #[serde(rename = "ca_information_access")]
    pub ca_information_access: Option<CaInformationAccess>,
    #[serde(rename = "certificate_policies")]
    pub certificate_policies: Option<Vec<String>>,
    #[serde(rename = "extended_key_usage")]
    pub extended_key_usage: Option<Vec<String>>,
    #[serde(rename = "key_usage")]
    pub key_usage: Option<Vec<String>>,
    #[serde(rename = "subject_alternative_name")]
    pub subject_alternative_name: Option<Vec<String>>,
    #[serde(rename = "subject_key_identifier")]
    pub subject_key_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorityKeyIdentifier {
    pub keyid: Option<String>,
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
pub struct PublicKey {
    pub algorithm: Option<String>,
    pub rsa: Option<Rsa>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rsa {
    pub exponent: Option<String>,
    #[serde(rename = "key_size")]
    pub key_size: Option<i64>,
    pub modulus: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    #[serde(rename = "CN")]
    pub cn: Option<String>,
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
