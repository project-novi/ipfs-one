use axum::{
    body::Body,
    extract::{Request, State},
    response::{IntoResponse, Response},
};
use futures::{stream::FuturesUnordered, StreamExt};
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::warn;

pub struct AppState {
    pub gateways: Vec<String>,
}

pub fn make_state(gateways: Vec<String>) -> Arc<AppState> {
    Arc::new(AppState { gateways })
}

pub async fn entry(state: State<Arc<AppState>>, request: Request) -> Response {
    let uri = request.uri();
    let path = uri.path();
    let Some(_content) = path.strip_prefix("/ipfs/") else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let mut headers = request.headers().clone();
    headers.remove("Host");

    let client = reqwest::Client::new();
    let mut tasks: FuturesUnordered<_> = state
        .gateways
        .iter()
        .map(|gateway| {
            let client = &client;
            let headers = headers.clone();
            let gateway = gateway.clone();
            async move {
                client
                    .get(&format!("{gateway}{path}"))
                    .headers(headers)
                    .send()
                    .await
                    .and_then(|it| it.error_for_status())
                    .map_err(|err| (err, gateway))
            }
        })
        .collect();

    // TODO: cache which gateway is available for this CID
    while let Some(result) = tasks.next().await {
        match result {
            Ok(resp) => {
                return (
                    resp.headers().clone(),
                    Body::from_stream(resp.bytes_stream()),
                )
                    .into_response();
            }
            Err((err, gateway)) => {
                warn!(gateway, "error fetching from gateway: {err}");
            }
        }
    }

    StatusCode::NOT_FOUND.into_response()
}
