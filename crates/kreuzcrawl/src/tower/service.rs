//! Base HTTP fetch service (innermost in the Tower stack).

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use tower::Service;

use super::types::{CrawlRequest, CrawlResponse};
use crate::error::{CrawlError, classify_reqwest_error};
use crate::types::CrawlConfig;

/// Innermost Tower service that performs the actual HTTP fetch.
#[derive(Clone)]
pub struct HttpFetchService {
    client: reqwest::Client,
    config: Arc<CrawlConfig>,
}

impl HttpFetchService {
    pub fn new(client: reqwest::Client, config: CrawlConfig) -> Self {
        Self {
            client,
            config: Arc::new(config),
        }
    }
}

impl Service<CrawlRequest> for HttpFetchService {
    type Response = CrawlResponse;
    type Error = CrawlError;
    type Future = Pin<Box<dyn Future<Output = Result<CrawlResponse, CrawlError>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: CrawlRequest) -> Self::Future {
        let client = self.client.clone();
        let config = self.config.clone();

        Box::pin(async move {
            // Build reqwest request
            let mut http_req = client.get(&req.url);

            // Set user-agent (skip if request-level headers already provide one,
            // e.g. from the UaRotationLayer)
            if !req.headers.contains_key("user-agent") {
                if let Some(ref ua) = config.user_agent {
                    http_req = http_req.header(reqwest::header::USER_AGENT, ua.as_str());
                } else {
                    http_req = http_req.header(
                        reqwest::header::USER_AGENT,
                        concat!("kreuzcrawl/", env!("CARGO_PKG_VERSION")),
                    );
                }
            }

            // Auth
            if let Some(ref auth) = config.auth {
                match auth {
                    crate::types::AuthConfig::Basic { username, password } => {
                        http_req = http_req.basic_auth(username, Some(password));
                    }
                    crate::types::AuthConfig::Bearer { token } => {
                        http_req = http_req.bearer_auth(token);
                    }
                    crate::types::AuthConfig::Header { name, value } => {
                        http_req = http_req.header(name.as_str(), value.as_str());
                    }
                }
            }

            // Config custom headers
            for (k, v) in &config.custom_headers {
                http_req = http_req.header(k.as_str(), v.as_str());
            }

            // Request-level headers (from middleware layers)
            for (k, v) in &req.headers {
                http_req = http_req.header(k.as_str(), v.as_str());
            }

            // Send
            let resp = http_req
                .send()
                .await
                .map_err(|e| classify_reqwest_error(&e))?;

            let status = resp.status().as_u16();
            let content_type = resp
                .headers()
                .get_all(reqwest::header::CONTENT_TYPE)
                .iter()
                .next_back()
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_owned();

            // Extract headers into HashMap<String, String>
            let mut headers = HashMap::new();
            for (name, value) in resp.headers().iter() {
                if let Ok(v) = value.to_str() {
                    headers.insert(name.to_string(), v.to_string());
                }
            }

            // Check error status codes
            match status {
                401 => return Err(CrawlError::Unauthorized("unauthorized".into())),
                403 => {
                    let server = headers
                        .get("server")
                        .map(|s| s.to_lowercase())
                        .unwrap_or_default();
                    let body = resp.text().await.unwrap_or_default();
                    if crate::http::is_waf_blocked(&server, &body, &headers) {
                        let vendor = crate::http::detect_waf_vendor(&server, &body.to_lowercase());
                        return Err(CrawlError::WafBlocked(format!(
                            "waf/blocked detected: {vendor}"
                        )));
                    }
                    return Err(CrawlError::Forbidden("forbidden".into()));
                }
                404 => return Err(CrawlError::NotFound("not_found".into())),
                408 => return Err(CrawlError::Timeout("timeout".into())),
                410 => return Err(CrawlError::Gone("gone".into())),
                429 => return Err(CrawlError::RateLimited("rate_limited".into())),
                500 => return Err(CrawlError::ServerError("server_error".into())),
                502 => return Err(CrawlError::BadGateway("bad_gateway".into())),
                503 => {
                    return Err(CrawlError::ServerError("service unavailable".into()));
                }
                _ => {}
            }

            let body_bytes = resp.bytes().await.map_err(|e| classify_reqwest_error(&e))?;

            let body_vec = body_bytes.to_vec();
            let body = String::from_utf8_lossy(&body_vec).into_owned();

            Ok(CrawlResponse {
                status,
                content_type,
                body,
                body_bytes: body_vec,
                headers,
            })
        })
    }
}
