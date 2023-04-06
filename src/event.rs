use std::fmt;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

use arrayvec::ArrayVec;
use rdev::{display_size, Button};
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const HISTO_SIZE: usize = 10000;

// pub type PointHistory = ArrayVec<Point, HISTO_SIZE>;
#[derive(Default, Debug, Clone)]
pub struct PointHistory(ArrayVec<Point, HISTO_SIZE>);

impl PointHistory {
    pub fn new() -> PointHistory {
        PointHistory(ArrayVec::<Point, HISTO_SIZE>::new())
    }
}

impl Deref for PointHistory {
    type Target = ArrayVec<Point, HISTO_SIZE>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PointHistory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Serialize for PointHistory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len() * 2))?;
        for p in &self.0 {
            seq.serialize_element(&p.x)?;
            seq.serialize_element(&p.y)?;
        }
        seq.end()
    }
}

impl<'de> Visitor<'de> for PointHistory {
    type Value = PointHistory;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // FIXME
        formatter.write_str("PointHistory")
    }

    fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut ph = PointHistory::new();

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some(x) = access.next_element()? {
            if let Some(y) = access.next_element()? {
                ph.push(Point { x, y });
            }
        }

        Ok(ph)
    }
}

impl<'de> Deserialize<'de> for PointHistory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(PointHistory::new())
    }
}

pub type PointHistoryArcMutex = Arc<Mutex<PointHistory>>;

#[derive(Debug, Copy, Clone, Default)]
pub struct KeyboardState {
    pub shift_left: bool,
    pub shift_right: bool,
    pub control_left: bool,
    pub control_right: bool,
    pub meta_left: bool,
    pub alt: bool,
    pub alt_gr: bool,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

impl Edge {
    pub fn edges_from_pos(x: i32, y: i32) -> Vec<Edge> {
        let (width, height) = display_size().unwrap();
        let mut edges: Vec<Edge> = vec![];

        if x == 0 {
            edges.push(Edge::Left);
        } else if (x) == (width as i32 - 1) {
            edges.push(Edge::Right);
        }
        if y == 0 {
            edges.push(Edge::Top);
        } else if (y) == (height as i32 - 1) {
            edges.push(Edge::Bottom);
        }
        edges
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum KeyboardModifier {
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    MetaLeft,
    Alt,
    AltGr,
}

impl KeyboardModifier {
    pub fn from_keyboard_state(keyboard_state: KeyboardState) -> Vec<KeyboardModifier> {
        let mut modifiers: Vec<KeyboardModifier> = vec![];
        if keyboard_state.shift_left {
            modifiers.push(KeyboardModifier::ShiftLeft)
        };
        if keyboard_state.shift_right {
            modifiers.push(KeyboardModifier::ShiftRight)
        };
        if keyboard_state.control_left {
            modifiers.push(KeyboardModifier::ControlLeft)
        };
        if keyboard_state.control_right {
            modifiers.push(KeyboardModifier::ControlRight)
        };
        if keyboard_state.meta_left {
            modifiers.push(KeyboardModifier::MetaLeft)
        };
        if keyboard_state.alt {
            modifiers.push(KeyboardModifier::Alt)
        };
        if keyboard_state.alt_gr {
            modifiers.push(KeyboardModifier::AltGr)
        };
        modifiers
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum PressState {
    Press,
    Release,
    Click,
}

impl Default for PressState {
    fn default() -> Self {
        PressState::Click
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Side,
    Extra,
    Forward,
    Back,
    Task,
    Trigger,
    Thumb,
    Thumb2,
    WheelUp,
    WheelDown,
    Unknown,
    None,
}

impl MouseButton {
    pub fn from_rdev_event(btn: Button) -> MouseButton {
        match btn {
            Button::Left => MouseButton::Left,
            Button::Right => MouseButton::Right,
            Button::Middle => MouseButton::Middle,
            Button::Side => MouseButton::Side,
            Button::Extra => MouseButton::Extra,
            Button::Forward => MouseButton::Forward,
            Button::Back => MouseButton::Back,
            Button::Task => MouseButton::Task,
            Button::Trigger => MouseButton::Trigger,
            Button::Thumb => MouseButton::Thumb,
            Button::Thumb2 => MouseButton::Thumb2,
            Button::Unknown(_) => MouseButton::Unknown,
        }
    }
    pub fn to_rdev_event(self) -> Button {
        match self {
            MouseButton::Left => Button::Left,
            MouseButton::Right => Button::Right,
            MouseButton::Middle => Button::Middle,
            MouseButton::Side => Button::Side,
            MouseButton::Extra => Button::Extra,
            MouseButton::Forward => Button::Forward,
            MouseButton::Back => Button::Back,
            MouseButton::Task => Button::Task,
            MouseButton::Trigger => Button::Trigger,
            MouseButton::Thumb => Button::Thumb,
            MouseButton::Thumb2 => Button::Thumb2,
            _ => Button::Unknown(0),
        }
    }

    pub fn from_rdev_wheel(delta_y: i64) -> MouseButton {
        if delta_y > 0 {
            MouseButton::WheelUp
        } else {
            MouseButton::WheelDown
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClickEvent {
    pub button: MouseButton,

    #[serde(default)]
    pub edges: Vec<Edge>,

    #[serde(default)]
    pub modifiers: Vec<KeyboardModifier>,

    #[serde(default)]
    pub event_type: PressState,

    #[serde(default)]
    pub shape_angles: Vec<f64>,
}

pub fn edges_are_equals(edges1: &[Edge], edges2: &[Edge]) -> bool {
    edges1.len() == edges2.len() && edges1.iter().all(|edge| edges2.contains(edge))
}

pub fn modifiers_are_equals(
    modifiers1: &[KeyboardModifier],
    modifiers2: &[KeyboardModifier],
) -> bool {
    modifiers1.len() == modifiers2.len() && modifiers1.iter().all(|edge| modifiers2.contains(edge))
}

#[derive(Debug, Copy, Clone)]
pub enum ButtonState {
    Pressed(Button),
    None,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

#[cfg(test)]
mod tests {
    use crate::event::{Point, PointHistory};

    #[test]
    fn test_point_history_serialize() {
        let mut points = PointHistory::new();
        points.push(Point { x: 0, y: 0 });
        points.push(Point { x: 1, y: 1 });
        points.push(Point { x: 2, y: 2 });
        points.push(Point { x: 3, y: 3 });
        let serialized = serde_json::to_string(&points).unwrap();
        assert_eq!(serialized, "[0,0,1,1,2,2,3,3]");
    }

    #[test]
    fn test_point_history_deserialize() {
        let ph: PointHistory = serde_json::from_str("[0,0,1,1,2,2,3,3]").unwrap();
        let v: Vec<i32> = ph.to_vec().iter().flat_map(|p| vec![p.x, p.y]).collect();
        assert_eq!(v, vec![0, 0, 1, 1, 2, 2, 3, 3]);
    }
}
