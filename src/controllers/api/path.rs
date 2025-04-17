use may_minihttp::{Request, Response};
use regex::Regex;
use yarte::Serialize;
use crate::constants::http;

#[derive(Serialize)]
struct TTwoPathsRes {
    path1: String,
    path2: String
}

pub fn handle_path(req: Request, resp: &mut Response) {
    resp.header(http::content_type::CT_APPLICATION_JSON);

    let mut path1: String = "".to_string();
    let mut path2: String = "".to_string();

    let raw = req.path().splitn(2, '?').next().unwrap_or("").to_string();
    let re = Regex::new(r"^/api/path/([^/]+)/([^/]+)$").unwrap();
    if let Some(caps) = re.captures(&raw) {
        path1 = caps.get(1).unwrap().as_str().to_string();
        path2 = caps.get(2).unwrap().as_str().to_string();
    }

    // path1 & path2 will always string
    // do something else if want to check it's numeric or not

    // sanity check
    // move to util?
    if !path1.chars().all(|c| c.is_alphanumeric()) {
        resp.status_code(400, "bad request")
            .body(r#"{"error":"path1 must be alphanumeric"}"#);
        return;
    }
    if path2.len() < 3 {
        resp.status_code(400, "bad request")
            .body(r#"{"error":"path2 too short"}"#);
        return;
    }

    let callback_data = TTwoPathsRes {
        path1: path1,
        path2: path2,
    };

    callback_data.to_bytes_mut(resp.body_mut());

    resp.status_code(200, "ok");
}
