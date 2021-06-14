use vt3::VtClient;

fn main() {
    let api_key = match std::env::args().nth(2).ok_or("Please provide the api key!") {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };
    let ip_address = "5.2.69.42";

    let res = VtClient::new(&api_key)
        .user_agent("Chrome for Windows")
        .ip_info(ip_address);
    match res {
        Ok(report) => println!("{:#?}", report),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
