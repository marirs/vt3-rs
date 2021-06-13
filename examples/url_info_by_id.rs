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
    let url = "https://www.example.com";

    let vt_client = VtClient::new(&api_key);
    let resource_id = match vt_client.url_scan(url) {
        Ok(report) => report,
        Err(e) => {
            println!("Error: {}", e.to_string());
            std::process::exit(1)
        }
    };

    if resource_id.data.id.is_empty() {
        println!("No resource id found")
    } else {
        match vt_client.url_info_by_id(&resource_id.data.id) {
            Ok(report) => println!("{:#?}", report),
            Err(e) => println!("Error: {}", e.to_string())
        }
    }
}
