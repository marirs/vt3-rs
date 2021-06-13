# VirusTotal Api v3

VT3 provides an easy api interface to use VirusTotal v3 REST endpoints, 
including those exclusive to VirusTotal Enterprise such as 
- Live Hunt
- Retro Hunt 
- Zip Files 

## Usage
- Cargo.toml
```toml
[dependencies]
vt3 = "0.3.2"
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


--
License: MIT

