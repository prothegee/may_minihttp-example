use yarte::Serialize;

#[derive(Serialize)]
pub struct TMessage {
    pub message: &'static str
}