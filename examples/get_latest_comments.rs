use vt3::VtClient;

fn main() {
    let api_key = match std::env::args().nth(1).ok_or("Please provide the api key!") {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };

    let res = VtClient::new(&api_key).get_comments(Some("10"), Some("tag:malware"), None);
    match res {
        Ok(report) => println!("{:#?}", report),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
