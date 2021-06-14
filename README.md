# VirusTotal Api v3
![Crates.io](https://img.shields.io/crates/v/vt3)
[![Documentation](https://docs.rs/vt3/badge.svg)](https://docs.rs/vt3)
![docs.rs](https://img.shields.io/docsrs/vt3/latest)
[![Build Status](https://travis-ci.com/marirs/vt3-rs.svg?branch=main)](https://travis-ci.com/marirs/vt3-rs)
[![GitHub license](https://img.shields.io/github/license/marirs/vt3-rs)](https://github.com/marirs/vt3-rs/blob/main/LICENSE)

VT3 provides an easy api interface to use VirusTotal v3 REST endpoints, 
including those exclusive to VirusTotal Enterprise such as 
- Live Hunt
- Retro Hunt 
- Zip Files 

## Usage
- Cargo.toml
```toml
[dependencies]
vt3 = "0.3.5"
```

- and then: to get `ip information`
```rust
use vt3::VtClient;

fn main() {
    let api_key = match std::env::args()
        .skip(1)
        .next()
        .ok_or_else(|| "Please provide the api key!")
    {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };
    let ip_address = "5.2.69.42";

    let vt_client = VtClient::new(&api_key);
    match vt_client.ip_info(ip_address) {
        Ok(report) => println!("{:#?}", report),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
```

## Examples
To run the examples:
- `cargo run --example domain_info <your_api_key>`
- `cargo run --example ip_info <your_api_key>`
- `cargo run --example url_info <your_api_key>`
- `cargo run --example url_info_by_id <your_api_key>`
- `cargo run --example url_rescan <your_api_key>`
- `cargo run --example file_scan <your_api_key>`
---
License: MIT

