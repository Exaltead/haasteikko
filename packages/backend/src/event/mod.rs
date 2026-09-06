use serde::{Deserialize, Serialize};

mod api;
mod domain;
mod repository;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub user_id: String,
    pub item_id: String,
    pub completed_at: String,
    pub activated_challenge_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewEvent {
    pub item_id: String,
    pub completed_at: String,
    #[serde(default)]
    pub activated_challenge_ids: Option<Vec<String>>,
}

pub use api::routes;
pub use repository::{EventFilter, EventRepository};
