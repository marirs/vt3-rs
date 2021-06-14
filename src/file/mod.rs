use reqwest::blocking::{multipart::Form, multipart::Part};
use std::{fs::File, io::Read};

mod response;
use response::Root;

use crate::{error::VtError, utils::http_multipart_post, VtClient};

impl<'a> VtClient<'a> {
    pub fn file_scan(self, file: &'static str) -> Result<Root, VtError> {
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

        let res: Root = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(VtError::Json(e)),
        };

        Ok(res)
    }
}
