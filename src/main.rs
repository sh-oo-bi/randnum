use rand::Rng;
use serde::Serialize;
use warp::{Filter, Rejection};

#[derive(Debug, Serialize)]
struct Randnum {
    num: i32,
}

impl Randnum {
    fn new() -> Self {
        let random = {
            let mut rng = rand::thread_rng();
            rng.gen_range(1..=1000)
        };

        Self { num: random }
    }
}

async fn get_random() -> Result<impl warp::Reply, Rejection> {
    let random_number = Randnum::new();

    Ok(warp::reply::json(&random_number))
}

#[tokio::main]
async fn main() {
    let get_random = warp::get()
        .and(warp::path("randnum"))
        .and(warp::path::end())
        .and_then(get_random);

    let route = get_random;
    warp::serve(route).run(([127, 0, 0, 1], 1337)).await;
}
