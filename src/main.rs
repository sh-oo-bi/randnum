use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::path("randnum").map(|| format!("123456"));

    warp::serve(route).run(([127, 0, 0, 1], 1337)).await;
}
