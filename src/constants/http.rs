#![allow(dead_code)]

/// module for http method constants
pub mod method {
    pub const GET: &str = "GET";
    pub const HEAD: &str = "HEAD";
    pub const POST: &str = "POST";
    pub const PUT: &str = "PUT";
    pub const DELETE: &str = "DELETE";
    pub const CONNECT: &str = "CONNECT";
    pub const OPTIONS: &str = "OPTIONS";
    pub const TRACE: &str = "TRACE";
    pub const PATCH: &str = "PATCH";
}

/// module for http content-type
pub mod content_type {
    /// application/json
    pub const APPLICATION_JSON: &str = "application/json";
    /// content-type: application/json
    pub const CT_APPLICATION_JSON: &str = "content-type: application/json";

    /// application/x-www-form-urlencoded
    pub const APPLICATION_FORM_URLENCODED: &str = "application/x-www-form-urlencoded";
    /// content-type: pplication/x-www-form-urlencoded
    pub const CT_APPLICATION_FORM_URLENCODED: &str = "content-type: pplication/x-www-form-urlencoded";

    /// multipart/form-data
    pub const MULTIPART_FORM_DATA: &str = "multipart/form-data";
    /// content-type: multipart/form-data
    pub const CT_MULTIPART_FORM_DATA: &str = "content-type: multipart/form-data";

    /// text/plain
    pub const TEXT_PLAIN: &str = "text/plain";
    /// content-type: text/plain
    pub const CT_TEXT_PLAIN: &str = "content-type: text/plain";

    /// text/html
    pub const TEXT_HTML: &str = "text/html";
    /// content-type: text/html
    pub const CT_TEXT_HTML: &str = "content-type: text/html";

    /// application/octet-stream
    pub const APPLICATION_OCTET_STREAM: &str = "application/octet-stream";
    /// content-type: application/octet-stream
    pub const CT_APPLICATION_OCTET_STREAM: &str = "content-type: application/octet-stream";
}