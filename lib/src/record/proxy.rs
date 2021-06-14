use std::{net::SocketAddr, time::Duration};

use log::info;
use warp::{Filter, filters::{host::Authority, path::FullPath}, http::HeaderMap, hyper::body::Bytes, Rejection, Reply};
use warp_reverse_proxy::{extract_request_data_filter, Method, proxy_to_and_forward_response, QueryParameters};

use crate::record::{port::PortAllocator, RecordInput};

use super::{
    config::RecordConfig,
    http::RecordedExchange,
    warp_exchange::{WarpExchange, WarpRequest, WarpResponse},
};

pub struct Proxy;

impl Proxy {
    pub(crate) fn run(cfg: RecordConfig, then: fn(RecordInput)) -> SocketAddr {
        let addr = PortAllocator::new_binding(cfg.port);
        tokio::spawn(async move {
            warp::serve(warp::any().and(Self::forward_and_record(cfg, then).boxed()))
                .run(addr).await;
        });
        // give some time to warp server to spawn
        // TODO: try awaiting
        std::thread::sleep(Duration::from_millis(100));
        info!("Started stubr recorder on {}", addr);
        addr
    }

    fn forward_and_record(cfg: RecordConfig, then: fn(RecordInput)) -> impl Filter<Extract=(impl Reply, ), Error=Rejection> {
        Self::host()
            .and(warp::any().map(String::new))
            .and(extract_request_data_filter())
            .and_then(Self::proxy)
            .and_then(move |exchange| Self::reply(exchange, cfg.clone(), then))
    }

    fn host() -> impl Filter<Extract=(String, ), Error=Rejection> {
        warp::filters::host::optional()
            .map(|authority: Option<Authority>| authority.map(Self::base_uri).unwrap_or_default())
    }

    fn base_uri(a: Authority) -> String {
        format!("http://{}:{}/", a.host(), a.port_u16().unwrap_or(80))
    }

    async fn proxy(
        addr: String,
        base_path: String,
        uri: FullPath,
        queries: QueryParameters,
        method: Method,
        headers: HeaderMap,
        body: Bytes,
    ) -> Result<RecordedExchange, Rejection> {
        let path = uri.as_str().to_string();
        proxy_to_and_forward_response(addr.clone(), base_path, uri, queries.clone(), method.clone(), headers.clone(), body.clone()).await
            .map(move |resp| {
                let req = WarpRequest { method, addr, path, queries, headers, body };
                let resp = WarpResponse(resp);
                WarpExchange(req, resp).into()
            })
    }

    async fn reply(mut exchange: RecordedExchange, cfg: RecordConfig, then: fn(RecordInput)) -> Result<impl Reply, Rejection> {
        then((&mut exchange, cfg));
        Ok(exchange.1)
    }
}