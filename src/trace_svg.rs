use std::ops::Mul;

use log::trace;

use crate::event::PointHistory;
use crate::grab;

pub fn trace_svg(point_history: &PointHistory, angles: &Vec<f64>) {
    let points;
    {
        let ph = point_history;

        points = grab::normalize_points(&ph, true);
    }
    let histo_str = points
        .iter()
        .map(|p| format!("L {},{}", p.x, p.y))
        .collect::<Vec<String>>()
        .join(" ");

    if !histo_str.is_empty() {
        let mut angle_curve = String::new();
        let mut x_curve = String::new();
        let mut y_curve = String::new();
        for (x, angle) in angles.iter().enumerate() {
            angle_curve.push_str(format!("{},{} ", 100 + x * 2, 100.0 + angle.mul(30.)).as_str());
        }

        for (x, p) in points.iter().enumerate() {
            x_curve.push_str(format!("{},{} ", 100 + x * 2, 300 + p.x).as_str());
        }

        for (x, p) in points.iter().enumerate() {
            y_curve.push_str(format!("{},{} ", 100 + x * 2, 800 + p.y).as_str());
        }

        trace!(
            r###"

<svg width="3440" height="1440" xmlns="http://www.w3.org/2000/svg">
  <path stroke-width="5" fill="none" stroke="red" d="M{}"/>
  <polyline stroke-width="5" fill="none" stroke="blue" points="{angle_curve}"/>
  <polyline stroke-width="5" fill="none" stroke="blue" points="{x_curve}"/>
  <polyline stroke-width="5" fill="none" stroke="blue" points="{y_curve}"/>
</svg>

"###,
            &histo_str[1..]
        );
    }
}
