use std::fs;
use std::result::Result;

use warp::Filter;

#[tokio::main]
async fn main() {
    const DIR: &str = ".";

    let file_list = warp::path!("files")
        .map(|| warp::reply::html(list_files_to_html(DIR)));

    let stream = warp::get()
        .and(warp::path("stream"))
        .and(warp::path::param::<String>())
        .and(warp::header::optional::<String>("range"))
        .map(|name: String, range: Option<String>| {
            let range_val = match range {
                Some(range) => { range }
                None => { "Not present.".to_string() }
            };
            format!("Name: {}, range: {}", name, range_val)
        });

    println!("Server starting!");
    warp::serve(file_list.or(stream))

        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn list_files_to_html(dir: &str) -> String {
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