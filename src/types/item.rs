use yarte::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct TItem {
    pub item_name: String,
    pub item_image: String,
    pub quantity: i32
}

////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct TItemSubmissionData {
    pub items: Vec<TItem>
}