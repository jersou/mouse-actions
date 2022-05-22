use serde::{Deserialize, Serialize};

use crate::event::ClickEvent;

#[derive(Serialize, Deserialize, Debug)]
pub struct Binding {
    pub event: ClickEvent,
    pub cmd: Vec<String>,
}
