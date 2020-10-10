use warp::Filter;
use std::fs;
use std::error::Error;
use std::fs::ReadDir;
use std::result::Result;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let file_list = warp::path!("files")
        .map(|| {
            format!("File list: {}", file_list_html("."))
        });


    warp::serve(file_list)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn file_list_html(dir: &str) -> String {
    let mut result: String = String::new();
    result.push_str("<html><body><ul>");
    let dir = fs::read_dir(dir);
    match dir {
        // todo message
        Result::Err(err) => { result.push_str("ERROR!!")}
        Result::Ok(read_dir) => {
            for entry in read_dir {
                // I'm here
                result.push_str("<li>{}</li>",)
            }
        }
    }

    result.push_str("</ul></body></html>");
    return result;
}