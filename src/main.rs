use std::{env};
use std::path::PathBuf;
use warp::{Filter, Reply};
use streamerpi::stream::range::Range;
use streamerpi::stream::video::{read_file_range_to_video_stream};
use streamerpi::browse::dir::{list_files_to_html, video_html};
use urlencoding::decode;
use warp::http::StatusCode;

//  TODO
// - reading file as async?
// - error handling
// - this goes away with real streaming


const MAX_STREAM_RESPONSE_SIZE: u64 = 2621440;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let root_dir = args.get(1).expect("Root directory argument expected.").to_string();
    // todo is there no better way?
    let root_clone = root_dir.clone();
    let root_clone2 = root_dir.clone();
    println!("Root dir: {}", root_dir);

    let file_list = warp::path::end()
        .map(move || {
            match list_files_to_html(&root_clone, "") {
                Ok(html) => {
                    warp::reply::html(html).into_response()
                }
                Err(err) => {
                    eprintln!("error: {}", err);
                    warp::reply::with_status(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR).into_response()
                }
            }
        });

    let dir_view = warp::get()
        .and(warp::path("dir"))
        .and(warp::path::param::<String>())
        .map(move |dir: String| {
            let decoded_dir = decode(dir.as_str()).unwrap();
            match list_files_to_html(root_clone2.as_str(), decoded_dir.as_str()) {
                Ok(html) => {
                    warp::reply::html(html).into_response()
                }
                Err(err) => {
                    eprintln!("error: {}", err);
                    warp::reply::with_status(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR).into_response()
                }
            }
        });

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
        .map(move |path: String, range: Option<String>| {
            match range {
                Some(range) => {
                    // todo error handling
                    let decoded_path = decode(path.as_str()).unwrap();
                    let mut path = PathBuf::new();
                    path.push(&root_dir);
                    path.push(decoded_path);
                    let range_result = Range::parse_range(range);
                    match range_result {
                        Ok(range) => {
                            return read_file_range_to_video_stream(path, range, MAX_STREAM_RESPONSE_SIZE);
                        }
                        // todo error handling
                        Err(err) => { panic!(err) }
                    }
                }
                None => { panic!("TODO") }
            };
        });

    println!("Server up! On port: {}!", 8080);
    warp::serve(file_list.or(stream).or(video_page).or(dir_view))
        .run(([0, 0, 0, 0], 8080)) // todo port as param
        .await;
}
