use url::{Url};
use std::collections::HashMap;
use std::convert::Infallible;

async fn get_headers(url: Url) -> Option<HashMap<String, String>> {
    let res = reqwest::Client::new().head(url.as_str()).send().await;

    match res {
        Ok(r) => {
            let hash_map = r.headers().iter()
                .map(|p| (p.0.to_string(), p.1.to_str().unwrap_or_default().to_string()))
                .collect::<HashMap<String, String>>();
            Some(hash_map)
        }
        Err(_) => None
    }
}

pub async fn probe(urls: Vec<Url>) -> Result<Vec<HashMap<String, String>>, Infallible> {
    let tasks = urls.iter().map(|u| tokio::spawn(get_headers(u.to_owned()))).collect::<Vec<_>>();
    let res = futures::future::join_all(tasks).await;

    let filtered: Vec<HashMap<String, String>> = res.into_iter().flatten()
        .filter_map(|p| p)
        .collect();

    Ok(filtered)
}