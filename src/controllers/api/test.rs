use yarte::Serialize;
use may_minihttp::{Request, Response};

use crate::{constants::http, types::message::TMessage};

pub fn hello(req: Request, resp: &mut Response) {
    resp.header(http::content_type::CT_APPLICATION_JSON);

    // maybe the's a condition where you want to use multiple methods in single endpoint
    match req.method() {
        http::method::GET => {
            let callback = TMessage {
                message: "hello"
            };

            callback.to_bytes_mut(resp.body_mut());
            resp.status_code(200, "ok");
        }
        _ => {
            resp.status_code(405, "not allowed");
        }
    }
}