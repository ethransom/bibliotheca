use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!().map(|| format!("Hello, world!"));

    warp::serve(hello).run(([0, 0, 0, 0, 0, 0, 0, 0], 8080)).await;
}
