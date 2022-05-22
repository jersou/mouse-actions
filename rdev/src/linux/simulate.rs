use std::convert::TryInto;
use std::os::raw::c_int;
use std::ptr::null;

use x11::xlib;
use x11::xtest;

use crate::linux::common::{FALSE, TRUE};
use crate::linux::keycodes::code_from_key;
use crate::rdev::{Button, EventType, SimulateError};

unsafe fn send_native(event_type: &EventType, display: *mut xlib::Display) -> Option<()> {
    let res = match event_type {
        EventType::KeyPress(key) => {
            let code = code_from_key(*key)?;
            xtest::XTestFakeKeyEvent(display, code, TRUE, 0)
        }
        EventType::KeyRelease(key) => {
            let code = code_from_key(*key)?;
            xtest::XTestFakeKeyEvent(display, code, FALSE, 0)
        }
        EventType::ButtonPress(button) => match button {
            Button::Left => xtest::XTestFakeButtonEvent(display, 1, TRUE, 0),
            Button::Middle => xtest::XTestFakeButtonEvent(display, 2, TRUE, 0),
            Button::Right => xtest::XTestFakeButtonEvent(display, 3, TRUE, 0),

            // FIXME button mapping
            Button::Side => xtest::XTestFakeButtonEvent(display, 10, TRUE, 0),
            Button::Extra => xtest::XTestFakeButtonEvent(display, 11, TRUE, 0),
            Button::Forward => xtest::XTestFakeButtonEvent(display, 9, TRUE, 0),
            Button::Back => xtest::XTestFakeButtonEvent(display, 8, TRUE, 0),
            Button::Task => xtest::XTestFakeButtonEvent(display, 12, TRUE, 0),
            Button::Trigger => xtest::XTestFakeButtonEvent(display, 13, TRUE, 0),
            Button::Thumb => xtest::XTestFakeButtonEvent(display, 14, TRUE, 0),
            Button::Thumb2 => xtest::XTestFakeButtonEvent(display, 15, TRUE, 0),

            Button::Unknown(code) => {
                xtest::XTestFakeButtonEvent(display, (*code).try_into().ok()?, TRUE, 0)
            }
        },
        EventType::ButtonRelease(button) => match button {
            Button::Left => xtest::XTestFakeButtonEvent(display, 1, FALSE, 0),
            Button::Middle => xtest::XTestFakeButtonEvent(display, 2, FALSE, 0),
            Button::Right => xtest::XTestFakeButtonEvent(display, 3, FALSE, 0),

            // FIXME button mapping
            Button::Side => xtest::XTestFakeButtonEvent(display, 10, FALSE, 0),
            Button::Extra => xtest::XTestFakeButtonEvent(display, 11, FALSE, 0),
            Button::Forward => xtest::XTestFakeButtonEvent(display, 9, FALSE, 0),
            Button::Back => xtest::XTestFakeButtonEvent(display, 8, FALSE, 0),
            Button::Task => xtest::XTestFakeButtonEvent(display, 12, FALSE, 0),
            Button::Trigger => xtest::XTestFakeButtonEvent(display, 13, FALSE, 0),
            Button::Thumb => xtest::XTestFakeButtonEvent(display, 14, FALSE, 0),
            Button::Thumb2 => xtest::XTestFakeButtonEvent(display, 15, FALSE, 0),

            Button::Unknown(code) => {
                xtest::XTestFakeButtonEvent(display, (*code).try_into().ok()?, FALSE, 0)
            }
        },
        EventType::MouseMove { x, y } => {
            //TODO: replace with clamp if it is stabalized
            let x = if x.is_finite() {
                x.min(c_int::max_value().into())
                    .max(c_int::min_value().into())
                    .round() as c_int
            } else {
                0
            };
            let y = if y.is_finite() {
                y.min(c_int::max_value().into())
                    .max(c_int::min_value().into())
                    .round() as c_int
            } else {
                0
            };
            xtest::XTestFakeMotionEvent(display, 0, x, y, 0)
            //     xlib::XWarpPointer(display, 0, root, 0, 0, 0, 0, *x as i32, *y as i32);
        }
        EventType::Wheel { delta_y, .. } => {
            let code = if *delta_y > 0 { 4 } else { 5 };
            xtest::XTestFakeButtonEvent(display, code, TRUE, 0)
                & xtest::XTestFakeButtonEvent(display, code, FALSE, 0)
        }
    };
    if res == 0 {
        None
    } else {
        Some(())
    }
}

pub fn simulate(event_type: &EventType) -> Result<(), SimulateError> {
    unsafe {
        let dpy = xlib::XOpenDisplay(null());
        if dpy.is_null() {
            return Err(SimulateError);
        }
        match send_native(event_type, dpy) {
            Some(_) => {
                xlib::XFlush(dpy);
                xlib::XSync(dpy, 0);
                xlib::XCloseDisplay(dpy);
                Ok(())
            }
            None => {
                xlib::XCloseDisplay(dpy);
                Err(SimulateError)
            }
        }
    }
}
