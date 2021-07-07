use vt3::{
    public_api::ip::{CommentAttributes, Relationships, VoteAttributes},
    VtClient,
};

fn main() {
    let api_key = match std::env::args().nth(1).ok_or("Please provide the api key!") {
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

    let client = VtClient::new(&api_key).user_agent("Chrome_For_Windows");

    // Add comment to an ip_addres
    let attrs = CommentAttributes::new(
        None,
        None,
        None,
        Some("This is an example".to_string()),
        None,
    );
    let new_comments = client.add_ip_comment(ip_address, attrs);
    println!("New Comments: {:?}", new_comments);

    // List comments for an ip address
    let comments = client.list_ip_related_objects(ip_address, Relationships::Comments);
    println!("Related Comments: {:?}", comments);

    // List comments for an ip address
    let comment_ids = client.list_ip_related_ids(ip_address, Relationships::Comments);
    println!("Related Ids: {:?}", comment_ids);

    // List votes for an ip address
    let votes = client.list_ip_votes(ip_address);
    println!("Votes: {:?}", votes);

    // Create Votes
    let vote = VoteAttributes::new(None, Some(10), Some("harmless".to_string()));
    println!("New vote: {:?}", vote);
}
