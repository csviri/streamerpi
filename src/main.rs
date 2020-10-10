use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let fileList = warp::path!("files")
        .map(|| {
            format!("File list")
        });


    warp::serve(fileList)
        .run(([127, 0, 0, 1], 3030))
        .await;
}