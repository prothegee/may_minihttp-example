#![allow(dead_code)]

/// module for http content-type constant
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

/// module for http method constant
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

//// module for http status code constant
///
/// ref: https://en.wikipedia.org/wiki/List_of_HTTP_status_codes
pub mod status_code {
    use crate::types::status::TStatusCodeResponse;

    // 1xx: information response
    pub const STATUS_100_CONTINUE: TStatusCodeResponse
        = TStatusCodeResponse { code: 100, msg: "Continue" };
    pub const STATUS_101_SWITCHING_PROTOCOL: TStatusCodeResponse
        = TStatusCodeResponse { code: 101, msg: "Switching Protocols" };
    pub const STATUS_102_PROCESSING: TStatusCodeResponse
        = TStatusCodeResponse { code: 102, msg: "Processing" };
    pub const STATUS_103_EARLY_HINT: TStatusCodeResponse
        = TStatusCodeResponse { code: 103, msg: "Early Hints" };

    // 2xx: success
    pub const STATUS_200_OK: TStatusCodeResponse
        = TStatusCodeResponse { code: 200, msg: "Ok" };
    pub const STATUS_201_CREATED: TStatusCodeResponse
        = TStatusCodeResponse { code: 201, msg: "Created" };
    pub const STATUS_202_ACCEPTED: TStatusCodeResponse
        = TStatusCodeResponse { code: 202, msg: "Accepted" };
    pub const STATUS_203_NON_AUTHORITATIVE_INFORMATION: TStatusCodeResponse
        = TStatusCodeResponse { code: 203, msg: "Non-Authoritative Information" };
    pub const STATUS_204_NO_CONTENT: TStatusCodeResponse
        = TStatusCodeResponse { code: 204, msg: "No Content" };
    pub const STATUS_205_RESET_CONTENT: TStatusCodeResponse
        = TStatusCodeResponse { code: 205, msg: "Reset Content" };
    pub const STATUS_206_PARTIAL_CONTENT: TStatusCodeResponse
        = TStatusCodeResponse { code: 206, msg: "Partial Content" };
    pub const STATUS_207_MULTI_STATUS: TStatusCodeResponse
        = TStatusCodeResponse { code: 207, msg: "Multi-Status" };
    pub const STATUS_208_ALREADY_REPORTED: TStatusCodeResponse
        = TStatusCodeResponse { code: 208, msg: "Already Reported" };
    pub const STATUS_226_IM_USED: TStatusCodeResponse
        = TStatusCodeResponse { code: 226, msg: "IM Used" };

    // 3xx: redirection
    pub const STATUS_300_MULTIPLE_CHOICES: TStatusCodeResponse
        = TStatusCodeResponse { code: 300, msg: "Multiple Choices" };
    pub const STATUS_301_MOVED_PERMANENTLY: TStatusCodeResponse
        = TStatusCodeResponse { code: 301, msg: "Moved Permanently" };
    pub const STATUS_302_FOUND: TStatusCodeResponse
        = TStatusCodeResponse { code: 302, msg: "Found" };
    pub const STATUS_303_SEE_OTHER: TStatusCodeResponse
        = TStatusCodeResponse { code: 303, msg: "See Other" };
    pub const STATUS_304_NOT_MODIFIED: TStatusCodeResponse
        = TStatusCodeResponse { code: 304, msg: "Not Modified" };
    pub const STATUS_305_USE_PROXY: TStatusCodeResponse
        = TStatusCodeResponse { code: 305, msg: "Use Proxy" };
    pub const STATUS_307_TEMPORARY_REDIRECT: TStatusCodeResponse
        = TStatusCodeResponse { code: 307, msg: "Temporary Redirect" };
    pub const STATUS_308_PERMANENT_REDIRECT: TStatusCodeResponse
        = TStatusCodeResponse { code: 308, msg: "Permanent Redirect" };

