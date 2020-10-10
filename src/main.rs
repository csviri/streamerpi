use std::fs;
use std::result::Result;

use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let file_list = warp::path!("files")
        .map(|| warp::reply::html(file_list_html(".")));

    println!("Starting server!");
    warp::serve(file_list)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn file_list_html(dir: &str) -> String {
    let mut result: String = String::new();
    result.push_str("<html><body><ul>");
    let dir = fs::read_dir(dir);
    match dir {
        Result::Err(_) => { result.push_str("ERROR!! : ") }
        Result::Ok(read_dir) => {
            for entry in read_dir {
                match entry {
                    Ok(dir_entry) => {
                        let file = dir_entry.file_name();
                        match file.to_str() {
                            Some(file) => { result.push_str(format!("<li>{}</li>", file).as_str()) }
                            None => {}
                        }
                    }
                    Err(err) => eprintln!("Error: {}", err)
                }
            }
        }
    }
    result.push_str("</ul></body></html>");
    return result;
}