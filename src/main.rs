use std::{fs, env};
use std::result::Result;

use warp::{Filter, hyper};
use warp::http::StatusCode;
use std::fs::File;
use std::io::Read;
use warp::reply::Response;

static VIDEO_PAGE_TEMPLATE: &'static str = include_str!("video.html");

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let root_dir = args.get(1).expect("Root directory argument expected.").to_string();

    println!("Root dir: {}",root_dir);

    let file_list = warp::path::end()
        .map(move || warp::reply::html(list_files_to_html(&root_dir)));

    let video_page = warp::get()
        .and(warp::path("video"))
        .and(warp::path::param::<String>())
        .map(|file_name: String| {
            warp::reply::html(video_html(file_name))
        });

    let stream = warp::get()
        .and(warp::path("stream"))
        .and(warp::path::param::<String>())
        .and(warp::header::optional::<String>("range"))
        .and_then(get_file_bytes);
        // .map(|name: String, range: Option<String>| {
        //     return match range {
        //         Some(range) => {
        //             let bytes = read_file_range(&root_dir,name,range);
        //             Ok(hyper::Body::from(bytes))
        //         }
        //         None => {
        //             panic!("TODO")
        //         }
        //     };
        //
        // });

    println!("Server starting!");
    warp::serve(file_list.or(stream).or(video_page))
        .run(([127, 0, 0, 1], 8080))
        .await;
}

async fn get_file_bytes(name: String, range: Option<String>) -> Result<impl warp::Reply, std::convert::Infallible> {
    //todo continue from here
    let bytes = read_file_range("ddd",name,range.expect("xxx"));
    Ok(bytes)
}

fn read_file_range(root_dir: &str,file_name: String, _range: String) -> Vec<u8> {

    let mut f = File::open(&file_name).expect("no file found");
    let metadata = fs::metadata(&file_name).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer

}


fn video_html(file_name: String) -> String {
    return VIDEO_PAGE_TEMPLATE.replace("{file_name}",&file_name);
}

fn list_files_to_html(root_dir: &str) -> String {
    let mut result: String = String::new();
    result.push_str("<html><body><ul>");
    let dir = fs::read_dir(root_dir);
    match dir {
        Result::Err(_) => { result.push_str(format!("ERROR Reading dir: {}", root_dir).as_str()) }
        Result::Ok(read_dir) => {
            for entry in read_dir {
                match entry {
                    Ok(dir_entry) => {
                        let file = dir_entry.file_name();
                        match file.to_str() {
                            Some(file) => { result.push_str(format!("<li><a href=\"/video/{}\">{}</a></li>", file, file).as_str()) }
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