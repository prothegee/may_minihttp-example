use yarte::Serialize;
use may_minihttp::{Request, Response};

use crate::{constants::http, types::message::TMessage};

#[allow(non_snake_case, unused_variables)]
pub fn hello(req: Request, resp: &mut Response) {
    resp.header(http::content_type::CT_APPLICATION_JSON);

    // maybe the's a condition where you want to use multiple methods in single endpoint
    match req.method() {
        http::method::GET => {
            let callbackData = TMessage {
                message: "hello"
            };

            callbackData.to_bytes_mut(resp.body_mut());
        }
        _ => {
            resp.status_code(405, "not allowed");
        }
    }
}