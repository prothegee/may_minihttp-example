use yarte::Serialize;

#[derive(Serialize)]
pub struct TStatusConfig {
    pub status: i8,
    pub version: &'static str,
    pub release_date: &'static str,
}