use std::ops::{Deref, Mul};
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{thread, time};
use std::fmt;

use log::{debug, error, info, trace, warn};
use rdev::{simulate, EventType, Button};

use crate::args::Args;
use crate::binding::Binding;
use crate::compare_angles::compare_angles_with_offset;
use crate::config::Config;
use crate::event;
use crate::event::{edges_are_equals, modifiers_are_equals, ClickEvent};
use crate::grab::normalize_points;
use crate::record::reduce_shape_precision;

const DIFF_MAX: f64 = 0.8;
const DIFF_MIN_WITH_SECOND: f64 = 0.05;
const DIFF_MAX_PRINT: f64 = 300.0;
const SHAPE_MIN_SIZE: usize = 8;

// TODO refactor

/// filter the binding[] of config : keep bindings that have the same button, edges and modifiers
pub fn find_candidates<'a>(config: &'a Config, event: &ClickEvent) -> Vec<&'a Binding> {
    let shape_button = &config.shape_button;
    config
        .bindings
        .iter()
        .filter(|binding| {
            // TODO comment
            (binding.event.shapes_angles.is_empty()
                || shape_button != &binding.event.button
                || event.event_type != event::EventType::Press)
                && binding.event.button == event.button
                && (binding.event.event_type == event.event_type
                    || (binding.event.event_type == event::EventType::Click
                        || binding.event.event_type == event::EventType::Shape
                            && event.event_type == event::EventType::Release))
                && (edges_are_equals(&binding.event.edges, &event.edges)
                    || binding.event.event_type == event::EventType::Shape)
                && modifiers_are_equals(&binding.event.modifiers, &event.modifiers)
        })
        .collect::<Vec<&Binding>>()
}

pub fn find_candidates_with_shape_with_offset<'a>(
    candidates: &'a [&Binding],
    event: &ClickEvent,
    // FIXME use struct & check lifetime usages
) -> Vec<(&'a &'a Binding, f64)> {
    debug!(
        "angles: {}",
        event
            .shapes_angles
            .first()
            .map(|angles| angles
                .iter()
                .map(|a| format!("{:.2}, ", a))
                .collect::<String>())
            .unwrap_or_default()
    );
    let start = Instant::now();
    let mut candidates_with_shape = candidates
        .iter()
        .filter(|binding| binding.event.shapes_angles.first().is_some())
        .map(|binding| {
            trace!("compare_angles_with_offset of {}", binding.comment);
            (
                binding,
                binding
                    .event
                    .shapes_angles
                    .iter()
                    .filter(|angles| angles.len() > SHAPE_MIN_SIZE)
                    .map(|angles| {
                        let res = compare_angles_with_offset(
                            &event.shapes_angles.first().unwrap(),
                            &angles,
                        );
                        trace!("  res = {res}");
                        res
                    })
                    .min_by(|a, b| a.partial_cmp(b).unwrap()),
            )
        })
        .filter(|(_, res)| res.is_some())
        .map(|(b, res)| (b, res.unwrap()))
        .filter(|(_, diff)| *diff < DIFF_MAX_PRINT)
        .collect::<Vec<_>>();
    candidates_with_shape.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());
    debug!(
        "find_candidates_with_shape_with_offset duration : {:?}",
        start.elapsed()
    );
    candidates_with_shape
}

pub fn find_the_chosen_one_among_the_candidates_with_shape<'a>(
    candidates: &'a [&Binding],
    event: &ClickEvent,
) -> Option<&'a Binding> {
    let shape_size = event
        .shapes_angles
        .first()
        .map(|angles| angles.len())
        .unwrap_or_default();

    if shape_size > SHAPE_MIN_SIZE {
        let candidates_with_shape = find_candidates_with_shape_with_offset(candidates, event);

        // check is not empty
        if let Some(first) = candidates_with_shape.first() {
            debug!("shape candidates=");
            candidates_with_shape
                .iter()
                .take(5)
                .for_each(|(binding, diff)| {
                    let pc = f64::max(0., 100.0 - diff.powi(2).mul(100.));
                    debug!(
                        "   {:05.2} %    {diff:.2} : {}    {:?}",
                        pc, binding.comment, binding.cmd
                    )
                });

            if first.1 < DIFF_MAX {
                if let Some(second) = candidates_with_shape.get(1) {
                    if second.1 - first.1 > DIFF_MIN_WITH_SECOND {
                        return Some(first.0);
                    } else {
                        debug!("The first candidate is too close to the second : {} - {} < {DIFF_MIN_WITH_SECOND} → ignore this event",
                            second.1 , first.1);
                    }
                } else {
                    // only one candidate
                    return Some(first.0);
                }
            } else {
                debug!("shape difference > {DIFF_MAX} → ignore this event");
            }
        }
    } else {
        trace!("shape size({shape_size}) <= {SHAPE_MIN_SIZE} → ignore this event");
    }
    None
}

pub fn find_the_chosen_one_among_the_candidates_without_shape<'a>(
    candidates: &'a [&Binding],
    event: &ClickEvent,
) -> Option<&'a Binding> {
    let candidates_without_shape = candidates
        .iter()
        .filter(|b| b.event.shapes_angles.is_empty())
        .collect::<Vec<_>>();

    match candidates_without_shape.len() {
        1 => {
            let binding = candidates_without_shape.first().unwrap();
            debug!("Binding without shape found : {:?}", binding);
            return Some(binding);
        }
        0 => {}
        _ => {
            warn!(
                "WARNING, several candidate ! ev = {:?} candidates = {:?}",
                event, candidates_without_shape
            );
        }
    }
    None
}

