use serde::{Deserialize, Serialize};

use crate::event::ClickEvent;

#[derive(Serialize, Deserialize, Debug)]
pub struct Binding {
    #[serde(default)]
    pub comment: String,
    pub event: ClickEvent,
    pub cmd: Vec<String>,
}
