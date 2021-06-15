use vt3::VtClient;

fn main() {
    let api_key = match std::env::args().nth(1).ok_or("Please provide the api key!") {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };
    let file = "data/eicar.com.txt";

    let res = VtClient::new(&api_key).file_scan(file);
    match res {
        Ok(report) => println!("{:#?}", report),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
