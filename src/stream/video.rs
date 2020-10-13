use warp::Reply;
use warp::hyper::Body;
use warp::http::{Response, HeaderValue};
use warp::hyper::header::CONTENT_TYPE;

// todo real streaming no buffering?
pub struct VideoStream {
    pub bytes: Vec<u8>
}

impl Reply for VideoStream {
    #[inline]
    fn into_response(self) -> Response<Body> {
        warp::http::Response::builder()
            .header(
                CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            )
            .status(200)
            .body(Body::from(self.bytes))
            .unwrap()
    }
}