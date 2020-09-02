use url::{Url};
use std::collections::HashMap;
use std::convert::Infallible;
use std::vec::IntoIter;
use async_std::net::{SocketAddr, ToSocketAddrs};
use crate::multi_spawn::MultiSpawn;
use std::sync::Arc;
use std::ops::Deref;

pub async fn probe(urls: Vec<Url>) -> Result<Vec<HashMap<String, String>>, Infallible> {
    let shared_urls = urls.into_iter().map(Arc::new).collect::<Vec<Arc<Url>>>();
    let res = shared_urls.iter().spawn_and_join(get_headers).await;

    let filtered: Vec<HashMap<String, String>> = res.into_iter().flatten()
        .filter_map(|p| p)
        .collect();

    Ok(filtered)
}

async fn get_headers<U: Deref<Target=Url>>(url: U) -> Option<HashMap<String, String>> {
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

async fn resolve_host<U: Deref<Target=Url>>(url: U) -> Result<IntoIter<SocketAddr>, std::io::Error> {
    let target: String;

    if url.port().is_none() {
        target = format!("{host}:{port}", host = url.host_str().expect("host is not present"), port = "80")
    } else {
        target = url.to_string();
    }

    let addr = target.to_socket_addrs().await?;
    Ok(addr)
}

mod probe_tests {
    use futures_await_test::async_test;
    use crate::probes::resolve_host;
    use std::sync::Arc;

    #[async_test]
    async fn test_ex() {
        use url::Url;

        let result = Url::parse("http://detectify.com");

        let addr = resolve_host(Arc::new(result.unwrap())).await.unwrap().next().unwrap();
        println!("{}", addr)
    }
}