use serde::{Deserialize, Serialize};

use crate::event::ClickEvent;

#[derive(Serialize, Deserialize, Debug)]
pub struct Binding {
    #[serde(default)]
    pub comment: String,
    pub event: ClickEvent,

    #[serde(skip_serializing)]
    #[serde(default)]
    pub cmd: Vec<String>,

    #[serde(default)]
    pub cmd_str: String,
}
