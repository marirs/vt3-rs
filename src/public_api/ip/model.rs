use crate::public_api::comment::Comments;
use crate::public_api::file::VtFiles;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Relationships {
    Comments,
    CommunicatingFiles,
    DownloadedFiles,
    // Graphs,
    // HistorialSslCertificates,
    // HistoricalWhoIs,
    RelatedComments,
    ReferrerFiles,
    // Resolutions,
    // Urls,
}

impl Relationships {
    pub fn to_string(&self) -> String {
        match to_value(self).unwrap() {
            Value::String(val) => val.to_string(),
            _ => "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RelatedObjects {
    Comments(Comments),
    Files(VtFiles),
}
