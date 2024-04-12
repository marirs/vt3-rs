use vt3::VtClient;

fn main() {
    let api_key = match std::env::args().nth(1).ok_or("Please provide the api key!") {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };
    let comment_id = "f-e710deb5471eba0b3f28ccc961142b996c6452cc75a4051e26a8aee08b860208-c5b18827";

    let res = VtClient::new(&api_key)
        .user_agent("Chrome for Windows")
        .get_comment(comment_id);
    match res {
        Ok(report) => println!("{:#?}", report),
        Err(e) => println!("Error: {}", e),
    }
}
