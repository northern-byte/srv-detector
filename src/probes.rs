use url::{Url};
use std::collections::HashMap;
use async_std::net::{ToSocketAddrs, IpAddr};
use crate::multi_spawn::MultiSpawn;
use std::sync::Arc;
use std::ops::Deref;
use std::error::Error;
use serde_derive::{Serialize};

#[derive(Serialize)]
enum Header {
    Server(String),
    ContentLength(String),
}

#[derive(Serialize)]
pub struct ProbeResult {
    headers: Vec<Header>,
    ips: Vec<IpAddr>,
}

pub async fn probe(urls: Vec<Url>) -> HashMap<String, ProbeResult> {
    let mut probe_results: HashMap<String, ProbeResult> = urls.iter().map(|u| (u.as_str().to_string(), ProbeResult { headers: vec![], ips: vec![] })).collect();

    let shared_urls = urls.into_iter().map(Arc::new).collect::<Vec<Arc<Url>>>();
    let headers = shared_urls.iter().spawn_and_join(get_headers).await;
    let addresses = shared_urls.iter().spawn_and_join(resolve_host).await;

    headers.into_iter().filter(|r| r.is_ok()).map(|r| r.unwrap()).filter(|r| r.is_ok()).for_each(|r| {
        let t = r.unwrap();
        probe_results.insert(t.0, ProbeResult { headers: t.1, ips: vec![] });
    });

    addresses.into_iter().filter(|r| r.is_ok()).map(|r| r.unwrap()).filter(|r| r.is_ok()).for_each(|r| {
        let s = r.unwrap();

        if let Some(mut pr) = probe_results.get_mut(s.0.as_str()) {
            pr.ips = s.1;
        };
    });


    probe_results
}

async fn get_headers<U: Deref<Target=Url>>(url: U) -> Result<(String, Vec<Header>), impl Error> {
    let url_str = url.as_str().to_string();
    let res = reqwest::Client::new().head(&url_str).send().await;

    match res {
        Ok(r) => {
            let headers = r.headers().iter()
                .map(|p| {
                    match p.0.as_str() {
                        "server" => Some(Header::Server(p.1.to_str().unwrap_or_default().to_string())),
                        "content-length" => Some(Header::ContentLength(p.1.to_str().unwrap_or_default().to_string())),
                        _ => None
                    }
                }).filter_map(|h| h).collect::<Vec<Header>>();

            Ok((url_str, headers))
        }
        Err(e) => Err(e)
    }
}

async fn resolve_host<U: Deref<Target=Url>>(url: U) -> Result<(String, Vec<IpAddr>), std::io::Error> {
    let target: String;

    if url.port().is_none() {
        target = format!("{host}:{port}", host = url.host_str().expect("host is not present"), port = "80")
    } else {
        target = url.to_string();
    }

    let addr = target.to_socket_addrs().await?;
    Ok((url.as_str().to_string(), addr.map(|a| a.ip()).collect()))
}

mod probe_tests {
    use futures_await_test::async_test;
    use crate::probes::resolve_host;
    use std::sync::Arc;

    #[async_test]
    async fn test_resolve_host() {
        use url::Url;

        let result = Url::parse("http://detectify.com");

        let (_, addr) = resolve_host(Arc::new(result.unwrap())).await.unwrap();
        assert_ne!(addr.len(), 0);
    }
}