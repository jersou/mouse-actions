use crate::event::{Point, PointHistory};

const MOV_AVG_COEFFICIENT: f64 = 0.8;

pub fn points_to_angles(point_history: &PointHistory) -> Vec<f64> {
    let histo = point_history.lock().unwrap();
    let mut angles: Vec<f64> = Vec::with_capacity(histo.len());
    if histo.len() > 1 {
        let mut last_point: Point = Point { x: -1, y: -1 };
        let mut x = 0.0;
        let mut y = 0.0;
        for point in histo.iter() {
            if last_point.x >= 0 {
                // Moving average on x & y from point_history
                x = x * MOV_AVG_COEFFICIENT + (point.x - last_point.x) as f64;
                y = y * MOV_AVG_COEFFICIENT + (point.y - last_point.y) as f64;
                let hyp = (x * x + y * y).sqrt();
                if hyp > 0.0 {
                    let angle = if y <= 0.0 {
                        (x / hyp).acos()
                    } else {
                        0.0 - (x / hyp).acos()
                    };
                    angles.push(angle);
                }
            }
            last_point = *point;
        }
    }
    angles
}
