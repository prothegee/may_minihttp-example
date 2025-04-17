#![allow(dead_code)]

use yarte::Serialize;

#[derive(Serialize)]
pub struct TMessage {
    pub message: &'static str
}

#[derive(Serialize)]
pub struct TMessageOk {
    pub ok: bool,
    pub message: &'static str
}