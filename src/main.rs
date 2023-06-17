use rand::Rng;
use warp::{http::Response, Filter, Rejection};

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

impl warp::Reply for Randnum {
    fn into_response(self) -> warp::reply::Response {
        Response::new(format!("random number is : {}", self.num.to_string()).into())
    }
}

async fn get_random() -> Result<Randnum, Rejection> {
    let random_number = Randnum::new();
    Ok(random_number)
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