pub fn find_the_chosen_one_among_the_candidates<'a>(
    candidates: &'a [&Binding],
    event: &ClickEvent,
) -> Option<&'a Binding> {
    find_the_chosen_one_among_the_candidates_with_shape(candidates, event)
        .or_else(|| find_the_chosen_one_among_the_candidates_without_shape(candidates, event))
}

pub fn trace_event(_config: Arc<Mutex<Config>>, event: ClickEvent, _args: Arc<Args>) -> bool {
    println!("event={:?}", event);
    if let Some(shapes_xy) = event.shapes_xy.first() {
        let normalized_points = normalize_points(&shapes_xy, false);
        trace!("normalized_points = {normalized_points:?}");
        println!(
            "shapes_xy={}",
            serde_json::to_string(&normalized_points).unwrap()
        );
    }

    true
}

pub fn grab_one_event(config: Arc<Mutex<Config>>, event: ClickEvent, _args: Arc<Args>) -> bool {
    if config.lock().unwrap().shape_button != event.button
        || !event.shapes_angles.is_empty()
        || event.event_type != event::EventType::Press
        || !event.edges.is_empty()
        || !event.modifiers.is_empty()
    {
        eprintln!("event=");
        let event = reduce_shape_precision(event);
        let serialized = serde_json::to_string(&event).unwrap();
        println!("{serialized}");
        eprintln!("====exit");
        exit(0);
    }
    true
}



/// Execute the command of the event if the corresponding binding is found.
/// return false if the event must not be propagated
pub fn process_event(config: Arc<Mutex<Config>>, event: ClickEvent, _args: Arc<Args>) -> bool {
    let mut propagate = true;
    let start = Instant::now();
    let config_lock = config.lock().unwrap();
    let config = config_lock.deref();
    let candidates = find_candidates(config, &event);
    trace!("event={:?}", event);
    // trace!("candidates={:?}", candidates);

    if !candidates.is_empty() {
        debug!("----------------------------------------");
        if let Some(binding) = find_the_chosen_one_among_the_candidates(&candidates, &event) {
            propagate = false;
            if !(event.event_type == event::EventType::Release
                && binding.event.event_type == event::EventType::Click
                && binding.event.shapes_angles.is_empty())
            {
                process_cmd(binding.cmd.clone());
            }
        } else if event.event_type == event::EventType::Release
            && event.button == config.shape_button
        {
            propagate = false;
            let rdev_btn = config.shape_button.to_rdev_event();

            debug!("simulate");
            debug!("event shape_angles: {}", event.shapes_angles.first().unwrap().len());
            /*simulate(&EventType::ButtonPress(rdev_btn))
                .map_err(|err| error!("simulate err: {:?}", err))
                .ok();
            thread::sleep(time::Duration::from_millis(10));
            simulate(&EventType::ButtonRelease(rdev_btn))
                .map_err(|err| error!("simulate err: {:?}", err))
                .ok(); */
            //if(rdev_btn == Button::Extra){
            let code = match rdev_btn {
                Button::Left   => 272,
                Button::Right  => 273,
                Button::Middle => 274,
                Button::Side   => 275,
                Button::Extra  => 276,
                Button::Forward=> 277,
                Button::Back   => 278,
                Button::Task   => 279,
                _ => 274
            };
            process_cmd(vec!["play-event.py".to_string(),"--code".to_string(), code.to_string()]);
            debug!("button attemped={:?}", rdev_btn);
            //} else if(rdev_btn == Button::Right){
            //    process_cmd(vec!["libinput".to_string(),"replay".to_string(),"--once".to_string(),"--replay-after".to_string(),"0".to_string(),"/home/spaceslug/Projects/mouse-actions/right-button.yaml".to_string()]);
            //    debug!("button extra attemped");
            //} else {
            //    debug!("no simulate attemped");
            //}
        }
    }
    trace!("propagate = {propagate}");
    if !propagate {
        debug!("Process event duration : {:?}", start.elapsed());
    }
    propagate
}

#[cfg(unix)]
pub fn process_cmd(cmd: Vec<String>) {
    thread::Builder::new()
        .name("process_cmd".to_string())
        .spawn(move || {
            info!("     → cmd {:?}", cmd);
            let res = Command::new(&cmd[0])
                .env_remove("RUST_LOG")
                .args(&cmd[1..])
                .process_group(0)
                .spawn();

            trace!("spawn result : {:?}", res);
        })
        .unwrap();
}

#[cfg(windows)]
fn process_cmd(cmd: Vec<String>) {
    thread::spawn(move || {
        info!("     → cmd {:?}", cmd);

        let p = Popen::create(
            &cmd,
            PopenConfig {
                env: Some(vec![("RUST_LOG".parse().unwrap(), "".parse().unwrap())]),
                ..Default::default()
            },
        );
        let res = Command::new(&cmd[0])
            .env("RUST_LOG", "")
            .args(&cmd[1..])
            .spawn();

        trace!("spawn result : {:?}", p);
    })
    .unwrap();
}
