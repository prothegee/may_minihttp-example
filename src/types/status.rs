use yarte::Serialize;

#[derive(Serialize)]
pub struct TStatusConfig {
    pub status: i8,
    pub version: &'static str,
    pub release_date: &'static str,
}

#[allow(dead_code)]
pub struct TStatusCodeResponse {
    pub code: usize,
    pub msg: &'static str
}