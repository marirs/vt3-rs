use vt3::VtClient;

fn main() {
    let mut args = std::env::args().skip(1);
    let api_key = match args.next().ok_or("Please provide the api key as argument!") {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };

    let res = VtClient::new(&api_key).get_retrohunt_jobs(None, None, None);
    match res {
        Ok(report) => println!("{:#?}", report),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
