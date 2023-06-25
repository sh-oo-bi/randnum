use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;
use warp::{http::StatusCode, reject::Reject, Filter, Rejection, Reply};

#[derive(Debug, Serialize)]
struct Randnum {
    num: i32,
}

#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    BadRange,
}

impl Reject for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
            Error::BadRange => write!(f, "BadRange"),
        }
    }
}

impl Randnum {
    fn new(start: Option<i32>, end: Option<i32>) -> Result<Self, Rejection> {
        let start = match start {
            Some(n) => n,
            _ => 1,
        };

        let end = match end {
            Some(n) => n,
            _ => 1000,
        };

        if start <= 0 || end <= 0 || end < start {
            return Err(warp::reject::custom(Error::BadRange));
        }

        let random = {
            let mut rng = rand::thread_rng();
            rng.gen_range(start..=end)
        };

        Ok(Self { num: random })
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

async fn get_random(params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
    let start = match params.get("start") {
        Some(n) => Some(n.parse::<i32>().map_err(Error::ParseError)?),
        None => None,
    };

    let end = match params.get("end") {
        Some(n) => Some(n.parse::<i32>().map_err(Error::ParseError)?),
        None => None,
    };

    let random_number = Randnum::new(start, end)?;

    Ok(warp::reply::json(&random_number))
}

#[tokio::main]
async fn main() {
    let get_random = warp::get()
        .and(warp::path("randnum"))
        .and(warp::path::end())
        .and(warp::query())
        .and_then(get_random);

    let routes = get_random.recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 1337)).await;
}
