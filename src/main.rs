use std::time::Duration;

use format::ListItem;

pub mod client;
pub mod format;
fn main() {
    // get the args
    let args: Vec<String> = std::env::args().collect();
    let mut name = String::new();
    let mut magnet_node = false;

    for index in 1..args.len() {
        // get the input name
        if !args[index].starts_with("--") {
            name.push_str(format!("{} ", args[index].as_str()).as_str());
        }
        // commands
        if args[index].starts_with("--magnet-mode") {
            magnet_node = true;
        }
    }

    // perform the fetch
    let url = format!("https://nyaa.si/?f=0&c=0_0&q={}", name);
    let req = client::Get {
        url: &url,
        timeout: Duration::from_secs(2),
    };
    let body = req.fetch();

    // parse the document (in format file) and then return the list of anime
    let response: Vec<ListItem> = format::get_list(body);

    // loop the list and output the result
    for element in response {
        if magnet_node {
            println!("{}", element.magnet);
        } else {
            println!(
                "Name: {}\nSize:{}\nMagnet:{}",
                element.name, element.size, element.magnet
            );
        }
    }
}
