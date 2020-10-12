use std::{env, fs};
use std::fs::{DirEntry, File};
use std::io::Read;
use std::path::PathBuf;
use std::result::Result;

use warp::Filter;

static VIDEO_PAGE_TEMPLATE: &'static str = include_str!("video.html");

// todo reading file as async?

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let root_dir = args.get(1).expect("Root directory argument expected.").to_string();
    // todo is there no better way?
    let root_clone = root_dir.clone();
    let root_clone2 = root_dir.clone();
    println!("Root dir: {}", root_dir);

    let file_list = warp::path::end()
        .map(move || warp::reply::html(list_files_to_html(&root_clone)));

    let dir_view = warp::get()
        .and(warp::path("dir"))
        .and(warp::path::param::<String>())
        .map(move |dir: String| warp::reply::html(list_files_to_html(format!("{}/{}", root_clone2, dir).as_str())));

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
        // .and_then(get_file_bytes);
        .map(move |name: String, range: Option<String>| {
            match range {
                Some(range) => {
                    let bytes = read_file_range(&root_dir, name, range);
                    return bytes;
                }
                None => {
                    panic!("TODO")
                }
            };
        });

    println!("Server up!");
    warp::serve(file_list.or(stream).or(video_page).or(dir_view))
        .run(([127, 0, 0, 1], 8080)) // todo port as param
        .await;
}

fn read_file_range(root_dir: &str, file_name: String, _range: String) -> Vec<u8> {

    // todo respect range header

    let mut path = PathBuf::new();
    path.push(root_dir);
    path.push(file_name);

    let mut f = File::open(&path).expect("no file found");
    let metadata = fs::metadata(&path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

fn video_html(file_name: String) -> String {
    return VIDEO_PAGE_TEMPLATE.replace("{file_name}", &file_name);
}


fn list_files_to_html(path_to_dir: &str) -> String {
    let mut result: String = String::new();
    result.push_str("<html><body><ul>");
    let dir = fs::read_dir(path_to_dir);
    match dir {
        Result::Err(_) => { result.push_str(format!("ERROR Reading dir: {}", path_to_dir).as_str()) }
        Result::Ok(read_dir) => {
            for entry in read_dir {
                match entry {
                    Ok(dir_entry) => {
                        match dir_entry_to_html(dir_entry) {
                            Ok(entry) => {
                                result.push_str(entry.as_str());
                            }
                            Err(_) => {}
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

fn dir_entry_to_html(dir_entry: DirEntry) -> Result<String, std::io::Error> {
    let file = dir_entry.file_name();
    let meta = dir_entry.metadata()?;

    match file.to_str() {
        Some(name) => {
            if meta.is_file() {
                return Ok(format!("<li><a href=\"/video/{}\">{}</a></li>", name, name));
            }
            if meta.is_dir() {
                return Ok(format!("<li><a href=\"/dir/{}\">[ {} ]</a></li>", name, name));
            }
            return Ok(String::new());
        }
        None => { return Ok(String::new()); }
    }
}