use std::cmp;
use std::f64::consts::PI;
use std::ops::Div;

use log::trace;

pub fn compare_angles_with_offset(vec_a: &Vec<f64>, vec_b: &Vec<f64>) -> f64 {
    let mut diff_values: Vec<f64> = Vec::with_capacity(25);
    diff_values.push(compare_angles(vec_a, vec_b));
    let vec1: &Vec<f64>;
    let vec2: &Vec<f64>;

    // iter on the smaller
    if vec_a.len() < vec_b.len() {
        vec1 = vec_a;
        vec2 = vec_b;
    } else {
        vec1 = vec_b;
        vec2 = vec_a;
    };
    let offset_max = cmp::min(20, vec1.len() / 10);

    for i in (2..offset_max).step_by(2) {
        diff_values.push(compare_angles(&vec1[i..], vec2));
    }
    for i in (2..offset_max).step_by(2) {
        diff_values.push(compare_angles(&vec1[0..vec1.len() - i], vec2));
    }
    trace!("diff_values {:?}", diff_values);
    diff_values
        .into_iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

pub fn compare_angles(vec_a: &[f64], vec_b: &[f64]) -> f64 {
    let smaller_vec: &[f64];
    let bigger_vec: &[f64];

    // iter on the smaller
    if vec_a.len() < vec_b.len() {
        smaller_vec = vec_a;
        bigger_vec = vec_b;
    } else {
        smaller_vec = vec_b;
        bigger_vec = vec_a;
    };
    // TODO check vet1 len > min accept

    let mut diff = 0.0;
    for i in 0..smaller_vec.len() {
        let angle_ref: &f64 = smaller_vec.get(i).unwrap();
        let progress: f64 = (i as f64) / (smaller_vec.len() as f64);
        let angle_opt = bigger_vec.get(((bigger_vec.len() as f64) * progress) as usize);
        if let Some(angle) = angle_opt {
            let mut raw_diff = (angle_ref - angle).abs();
            raw_diff = if raw_diff > PI {
                2.0 * PI - raw_diff
            } else {
                raw_diff
            };
            diff += raw_diff;
        } else {
            // TODO
            diff += PI;
        }
    }
    diff.div(smaller_vec.len() as f64)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::compare_angles::{compare_angles, compare_angles_with_offset};

    #[test]
    fn test_compare_angles() {
        let vect1: Vec<f64> = vec![
            0.00, 0.00, 0.55, 0.33, 0.22, 0.22, -0.12, -0.12, -0.19, -0.19, -0.43, -0.43, -0.51,
            -0.51, -0.51, -0.51, -0.47, -0.47, -0.49, -0.49, -0.51, -0.51, -0.58, -0.58, -0.60,
            -0.60, -0.64, -0.64, -0.65, -0.65, -0.72, -0.72, -0.74, -0.74, -0.78, -0.78, -0.83,
            -0.83, -0.91, -0.91, -1.00, -1.00, -1.06, -1.18, -1.27, -1.27, -1.33, -1.33, -1.49,
            -1.49, -1.76, -1.76, -2.19, -2.53, -2.76, -2.89, -2.97, -2.97, -3.11, -3.11, 3.05,
            3.05, 3.05, 3.05, 3.05, 2.98, 2.98, 2.90, 2.90, 2.82, 2.82, 2.78, 2.78, 2.78, 2.62,
            2.62, 2.46, 2.26, 2.04, 2.04, 1.76, 1.76, 1.57, 1.57, 1.57, 1.30, 1.30, 1.02, 1.02,
            1.06, 1.06, 0.93, 0.93, 0.93, 0.85, 0.85, 0.70, 0.70, 0.64, 0.64, 0.58, 0.58, 0.56,
            0.56, 0.56, 0.60, 0.60, 0.60, 0.60, 0.58, 0.58, 0.59, 0.59, 0.55, 0.55, 0.55, 0.57,
            0.57,
        ];
        let vect2: Vec<f64> = vec![
            0.00, 0.00, -0.62, -0.62, -0.60, -0.60, -0.57, -0.57, -0.61, -0.61, -0.58, -0.58,
            -0.60, -0.60, -0.58, -0.58, -0.62, -0.62, -0.59, -0.59, -0.57, -0.57, -0.56, -0.56,
            -0.55, -0.55, -0.55, -0.55, -0.55, -0.55, -0.58, -0.58, -0.60, -0.60, -0.62, -0.62,
            -0.66, -0.66, -0.70, -0.74, -0.78, -0.83, -0.83, -0.93, -0.93, -1.07, -1.07, -1.20,
            -1.20, -1.48, -1.85, -1.85, -2.17, -2.47, -2.47, -2.63, -2.63, -2.86, -2.86, -3.02,
            -3.02, -3.12, -3.12, 3.04, 3.04, 3.04, 2.91, 2.91, 2.70, 2.48, 2.30, 2.30, 2.07, 2.07,
            2.07, 1.80, 1.80, 1.50, 1.50, 1.26, 1.26, 1.02, 1.02, 0.86, 0.86, 0.86, 0.79, 0.79,
            0.68, 0.68, 0.65, 0.65, 0.61, 0.61, 0.58, 0.58, 0.58, 0.54, 0.54, 0.52, 0.52, 0.51,
            0.51, 0.52, 0.52, 0.51,
        ];

        let start = Instant::now();

        let diff = compare_angles(&vect1, &vect2);
        println!("compare_angles : {diff}");
        assert_eq!(diff, 0.28338365815675726);
        println!("Time elapsed : {:?}", start.elapsed());

        let start = Instant::now();
        let diff = compare_angles_with_offset(&vect1, &vect2);
        println!("compare_angles_check_offset : {diff}");
        assert_eq!(diff, 0.159040066692836);
        println!("Time elapsed : {:?}", start.elapsed());
    }
}
