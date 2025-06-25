use std::{
    collections::{HashMap, VecDeque},
    io::Read,
    net::{IpAddr, ToSocketAddrs},
    sync::Mutex,
    time::{Duration, Instant},
};

use lazy_static::lazy_static;
use log::{info, warn};
use sha2::{Digest, Sha256};
use url::Url;
use ureq::{Agent, AgentBuilder};

/// Error types
#[derive(Debug)]
pub enum FetchError {
    InvalidUrl(String),
    DnsBlocked(String),
    DnsRebindingDetected,
    InsecureScheme,
    NetworkError(String),
    ReadError(String),
    DangerousContentType(String),
    CertificateMismatch,
    RateLimitExceeded,
}

/// HTTP Response
#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

/// Optional certificate fingerprint to validate TLS cert
const EXPECTED_CERT_SHA256: Option<&str> = None; // Example: Some("SHA256_HASH_BASE64")

/// Only allow specific headers to pass through
const ALLOWED_HEADERS: &[&str] = &["content-type", "content-length", "server"];

/// In-memory rate limiter
lazy_static! {
    static ref RATE_LIMITER: Mutex<VecDeque<Instant>> = Mutex::new(VecDeque::new());
}

const RATE_LIMIT_WINDOW: Duration = Duration::from_secs(10);
const MAX_REQUESTS_PER_WINDOW: usize = 10;

/// Secure GET request with all enhancements
pub fn fetch_url(url: &str, req_type: RequestType, enforce_https: bool) -> Result<HttpResponse, FetchError> {
    enforce_rate_limit()?;

    let parsed_url = validate_url(url, enforce_https)?;
    let initial_ips = resolve_ips(&parsed_url)?;

    if initial_ips.iter().any(is_blocked_ip) {
        return Err(FetchError::DnsBlocked(parsed_url.to_string()));
    }

    let agent = AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .redirects(0)
        .build();

    let mut request = agent.get(parsed_url.as_str());
    request.set("User-Agent", "secure-fetch/2.0");
    request.set("Accept", req_type.accept_header());

    let response = request.call().map_err(|e| FetchError::NetworkError(e.to_string()))?;

    let post_ips = resolve_ips(&parsed_url)?;
    if initial_ips != post_ips {
        return Err(FetchError::DnsRebindingDetected);
    }

    if let Some(cert_fingerprint) = EXPECTED_CERT_SHA256 {
        validate_cert(&response, cert_fingerprint)?;
    }

    let content_type = response
        .header("Content-Type")
        .unwrap_or("unknown")
        .to_lowercase();
    if is_dangerous_mime(&content_type) {
        return Err(FetchError::DangerousContentType(content_type));
    }

    let headers = response
        .headers_names()
        .iter()
        .filter_map(|k| {
            if ALLOWED_HEADERS.contains(&k.to_ascii_lowercase().as_str()) {
                response.header(k).map(|v| (k.to_string(), v.to_string()))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    let mut body = Vec::new();
    response
        .into_reader()
        .take(1_048_576)
        .read_to_end(&mut body)
        .map_err(|e| FetchError::ReadError(e.to_string()))?;

    info!("Fetched: {} -> Status: {}", url, response.status());

    Ok(HttpResponse {
        status: response.status(),
        headers,
        body,
    })
}

/// Enforce a basic per-process rate limit
fn enforce_rate_limit() -> Result<(), FetchError> {
    let mut limiter = RATE_LIMITER.lock().unwrap();
    let now = Instant::now();

    // Remove old entries
    while limiter.front().map_or(false, |t| now.duration_since(*t) > RATE_LIMIT_WINDOW) {
        limiter.pop_front();
    }

    if limiter.len() >= MAX_REQUESTS_PER_WINDOW {
        warn!("Rate limit exceeded");
        return Err(FetchError::RateLimitExceeded);
    }

    limiter.push_back(now);
    Ok(())
}

/// Validate URL
fn validate_url(url: &str, enforce_https: bool) -> Result<Url, FetchError> {
    let parsed = Url::parse(url).map_err(|e| FetchError::InvalidUrl(e.to_string()))?;
    let scheme = parsed.scheme();

    match scheme {
        "https" => Ok(parsed),
        "http" if !enforce_https => Ok(parsed),
        "http" => Err(FetchError::InsecureScheme),
        _ => Err(FetchError::InvalidUrl("Unsupported scheme".into())),
    }
}

/// Resolve IPs of URL host
fn resolve_ips(url: &Url) -> Result<Vec<IpAddr>, FetchError> {
    let host = url.host_str().ok_or(FetchError::InvalidUrl("Missing host".into()))?;
    let port = url.port_or_known_default().unwrap_or(80);
    (host, port)
        .to_socket_addrs()
        .map_err(|e| FetchError::DnsBlocked(e.to_string()))?
        .map(|addr| Ok(addr.ip()))
        .collect()
}

/// Block dangerous IPs
fn is_blocked_ip(ip: &IpAddr) -> bool {
    ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local()
        || ip.is_multicast()
}

/// Detect dangerous MIME types
fn is_dangerous_mime(mime: &str) -> bool {
    mime.contains("application/x-msdownload")
        || mime.contains("application/octet-stream")
        || mime.contains("application/x-sh")
        || mime.contains("text/x-script")
}

/// Validate server TLS certificate fingerprint
fn validate_cert(response: &ureq::Response, expected_fingerprint: &str) -> Result<(), FetchError> {
    use base64::{engine::general_purpose, Engine as _};
    if let Some(cert_der) = response.synthetic_certificate() {
        let mut hasher = Sha256::new();
        hasher.update(cert_der);
        let result = hasher.finalize();
        let actual = general_purpose::STANDARD.encode(result);

        if actual != expected_fingerprint {
            return Err(FetchError::CertificateMismatch);
        }
    }
    Ok(())
}
