use reqwest::blocking::{multipart::Form, multipart::Part};
use std::{fs::File, io::Read};

mod response;
use response::{Attributes, Root, ScanRoot};
mod model;

pub use model::VtFiles;

use crate::{
    utils::{http_get, http_multipart_post, http_post},
    VtClient, VtResult,
};

impl VtClient {
    pub fn file_info(&self, id: &str) -> VtResult<Root> {
        //! Retrieve public_api.file scan reports
        //! id: SHA-256, SHA-1 or MD5 identifying the public_api.file
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.file_info("44d88612fea8a8f36de82e1278abb02f");
        //! ```
        let url = format!("{}/files/{}", &self.endpoint, id);
        http_get(&self.api_key, &self.user_agent, &url)
    }

    pub fn file_scan(&self, file: &str) -> VtResult<ScanRoot> {
        //! Upload and scan a public_api.file
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! println!("{:?}", vt.file_scan("data/eicar.com.txt"));
        //! ```
        let mut f = File::open(file)?;
        let mut buffer = Vec::new();
        {
            f.read_to_end(&mut buffer)?;
        }
        let form_data = Form::new().part("file", Part::bytes(buffer).file_name(file.to_owned()));
        let url = format!("{}/files", &self.endpoint);
        http_multipart_post(&self.api_key, &self.user_agent, &url, form_data)
    }

    pub fn file_rescan(&self, id: &str) -> VtResult<ScanRoot> {
        //! Re-submit/Re-scan already submitted files
        //! id: SHA-256, SHA-1 or MD5 identifying the public_api.file
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.file_rescan("44d88612fea8a8f36de82e1278abb02f");
        //! ```
        let url = format!("{}/files/{}/analyse", &self.endpoint, id);
        let form_data = &[("id", id)];
        http_post(&self.api_key, &self.user_agent, &url, form_data)
    }
}
