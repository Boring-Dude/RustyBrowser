//! Networking module for downloading HTML, CSS, images, etc.
//! Currently supports simple blocking HTTP GET Requests.

pub mod http;
pub mod fetch;
pub mod requests;

// Re-export types for external convenience
pub use http::{fetch_url, HttpResponse, FetchError};
pub use fetch::{fetch_resource, fetch_html, FetchResult, ResourceType};
pub use requests::{RequestType};
