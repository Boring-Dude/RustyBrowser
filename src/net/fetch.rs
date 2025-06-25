//! fetch.rs — Secure, resource-type aware fetching for HTML, CSS, images, etc.

use crate::net::{http::fetch_url, requests::RequestType};
use crate::utils::logger::log;
use std::collections::HashMap;
use std::time::Instant;

/// Types of web resources
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceType {
    Html,
    Css,
    Image,
    Script,
    Json,
    Xml,
    Font,
    Other,
}

/// Result of a fetch request
pub struct FetchResult {
    pub url: String,
    pub content_type: String,
    pub data: Vec<u8>,
    pub resource_type: ResourceType,
}

/// Custom error for fetch logic
#[derive(Debug)]
pub enum FetchError {
    Network(String),
    ContentTypeMismatch(String),
    DecodeError(String),
    UnexpectedType(String),
}

/// Fetch a resource and classify it by MIME type
pub fn fetch_resource(url: &str, req_type: RequestType) -> Result<FetchResult, FetchError> {
    log(&format!("Fetching {:?} from {}", req_type, url));
    let start = Instant::now();

    let response = fetch_url(url, req_type).map_err(FetchError::Network)?;

    let content_type = response
        .headers
        .get("Content-Type")
        .cloned()
        .unwrap_or_else(|| "application/octet-stream".into());

    let resource_type = detect_type(&content_type);

    log(&format!(
        "Received: {} [{}] in {:?}",
        url, content_type, start.elapsed()
    ));

    Ok(FetchResult {
        url: url.to_string(),
        content_type,
        data: response.body,
        resource_type,
    })
}

/// Determine the resource type based on Content-Type
pub fn detect_type(content_type: &str) -> ResourceType {
    let ct = content_type.to_ascii_lowercase();
    match ct.as_str() {
        ct if ct.contains("text/html") => ResourceType::Html,
        ct if ct.contains("text/css") => ResourceType::Css,
        ct if ct.contains("javascript") || ct.contains("ecmascript") => ResourceType::Script,
        ct if ct.starts_with("image/") => ResourceType::Image,
        ct if ct.contains("application/json") => ResourceType::Json,
        ct if ct.contains("application/xml") || ct.contains("text/xml") => ResourceType::Xml,
        ct if ct.contains("font/") || ct.contains("woff") || ct.contains("truetype") => ResourceType::Font,
        _ => ResourceType::Other,
    }
}

/// Fetch and return HTML as a UTF-8 string (with basic encoding fallback)
pub fn fetch_html(url: &str) -> Result<String, FetchError> {
    let result = fetch_resource(url, RequestType::Document)?;

    if result.resource_type != ResourceType::Html {
        return Err(FetchError::ContentTypeMismatch(result.content_type));
    }

    String::from_utf8(result.data.clone()).or_else(|_| {
        let fallback = result.data.iter().map(|&b| b as char).collect::<String>();
        if fallback.is_empty() {
            Err(FetchError::DecodeError("Empty HTML content".into()))
        } else {
            Ok(fallback)
        }
    })
}
