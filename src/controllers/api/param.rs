use std::collections::HashMap;

use may_minihttp::{Request, Response};
use url::form_urlencoded;
use yarte::Serialize;
use crate::constants::http;

#[derive(Serialize)]
struct TParamMessage {
    message: String
}

pub fn message(req: Request, resp: &mut Response) {
    resp.header(http::content_type::CT_APPLICATION_JSON);

    // skip all method, it's allow

    let query = req.path().splitn(2, '?').nth(1).unwrap_or("doesn't exists");

    let params: HashMap<String, String> = form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect();

    let message = params.get("message").map(|s| s.as_str()).unwrap_or("default").to_string();

    let callback_data = TParamMessage {
        message: format!("{}, {}", req.method(), message)
    };

    callback_data.to_bytes_mut(resp.body_mut());

    resp.status_code(200, "ok");
}