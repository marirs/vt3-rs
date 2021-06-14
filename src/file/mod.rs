use std::{fs::File, io::Read};
use reqwest::blocking::{multipart::Form, multipart::Part};

mod response;
use response::{ScanRoot, Root};

use crate::{VtClient, error::VtError, utils::{http_multipart_post, http_post, http_get}};

impl<'a> VtClient<'a> {
    pub fn file_info(self, id: &str) -> Result<Root, VtError> {
        //! Retrieve file scan reports
        //! id: SHA-256, SHA-1 or MD5 identifying the file
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.file_info("44d88612fea8a8f36de82e1278abb02f");
        //! ```
        let url = format!("{}/files/{}", self.endpoint, id);

        let text = match http_get(self.api_key, self.user_agent, &url) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        let res: Root = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(VtError::Json(e)),
        };

        Ok(res)
    }
    
    pub fn file_scan(self, file: &'static str) -> Result<ScanRoot, VtError> {
        //! Upload and scan a file
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! println!("{:?}", vt.file_scan("data/eicar.com.txt"));
        //! ```
        let mut f = File::open(file).unwrap();
        let mut buffer = Vec::new();
        {
            match f.read_to_end(&mut buffer) {
                Ok(_) => {}
                Err(e) => return Err(VtError::Io(e)),
            };
        }
        let form_data = Form::new().part("file", Part::bytes(buffer).file_name(file));

        let url = format!("{}/files", self.endpoint);
        let text = match http_multipart_post(self.api_key, self.user_agent, &url, form_data) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        let res: ScanRoot = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(VtError::Json(e)),
        };

        Ok(res)
    }

    pub fn file_rescan(self, id: &str) -> Result<ScanRoot, VtError> {
        //! Re-submit/Re-scan already submitted files
        //! id: SHA-256, SHA-1 or MD5 identifying the file
        //!
        //! ## Example Usage
        //! ```rust
        //! use vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.file_rescan("44d88612fea8a8f36de82e1278abb02f");
        //! ```
        let url = format!("{}/files/{}/analyse", self.endpoint, id);
        let form_data = &[("id", id)];

        let text = match http_post(self.api_key, self.user_agent, &url, form_data) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        let res: ScanRoot = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(VtError::Json(e)),
        };

        Ok(res)
    }
}