    // 4xx: client errors
    pub const STATUS_400_BAD_REQUEST: TStatusCodeResponse
        = TStatusCodeResponse { code: 400, msg: "Bad Request" };
    pub const STATUS_401_UNAUTHORIZED: TStatusCodeResponse
        = TStatusCodeResponse { code: 401, msg: "Unauthorized" };
    pub const STATUS_402_PAYMENT_REQUIRED: TStatusCodeResponse
        = TStatusCodeResponse { code: 402, msg: "Payment Required" };
    pub const STATUS_403_FORBIDDEN: TStatusCodeResponse
        = TStatusCodeResponse { code: 403, msg: "Forbidden" };
    pub const STATUS_404_NOT_FOUND: TStatusCodeResponse
        = TStatusCodeResponse { code: 404, msg: "Not Found" };
    pub const STATUS_405_METHOD_NOT_ALLOWED: TStatusCodeResponse
        = TStatusCodeResponse { code: 405, msg: "Method Not Allowed" };
    pub const STATUS_406_NOT_ACCEPTABLE: TStatusCodeResponse
        = TStatusCodeResponse { code: 406, msg: "Not Acceptable" };
    pub const STATUS_407_PROXY_AUTHENTICATION_REQUIRED: TStatusCodeResponse
        = TStatusCodeResponse { code: 407, msg: "Proxy Authentication Required" };
    pub const STATUS_408_REQUEST_TIMEOUT: TStatusCodeResponse
        = TStatusCodeResponse { code: 408, msg: "Request Timeout" };
    pub const STATUS_409_CONFLICT: TStatusCodeResponse
        = TStatusCodeResponse { code: 409, msg: "Conflict" };
    pub const STATUS_410_GONE: TStatusCodeResponse
        = TStatusCodeResponse { code: 410, msg: "Gone" };
    pub const STATUS_411_LENGTH_REQUIRED: TStatusCodeResponse
        = TStatusCodeResponse { code: 411, msg: "Length Required" };
    pub const STATUS_412_PRECONDITION_FAILED: TStatusCodeResponse
        = TStatusCodeResponse { code: 412, msg: "Precondition Failed" };
    pub const STATUS_413_PAYLOAD_TOO_LARGE: TStatusCodeResponse
        = TStatusCodeResponse { code: 413, msg: "Payload Too Large" };
    pub const STATUS_414_URI_TOO_LONG: TStatusCodeResponse
        = TStatusCodeResponse { code: 414, msg: "URI Too Long" };
    pub const STATUS_415_UNSUPPORTED_MEDIA_TYPE: TStatusCodeResponse
        = TStatusCodeResponse { code: 415, msg: "Unsupported Media Type" };
    pub const STATUS_416_RANGE_NOT_SATISFIABLE: TStatusCodeResponse
        = TStatusCodeResponse { code: 416, msg: "Range Not Satisfiable" };
    pub const STATUS_417_EXPECTATION_FAILED: TStatusCodeResponse
        = TStatusCodeResponse { code: 417, msg: "Expectation Failed" };
    pub const STATUS_418_IM_A_TEAPOT: TStatusCodeResponse
        = TStatusCodeResponse { code: 418, msg: "I'm a teapot" };
    pub const STATUS_421_MISDIRECTED_REQUEST: TStatusCodeResponse
        = TStatusCodeResponse { code: 421, msg: "Misdirected Request" };
    pub const STATUS_422_UNPROCESSABLE_CONTENT: TStatusCodeResponse
        = TStatusCodeResponse { code: 422, msg: "Unprocessable Content" };
    pub const STATUS_423_LOCKED: TStatusCodeResponse
        = TStatusCodeResponse { code: 423, msg: "Locked" };
    pub const STATUS_424_FAILED_DEPENDENCY: TStatusCodeResponse
        = TStatusCodeResponse { code: 424, msg: "Failed Dependency" };
    pub const STATUS_425_TOO_EARLY: TStatusCodeResponse
        = TStatusCodeResponse { code: 425, msg: "Too Early" };
    pub const STATUS_426_UPGRADE_REQUIRED: TStatusCodeResponse
        = TStatusCodeResponse { code: 426, msg: "Upgrade Required" };
    pub const STATUS_428_PRECONDITION_REQUIRED: TStatusCodeResponse
        = TStatusCodeResponse { code: 428, msg: "Precondition Required" };
    pub const STATUS_429_TOO_MANY_REQUESTS: TStatusCodeResponse
        = TStatusCodeResponse { code: 429, msg: "Too Many Requests" };
    pub const STATUS_431_REQUEST_HEADER_FIELDS_TOO_LARGE: TStatusCodeResponse
        = TStatusCodeResponse { code: 431, msg: "Request Header Fields Too Large" };
    pub const STATUS_451_UNAVAILABLE_FOR_LEGAL_REASONS: TStatusCodeResponse
        = TStatusCodeResponse { code: 451, msg: "Unavailable For Legal Reasons" };

    // 5xx: server errors
    pub const STATUS_500_INTERNAL_SERVER_ERROR: TStatusCodeResponse
        = TStatusCodeResponse { code: 500, msg: "Internal Server Error" };
    pub const STATUS_501_NOT_IMPLEMENTED: TStatusCodeResponse
        = TStatusCodeResponse { code: 501, msg: "Not Implemented" };
    pub const STATUS_502_BAD_GATEWAY: TStatusCodeResponse
        = TStatusCodeResponse { code: 502, msg: "Bad Gateway" };
    pub const STATUS_503_SERVICE_UNAVAILABLE: TStatusCodeResponse
        = TStatusCodeResponse { code: 503, msg: "Service Unavailable" };
    pub const STATUS_504_GATEWAY_TIMEOUT: TStatusCodeResponse
        = TStatusCodeResponse { code: 504, msg: "Gateway Timeout" };
    pub const STATUS_505_HTTP_VERSION_NOT_SUPPORTED: TStatusCodeResponse
        = TStatusCodeResponse { code: 505, msg: "HTTP Version Not Supported" };
    pub const STATUS_506_VARIANT_ALSO_NEGOTIATES: TStatusCodeResponse
        = TStatusCodeResponse { code: 506, msg: "Variant Also Negotiates" };
    pub const STATUS_507_INSUFFICIENT_STORAGE: TStatusCodeResponse
        = TStatusCodeResponse { code: 507, msg: "Insufficient Storage" };
    pub const STATUS_508_LOOP_DETECTED: TStatusCodeResponse
        = TStatusCodeResponse { code: 508, msg: "Loop Detected" };
    pub const STATUS_510_NOT_EXTENDED: TStatusCodeResponse
        = TStatusCodeResponse { code: 510, msg: "Not Extended" };
    pub const STATUS_511_NETWORK_AUTHENTICATION_REQUIRED: TStatusCodeResponse
        = TStatusCodeResponse { code: 511, msg: "Network Authentication Required" };
}
