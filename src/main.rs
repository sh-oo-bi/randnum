use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;
use warp::{Filter, Rejection};

#[derive(Debug, Serialize)]
struct Randnum {
    num: i32,
}

impl Randnum {
    fn new(start: Option<i32>, end: Option<i32>) -> Self {
        let start = match start {
            Some(n) => n,
            _ => 1,
        };

        let end = match end {
            Some(n) => n,
            _ => 1000,
        };

        let random = {
            let mut rng = rand::thread_rng();
            rng.gen_range(start..=end)
        };

        Self { num: random }
    }
}

async fn get_random(params: HashMap<String, String>) -> Result<impl warp::Reply, Rejection> {
    let start = match params.get("start") {
        Some(n) => Some(n.parse::<i32>().expect("Could not parse start")),
        None => None,
    };

    let end = match params.get("end") {
        Some(n) => Some(n.parse::<i32>().expect("Could not parse end")),
        None => None,
    };

    println!("start:{:?}, end:{:?}", start, end);

    let random_number = Randnum::new(start, end);

    Ok(warp::reply::json(&random_number))
}

#[tokio::main]
async fn main() {
    let get_random = warp::get()
        .and(warp::path("randnum"))
        .and(warp::path::end())
        .and(warp::query())
        .and_then(get_random);

    let route = get_random;
    warp::serve(route).run(([127, 0, 0, 1], 1337)).await;
}
