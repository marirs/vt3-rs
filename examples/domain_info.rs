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
    let domain = "google.com";

    let vt_client = VtClient::new(&api_key);
    match vt_client.domain_info(domain) {
        Ok(report) => println!("{:#?}", report),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
