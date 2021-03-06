use vt3::VtClient;

fn main() {
    let mut args = std::env::args().skip(1);
    let api_key = match args
        .next()
        .ok_or("Please provide the api key as 1st argument!")
    {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };

    let user_id = match args
        .next()
        .ok_or("Please provide the user id as 2nd argument!")
    {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };

    let res = VtClient::new(&api_key).api_usage(&user_id, Some("20210618"), Some("20210620"));
    match res {
        Ok(report) => println!("{:#?}", report),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
