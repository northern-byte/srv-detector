#![feature(async_closure)]

mod probes;
mod errors;

use serde_derive::{Deserialize, Serialize};
use warp::{Filter, Rejection, reject};
use url::Url;
use crate::errors::InvalidUrlsError;

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

fn to_urls(values: &[String]) -> Result<Vec<Url>, errors::InvalidUrlsError> {
    let valid = values.iter().filter_map(|v| Url::parse(v).ok()).collect::<Vec<Url>>();

    if valid.len() != values.len() {
        return Err(errors::InvalidUrlsError::new());
    };

    Ok(valid)
}

impl warp::reject::Reject for InvalidUrlsError {}

async fn handle(payload: Payload) -> std::result::Result<impl warp::Reply, Rejection> {
    match to_urls(payload.domains.as_slice()) {
        Ok(result) => {
            let res = probes::probe(result).await;
            Ok(warp::reply::json(&res.unwrap()))
        }
        Err(e) => Err(reject::custom(e))
    }
}