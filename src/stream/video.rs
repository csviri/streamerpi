use std::cmp::min;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use warp::http::{HeaderValue, Response};
use warp::hyper::Body;
use warp::hyper::header::{CONTENT_RANGE, CONTENT_TYPE};
use warp::Reply;

use crate::stream::range::Range;

// todo real streaming in the future
// how it works usually with html5, browser asks for the whole file (even if its very big)
// but reads the stream only slowly (ste by step to fill a buffer ahead)

// todo discuss streams in sig

pub struct VideoStream {
    pub bytes: Vec<u8>,
    pub start: u64,
    pub end: u64,
    pub overall_size: u64,
}

impl Reply for VideoStream {
    #[inline]
    fn into_response(self) -> Response<Body> {
        warp::http::Response::builder()
            .header(CONTENT_TYPE, HeaderValue::from_static("video/mp4"))
            .header(CONTENT_RANGE, format!("bytes {}-{}/{}", self.start, self.end, self.overall_size),
            )
            .status(if self.end - self.start == self.overall_size { 200 } else { 206 })
            .body(Body::from(self.bytes))
            .unwrap()
    }
}

pub fn read_file_range_to_video_stream(file_path: PathBuf, range: Range, max_size: u64) -> VideoStream {
    let mut f = File::open(&file_path).expect("no file found");
    let metadata = fs::metadata(&file_path).expect("unable to read metadata");
    let chunk_size = calculate_chunk_size(&range, max_size, metadata.len());

    let _res= f.seek(SeekFrom::Start(range.start));
    let mut buffer = vec![0; chunk_size as usize];
    f.read(&mut buffer).expect("todo error handling");

    VideoStream {
        bytes: buffer,
        start: range.start,
        end: range.start + chunk_size,
        overall_size: metadata.len(),
    }
}

// todo unit tests
fn calculate_chunk_size(range: &Range, max_size: u64, file_length: u64) -> u64 {
    let res = min(max_size, file_length - range.start);
    return match range.end {
        Some(end) => { min(end - range.start, res) }
        None => res
    };
}