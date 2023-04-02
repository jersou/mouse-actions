use std::sync::{Arc, Mutex};
use std::{thread, time};

use log::error;
use rdev::{listen, Event, EventType};

use crate::event::Point;

pub fn start_listen(last_point_listen: Arc<Mutex<Point>>) {
    // FIXME rdev::grab mouse position does not correspond to reality, unlike rdev::listen
    thread::spawn(|| {
        thread::sleep(time::Duration::from_millis(1000));
        if let Err(error) = listen(move |event: Event| {
            if let EventType::MouseMove { x, y } = event.event_type {
                last_point_listen.lock().unwrap().set(x as i32, y as i32);
            }
        }) {
            error!("Listen Error: {:?}", error);
            std::process::exit(1);
        }
    });
}
