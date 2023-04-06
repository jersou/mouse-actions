use crate::event::PointHistory;

const MOV_AVG_COEFFICIENT: f64 = 0.8;
const MOV_AVG_COEFFICIENT_COMP: f64 = 1. - MOV_AVG_COEFFICIENT;

// get average angles from points
pub fn points_to_angles(point_history: &PointHistory) -> Vec<f64> {
    if point_history.len() > 1 {
        if let Some(first) = point_history.first() {
            let mut angles: Vec<f64> = Vec::with_capacity(point_history.len());
            let mut last_point = first;
            let mut rx = 0.;
            let mut ry = 0.;
            for point in point_history[1..].iter() {
                // Moving average on rx & ry from point_history
                rx = MOV_AVG_COEFFICIENT * rx
                    + MOV_AVG_COEFFICIENT_COMP * ((point.x - last_point.x) as f64);
                ry = MOV_AVG_COEFFICIENT * ry
                    + MOV_AVG_COEFFICIENT_COMP * ((point.y - last_point.y) as f64);
                // calculate angle of rx ry
                let hyp = (rx * rx + ry * ry).sqrt();
                // add only if point != last_point (move)
                if hyp > 0.0 {
                    let angle = if ry <= 0.0 {
                        (rx / hyp).acos()
                    } else {
                        0.0 - (rx / hyp).acos()
                    };
                    angles.push(angle);
                }
                last_point = point;
            }
            angles
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_4;

    use crate::event::{Point, PointHistory};
    use crate::points_to_angles::points_to_angles;

    #[test]
    fn test_points_to_angles() {
        let mut points = PointHistory::new();
        points.push(Point { x: 0, y: 0 });
        points.push(Point { x: 10, y: 10 });
        points.push(Point { x: 20, y: 20 });
        points.push(Point { x: 10, y: 10 });
        points.push(Point { x: 10, y: 10 });
        points.push(Point { x: 0, y: 0 });
        points.push(Point { x: 0, y: 10 });
        points.push(Point { x: 10, y: 10 });
        let angles = points_to_angles(&points);
        assert_eq!(
            angles,
            vec![
                -FRAC_PI_4,
                -FRAC_PI_4,
                -FRAC_PI_4,
                -FRAC_PI_4,
                2.356194490192345,
                -2.504536715598747,
                -0.5620315286694877,
            ]
        )
    }
}
