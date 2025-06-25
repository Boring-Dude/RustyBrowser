//! Resource request classification and helpers

/// Type of resource being requested
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequestType {
    Document,
    Stylesheet,
    Script,
    Image,
    Font,
    Fetch,
    Other,
}

impl RequestType {
    /// Suggest an appropriate Accept header for the request type
    pub fn accept_header(self) -> &'static str {
        match self {
            RequestType::Document => "text/html,application/xhtml+xml",
            RequestType::Stylesheet => "text/css,*/*;q=0.1",
            RequestType::Script => "application/javascript,application/ecmascript",
            RequestType::Image => "image/webp,image/apng,image/*,*/*;q=0.8",
            RequestType::Font => "font/woff2,application/font-woff;q=0.9,*/*;q=0.1",
            RequestType::Fetch => "application/json,text/plain,*/*",
            RequestType::Other => "*/*",
        }
    }

    /// Determines if this type should be preloaded early
    pub fn is_preloadable(self) -> bool {
        matches!(
            self,
            RequestType::Stylesheet | RequestType::Script | RequestType::Font
        )
    }
}

impl std::fmt::Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RequestType::Document => "Document",
            RequestType::Stylesheet => "Stylesheet",
            RequestType::Script => "Script",
            RequestType::Image => "Image",
            RequestType::Font => "Font",
            RequestType::Fetch => "Fetch",
            RequestType::Other => "Other",
        };
        write!(f, "{}", s)
    }
}
