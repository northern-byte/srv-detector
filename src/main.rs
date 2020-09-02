#![feature(async_closure)]

mod probes;
mod errors;
mod multi_spawn;

use serde_derive::{Deserialize, Serialize};
use warp::Filter;
use url::Url;
use warp::http::StatusCode;
use std::convert::Infallible;

#[derive(Deserialize, Serialize)]
struct Payload {
    domains: Vec<String>
}

#[tokio::main]
async fn main() {
    println!("Hello");
    let detect = warp::post().and(warp::path("detect")).and(warp::body::json()).and_then(handle);

    warp::serve(detect)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn to_urls(values: Vec<String>) -> Result<Vec<Url>, errors::InvalidUrlsError> {
    let mut invalid: Vec<String> = Vec::new();
    let valid = values.into_iter().filter_map(|v| {
        match Url::parse(&v) {
            Ok(t) => Some(t),
            _ => {
                invalid.push(v);
                None
            }
        }
    }).collect::<Vec<Url>>();

    if !invalid.is_empty() {
        return Err(errors::InvalidUrlsError::new(invalid));
    };

    Ok(valid)
}

async fn handle(payload: Payload) -> Result<impl warp::Reply, Infallible> {
    match to_urls(payload.domains) {
        Ok(result) => {
            let res = probes::probe(result).await;
            Ok(warp::reply::with_status(warp::reply::json(&res), StatusCode::OK))
        }
        Err(e) => Ok(warp::reply::with_status(warp::reply::json(&e), StatusCode::BAD_REQUEST))
    }
}