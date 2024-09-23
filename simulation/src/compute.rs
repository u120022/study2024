use nalgebra::*;

use crate::settings::*;

pub fn intersection_point(p0: [f64; 2], p1: [f64; 2], q0: [f64; 2], q1: [f64; 2]) -> [f64; 2] {
    let a_p = p1[1] - p0[1];
    let b_p = -(p1[0] - p0[0]);
    let c_p = -(a_p * p0[0] + b_p * p0[1]);
    let a_q = q1[1] - q0[1];
    let b_q = -(q1[0] - q0[0]);
    let c_q = -(a_q * q0[0] + b_q * q0[1]);
    let x_0 = (b_p * c_q - b_q * c_p) / (a_p * b_q - a_q * b_p);
    let y_0 = (-a_p * c_q + a_q * c_p) / (a_p * b_q - a_q * b_p);
    [x_0, y_0]
}

pub const ROAD_LENGTH: f64 = 64.0;
pub const STEP: f64 = 0.01;
pub const MAX_TIME: f64 = 100.0;

#[derive(Clone, Debug)]
pub struct VehOutput {
    pub c_in: f64,
    pub c_out: f64,
    pub v_min: f64,
    pub x_min: f64,
    pub t_min: f64,
    pub t_exit: f64,
    pub t_o: f64,
    pub x_o: f64,
    pub max_step: usize,
    pub velocity_series: Vec<[f64; 2]>,
    pub position_series: Vec<[f64; 2]>,
    pub curvature_series: Vec<[f64; 2]>,
    pub trajectory_series: Vec<[f64; 2]>,
}

pub fn compute_lt_veh(settings: &Settings, flow: &VehFlow) -> Option<VehOutput> {
    let rng = &mut rand::thread_rng();

    let mut velocity_series = vec![];

    let m = Rotation2::new(settings.angle.to_radians());

    let radius = settings.radius;

    let distr = rand_distr::Normal::new(flow.v_in_mean, flow.v_in_stdv).unwrap();
    let v_in = rand::Rng::sample(rng, distr);

    let distr = rand_distr::Normal::new(flow.v_out_mean, flow.v_out_stdv).unwrap();
    let v_out = rand::Rng::sample(rng, distr);

    let distr = rand_distr::Normal::new(flow.padding_out_mean, flow.padding_out_stdv).unwrap();
    let padding_out = rand::Rng::sample(rng, distr);

    let distr = rand_distr::Uniform::new(0.0, 1.0);
    let large = rand::Rng::sample(rng, distr) < flow.large_prob;
    let large_dummy = if large { 1.0 } else { 0.0 };

    let angle = match (flow.src_dir, flow.dst_dir) {
        (Dir::NxPy, Dir::PyNx) => settings.angle,
        (Dir::PyPx, Dir::PxPy) => 180.0 - settings.angle,
        (Dir::PxNy, Dir::NyPx) => settings.angle,
        (Dir::NyNx, Dir::NxNy) => 180.0 - settings.angle,
        _ => unreachable!(),
    };

    let tx = match (flow.src_dir, flow.dst_dir) {
        (Dir::NxPy, Dir::PyNx) => {
            let y = settings.lane_along[flow.src_lane];
            let p0 = Point2::new(-ROAD_LENGTH, y).into();
            let p1 = Point2::new(ROAD_LENGTH, y).into();
            let y = settings.width_across * 0.5 - padding_out;
            let q0 = (m * Point2::new(-ROAD_LENGTH, y)).into();
            let q1 = (m * Point2::new(ROAD_LENGTH, y)).into();
            let o = intersection_point(p0, p1, q0, q1).into();
            Isometry2::new(o, 0.0)
        }
        (Dir::PyPx, Dir::PxPy) => {
            let y = settings.lane_across[flow.src_lane];
            let p0 = (m * Point2::new(-ROAD_LENGTH, y)).into();
            let p1 = (m * Point2::new(ROAD_LENGTH, y)).into();
            let y = settings.width_along * 0.5 - padding_out;
            let q0 = Point2::new(-ROAD_LENGTH, y).into();
            let q1 = Point2::new(ROAD_LENGTH, y).into();
            let o = intersection_point(p0, p1, q0, q1).into();
            Isometry2::new(o, m.angle() + std::f64::consts::PI)
        }
        (Dir::PxNy, Dir::NyPx) => {
            let y = settings.lane_along[flow.src_lane];
            let p0 = Point2::new(-ROAD_LENGTH, y).into();
            let p1 = Point2::new(ROAD_LENGTH, y).into();
            let y = -(settings.width_across * 0.5 - padding_out);
            let q0 = (m * Point2::new(-ROAD_LENGTH, y)).into();
            let q1 = (m * Point2::new(ROAD_LENGTH, y)).into();
            let o = intersection_point(p0, p1, q0, q1).into();
            Isometry2::new(o, std::f64::consts::PI)
        }
        (Dir::NyNx, Dir::NxNy) => {
            let y = settings.lane_across[flow.src_lane];
            let p0 = (m * Point2::new(-ROAD_LENGTH, y)).into();
            let p1 = (m * Point2::new(ROAD_LENGTH, y)).into();
            let y = -(settings.width_along * 0.5 - padding_out);
            let q0 = Point2::new(-ROAD_LENGTH, y).into();
            let q1 = Point2::new(ROAD_LENGTH, y).into();
            let o = intersection_point(p0, p1, q0, q1).into();
            Isometry2::new(o, m.angle())
        }
        _ => unreachable!(),
    };

    // c_in parameter
    let a = vector![2.09, 0.256, -0.0155, 0.0, -0.168, 0.0];
    let x = vector![1.0, v_in, angle, radius, padding_out, v_out];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = vector![0.0573, -0.00173, -0.00109, 0.00219, 0.0];
    let y = vector![1.0, v_in, radius, padding_out, v_out];
    let scale = b.dot(&y).max(f64::EPSILON);
    let c_in = rand_distr::Gamma::new(shape, scale).unwrap();
    let c_in = rand::Rng::sample(rng, c_in);

    // c_out parameter
    let a = vector![1.40, 0.0, 0.0, 0.0, 0.0633, -0.0224];
    let x = vector![1.0, v_in, angle, radius, padding_out, v_out];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = vector![0.0772, 0.0, 0.0, 0.0, -0.00355];
    let y = vector![1.0, v_in, radius, padding_out, v_out];
    let scale = b.dot(&y).max(f64::EPSILON);
    let c_out = rand_distr::Gamma::new(shape, scale).unwrap();
    let c_out = rand::Rng::sample(rng, c_out);

    // v_min parameter
    let a = vector![-0.301, 0.0908, 0.0607, 0.0387, 0.233, -0.496];
    let x = vector![1.0, v_in, radius, angle, padding_out, large_dummy];
    let mean = a.dot(&x);
    let b = vector![0.665, 0.0, 0.0419];
    let y = vector![1.0, radius, padding_out];
    let std_dev = b.dot(&y).max(f64::EPSILON);
    let v_min = rand_distr::Normal::new(mean, std_dev).unwrap();
    let v_min = rand::Rng::sample(rng, v_min);

    // inflow
    let t_min = (2.0 / c_in * (v_in - v_min)).cbrt();
    let a = c_in;
    let b = -3.0 / 2.0 * a * t_min;
    let c = 0.0;
    let d = v_in;
    let mut t = 0.0;
    while t <= t_min {
        if t > MAX_TIME {
            break;
        }
        let point = [t, a * t.powi(3) + b * t.powi(2) + c * t + d];
        velocity_series.push(point);
        t += STEP;
    }

    // outflow
    let t_next = (2.0 / -c_out * (v_min - v_out)).cbrt();
    let a = -c_out;
    let b = -3.0 / 2.0 * a * t_next;
    let c = 0.0;
    let d = v_min;
    let mut t = 0.0;
    while t <= t_next {
        if t > MAX_TIME {
            break;
        }
        let point = [t + t_min, a * t.powi(3) + b * t.powi(2) + c * t + d];
        velocity_series.push(point);
        t += STEP;
    }

    let max_step = velocity_series.len();

    // x_min parameter
    let a = vector![1.42, 0.0, 0.586, 0.0896, 0.577, 0.0];
    let x = vector![1.0, v_in, radius, angle, padding_out, large_dummy];
    let mean = a.dot(&x);
    let b = vector![0.135, 0.144, 0.336];
    let y = vector![1.0, radius, padding_out];
    let std_dev = b.dot(&y).max(f64::EPSILON);
    let x_min = rand_distr::Normal::new(mean, std_dev).unwrap();
    let x_min = rand::Rng::sample(rng, x_min);

    // position
    let mut x_min_ = 0.0;
    let mut position_series = vec![[0.0; 2]; max_step];
    for i in 1..max_step {
        let [t0, v0] = velocity_series[i - 1];
        let [t1, v1] = velocity_series[i];

        let [_, x0] = position_series[i - 1];

        if t0 <= t_min && t_min < t1 {
            x_min_ = x0;
        }

        let x1 = x0 + (t1 - t0) * (v0 + v1) * 0.5;
        position_series[i] = [(t0 + t1) * 0.5, x1];
    }
    let x_o = x_min_ - x_min;

    // r_min parameter
    let a = vector![0.127, 0.390, 0.862, -6.46];
    let x = vector![angle, radius, padding_out, 1.0,];
    let mean = a.dot(&x);
    let b = vector![0.0363, 0.0624, 0.118, -2.86];
    let y = vector![angle, radius, padding_out, 1.0,];
    let std_dev = b.dot(&y).max(f64::EPSILON);
    let r_min = rand_distr::Normal::new(mean, std_dev).unwrap();
    let r_min = rand::Rng::sample(rng, r_min);

    // curvature
    let a = vector![-1.65, 0.0404, 0.334, 0.0, 0.461, 0.369];
    let x = vector![1.0, angle, radius, large_dummy, padding_out, v_min];
    let a1 = a.dot(&x);
    let a = vector![2.33, 0.0, 0.335, 2.05, 1.04, 0.268];
    let x = vector![1.0, angle, radius, large_dummy, padding_out, v_min];
    let a2 = a.dot(&x);
    let l_clothoid1 = r_min.recip() / a1.powi(2).recip();
    let angle_clothoid1 = 0.5 * a1.powi(2).recip() * l_clothoid1.powi(2);
    let l_clothoid2 = r_min.recip() / a2.powi(2).recip();
    let angle_clothoid2 = 0.5 * a2.powi(2).recip() * l_clothoid2.powi(2);
    let angle_arc = angle.to_radians() - (angle_clothoid1 + angle_clothoid2);
    let l_arc = angle_arc / r_min.recip();
    if l_arc < 0.0 {
        return None;
    }
    let x_0 = x_o;
    let x_1 = x_0 + l_clothoid1;
    let x_2 = x_1 + l_arc;
    let x_3 = x_2 + l_clothoid2;
    let mut curvature_series = vec![[0.0; 2]; max_step];
    for i in 0..max_step {
        let [_, x] = position_series[i];
        if x_0 <= x && x < x_1 {
            curvature_series[i] = [x, a1.powi(2).recip() * (x - x_0)];
        } else if x_1 <= x && x < x_2 {
            curvature_series[i] = [x, r_min.recip()];
        } else if x_2 <= x && x < x_3 {
            curvature_series[i] = [x, r_min.recip() - a2.powi(2).recip() * (x - x_2)];
        } else {
            curvature_series[i] = [x, 0.0];
        }
    }

    // trajectory
    let mut angle: f64 = 0.0;
    let mut position = [0.0; 2];
    let mut trajectory_series = vec![[0.0; 2]; max_step];
    for i in 0..max_step {
        let [_, v] = velocity_series[i];
        let [_, c] = curvature_series[i];

        let dx = v * STEP;
        angle += c * dx;
        position[0] += angle.cos() * dx;
        position[1] += angle.sin() * dx;

        trajectory_series[i] = position;
    }

    // origin time
    let mut t_o = 0.0;
    for w in position_series.windows(2) {
        let [t, x0] = w[0];
        let [_, x1] = w[1];

        if x0 <= x_o && x_o < x1 {
            t_o = t;
        }
    }

    // origin shift
    for [t, _] in velocity_series.iter_mut() {
        *t -= t_o;
    }
    for [t, x] in position_series.iter_mut() {
        *t -= t_o;
        *x -= x_o;
    }
    for [x, _] in curvature_series.iter_mut() {
        *x -= x_o;
    }

    // trajectory origin shift
    if trajectory_series.len() > 2 {
        let (start_idx, last_idx) = (0, trajectory_series.len() - 1);
        let trajectory_origin = intersection_point(
            trajectory_series[start_idx],
            trajectory_series[start_idx + 1],
            trajectory_series[last_idx],
            trajectory_series[last_idx - 1],
        );
        let tx = tx * Translation2::from(trajectory_origin).inverse();
        for p in trajectory_series.iter_mut() {
            *p = (tx * Point2::from(*p)).into();
        }
    }

    Some(VehOutput {
        c_in,
        c_out,
        v_min,
        x_min,
        t_min,
        t_exit: t_min + t_next,
        t_o,
        x_o,
        max_step,
        velocity_series,
        position_series,
        curvature_series,
        trajectory_series,
    })
}

pub fn compute_rt_veh(settings: &Settings, flow: &VehFlow) -> Option<VehOutput> {
    let rng = &mut rand::thread_rng();

    let mut velocity_series = vec![];

    let m = Rotation2::new(settings.angle.to_radians());

    let distr = rand_distr::Normal::new(flow.v_in_mean, flow.v_in_stdv).unwrap();
    let v_in = rand::Rng::sample(rng, distr);

    let distr = rand_distr::Normal::new(flow.v_out_mean, flow.v_out_stdv).unwrap();
    let v_out = rand::Rng::sample(rng, distr);

    let distr = rand_distr::Normal::new(flow.padding_out_mean, flow.padding_out_stdv).unwrap();
    let padding_out = rand::Rng::sample(rng, distr);

    let (hn_in, hn_out) = match (flow.src_dir, flow.dst_dir) {
        (Dir::NxPy, Dir::NyPx) => (settings.hn_along, settings.hn_across),
        (Dir::NyNx, Dir::PxPy) => (settings.hn_across, settings.hn_along),
        (Dir::PxNy, Dir::PyNx) => (settings.hn_along, settings.hn_across),
        (Dir::PyPx, Dir::NxNy) => (settings.hn_across, settings.hn_along),
        _ => unreachable!(),
    };

    let angle = match (flow.src_dir, flow.dst_dir) {
        (Dir::NxPy, Dir::NyPx) => 180.0 - settings.angle,
        (Dir::NyNx, Dir::PxPy) => settings.angle,
        (Dir::PxNy, Dir::PyNx) => 180.0 - settings.angle,
        (Dir::PyPx, Dir::NxNy) => settings.angle,
        _ => unreachable!(),
    };

    let tx = match (flow.src_dir, flow.dst_dir) {
        (Dir::NxPy, Dir::NyPx) => {
            let y = settings.lane_along[flow.src_lane];
            let p0 = Point2::new(-ROAD_LENGTH, y).into();
            let p1 = Point2::new(ROAD_LENGTH, y).into();
            let y = -(settings.width_across * 0.5 - padding_out);
            let q0 = (m * Point2::new(-ROAD_LENGTH, y)).into();
            let q1 = (m * Point2::new(ROAD_LENGTH, y)).into();
            let o = intersection_point(p0, p1, q0, q1).into();
            Isometry2::new(o, 0.0)
        }
        (Dir::NyNx, Dir::PxPy) => {
            let y = settings.lane_across[flow.src_lane];
            let p0 = (m * Point2::new(-ROAD_LENGTH, y)).into();
            let p1 = (m * Point2::new(ROAD_LENGTH, y)).into();
            let y = settings.width_along * 0.5 - padding_out;
            let q0 = Point2::new(-ROAD_LENGTH, y).into();
            let q1 = Point2::new(ROAD_LENGTH, y).into();
            let o = intersection_point(p0, p1, q0, q1).into();
            Isometry2::new(o, m.angle())
        }
        (Dir::PxNy, Dir::PyNx) => {
            let y = settings.lane_along[flow.src_lane];
            let p0 = Point2::new(-ROAD_LENGTH, y).into();
            let p1 = Point2::new(ROAD_LENGTH, y).into();
            let y = settings.width_across * 0.5 - padding_out;
            let q0 = (m * Point2::new(-ROAD_LENGTH, y)).into();
            let q1 = (m * Point2::new(ROAD_LENGTH, y)).into();
            let o = intersection_point(p0, p1, q0, q1).into();
            Isometry2::new(o, std::f64::consts::PI)
        }
        (Dir::PyPx, Dir::NxNy) => {
            let y = settings.lane_across[flow.src_lane];
            let p0 = (m * Point2::new(-ROAD_LENGTH, y)).into();
            let p1 = (m * Point2::new(ROAD_LENGTH, y)).into();
            let y = -(settings.width_along * 0.5 - padding_out);
            let q0 = Point2::new(-ROAD_LENGTH, y).into();
            let q1 = Point2::new(ROAD_LENGTH, y).into();
            let o = intersection_point(p0, p1, q0, q1).into();
            Isometry2::new(o, m.angle() + std::f64::consts::PI)
        }
        _ => unreachable!(),
    };

    // c_in parameter
    let a = vector![0.320, -0.0150];
    let x = vector![v_in, angle];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = vector![0.000334, 0.0];
    let y = vector![v_in, hn_out];
    let scale = b.dot(&y).max(f64::EPSILON);
    let c_in = rand_distr::Gamma::new(shape, scale).unwrap();
    let c_in = rand::Rng::sample(rng, c_in);

    // c_out parameter
    let a = vector![0.0275, 0.0108];
    let x = vector![v_in, angle];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = vector![0.000228, 0.00222];
    let y = vector![v_in, hn_out];
    let scale = b.dot(&y).max(f64::EPSILON);
    let c_out = rand_distr::Gamma::new(shape, scale).unwrap();
    let c_out = rand::Rng::sample(rng, c_out);

    // v_min parameter
    let a = vector![0.488, 0.0236, 0.0325];
    let x = vector![v_in, angle, hn_in];
    let mean = a.dot(&x);
    let b = vector![0.0261, 0.00689, 0.0];
    let y = vector![v_in, angle, hn_in];
    let std_dev = b.dot(&y).max(f64::EPSILON);
    let v_min = rand_distr::Normal::new(mean, std_dev).unwrap();
    let v_min = rand::Rng::sample(rng, v_min);

    // inflow
    let t_min = (2.0 / c_in * (v_in - v_min)).cbrt();
    let a = c_in;
    let b = -3.0 / 2.0 * a * t_min;
    let c = 0.0;
    let d = v_in;
    let mut t = 0.0;
    while t <= t_min {
        if t > MAX_TIME {
            break;
        }
        let point = [t, a * t.powi(3) + b * t.powi(2) + c * t + d];
        velocity_series.push(point);
        t += STEP;
    }

    // outflow
    let t_next = (2.0 / -c_out * (v_min - v_out)).cbrt();
    let a = -c_out;
    let b = -3.0 / 2.0 * a * t_next;
    let c = 0.0;
    let d = v_min;
    let mut t = 0.0;
    while t <= t_next {
        if t > MAX_TIME {
            break;
        }
        let point = [t + t_min, a * t.powi(3) + b * t.powi(2) + c * t + d];
        velocity_series.push(point);
        t += STEP;
    }

    let max_step = velocity_series.len();

    // x_min parameter
    let a = vector![0.917, 0.150, 0.218];
    let x = vector![v_in, angle, hn_in];
    let mean = a.dot(&x);
    let b = vector![-0.438, 0.0975, 0.101];
    let y = vector![v_in, angle, hn_in];
    let std_dev = b.dot(&y).max(f64::EPSILON);
    let x_min = rand_distr::Normal::new(mean, std_dev).unwrap();
    let x_min = rand::Rng::sample(rng, x_min);

    // position
    let mut x_min_ = 0.0;
    let mut position_series = vec![[0.0; 2]; max_step];
    for i in 1..max_step {
        let [t0, v0] = velocity_series[i - 1];
        let [t1, v1] = velocity_series[i];

        let [_, x0] = position_series[i - 1];

        if t0 <= t_min && t_min < t1 {
            x_min_ = x0;
        }

        let x1 = x0 + (t1 - t0) * (v0 + v1) * 0.5;
        position_series[i] = [(t0 + t1) * 0.5, x1];
    }
    let x_o = x_min_ - x_min;

    // dynamic r_min parameter
    let a = vector![0.0282, 0.0807];
    let x = vector![angle, f64::min(hn_in, hn_out)];
    let shape = a.dot(&x);
    let b = vector![0.162, 1.43];
    let y = vector![angle, v_min];
    let scale = b.dot(&y).max(f64::EPSILON);
    let r_min = rand_distr::Weibull::new(scale, shape).unwrap();
    let r_min = rand::Rng::sample(rng, r_min);

    // curvature
    let a = vector![6.09, 0.985, 0.186, 0.235, 0.0];
    let x = vector![1.0, v_min, r_min, hn_in, hn_out];
    let a1 = a.dot(&x);
    let a = vector![6.81, 0.611, 0.313, 0.0, 0.188];
    let x = vector![1.0, v_min, r_min, hn_in, hn_out];
    let a2 = a.dot(&x);
    let l_clothoid1 = r_min.recip() / a1.powi(2).recip();
    let angle_clothoid1 = 0.5 * a1.powi(2).recip() * l_clothoid1.powi(2);
    let l_clothoid2 = r_min.recip() / a2.powi(2).recip();
    let angle_clothoid2 = 0.5 * a2.powi(2).recip() * l_clothoid2.powi(2);
    let angle_arc = angle.to_radians() - (angle_clothoid1 + angle_clothoid2);
    let l_arc = angle_arc / r_min.recip();
    if l_arc < 0.0 {
        return None;
    }
    let x_0 = x_o;
    let x_1 = x_0 + l_clothoid1;
    let x_2 = x_1 + l_arc;
    let x_3 = x_2 + l_clothoid2;
    let mut curvature_series = vec![[0.0; 2]; max_step];
    for i in 0..max_step {
        let [_, x] = position_series[i];
        if x_0 <= x && x < x_1 {
            curvature_series[i] = [x, -a1.powi(2).recip() * (x - x_0)];
        } else if x_1 <= x && x < x_2 {
            curvature_series[i] = [x, -r_min.recip()];
        } else if x_2 <= x && x < x_3 {
            curvature_series[i] = [x, -(r_min.recip() - a2.powi(2).recip() * (x - x_2))];
        } else {
            curvature_series[i] = [x, 0.0];
        }
    }

    // trajectory
    let mut angle: f64 = 0.0;
    let mut position = [0.0; 2];
    let mut trajectory_series = vec![[0.0; 2]; max_step];
    for i in 0..max_step {
        let [_, v] = velocity_series[i];
        let [_, c] = curvature_series[i];

        let dx = v * STEP;
        angle += c * dx;
        position[0] += angle.cos() * dx;
        position[1] += angle.sin() * dx;

        trajectory_series[i] = position;
    }

    // origin time
    let mut t_o = 0.0;
    for w in position_series.windows(2) {
        let [t, x0] = w[0];
        let [_, x1] = w[1];

        if x0 <= x_o && x_o < x1 {
            t_o = t;
        }
    }

    // origin shift
    for [t, _] in velocity_series.iter_mut() {
        *t -= t_o;
    }
    for [t, x] in position_series.iter_mut() {
        *t -= t_o;
        *x -= x_o;
    }
    for [x, _] in curvature_series.iter_mut() {
        *x -= x_o;
    }

    // trajectory origin shift
    if trajectory_series.len() > 2 {
        let (start_idx, last_idx) = (0, trajectory_series.len() - 1);
        let trajectory_origin = intersection_point(
            trajectory_series[start_idx],
            trajectory_series[start_idx + 1],
            trajectory_series[last_idx],
            trajectory_series[last_idx - 1],
        );
        let tx = tx * Translation2::from(trajectory_origin).inverse();
        for p in trajectory_series.iter_mut() {
            *p = (tx * Point2::from(*p)).into();
        }
    }

    Some(VehOutput {
        c_in,
        c_out,
        v_min,
        x_min,
        t_min,
        t_exit: t_min + t_next,
        t_o,
        x_o,
        max_step,
        velocity_series,
        position_series,
        curvature_series,
        trajectory_series,
    })
}

#[derive(Clone, Debug)]
pub struct PedOutput {
    pub v_1: f64,
    pub v_2: f64,
    pub x_1: f64,
    pub x_2: f64,
    pub x_3: f64,
    pub max_step: usize,
    pub trajectory_series: Vec<[f64; 2]>,
}

pub fn compute_ped(settings: &Settings, flow: &PedFlow) -> Option<PedOutput> {
    let rng = &mut rand::thread_rng();

    let m = Rotation2::new(settings.angle.to_radians());

    // TODO: implement the following parameters
    let a_green = 0.0;

    let distr = rand_distr::Normal::new(flow.v_in_mean, flow.v_in_stdv).unwrap();
    let v_init = rand::Rng::sample(rng, distr);

    let distr = rand_distr::Normal::new(flow.x_in_mean, flow.x_in_stdv).unwrap();
    let x_init = rand::Rng::sample(rng, distr);

    let width = match (flow.src, flow.dst) {
        (Dir::NxNy, Dir::NxPy) | (Dir::NxPy, Dir::NxNy) => settings.width_along,
        (Dir::PyNx, Dir::PyPx) | (Dir::PyPx, Dir::PyNx) => settings.width_across,
        (Dir::PxNy, Dir::PxPy) | (Dir::PxPy, Dir::PxNy) => settings.width_along,
        (Dir::NyNx, Dir::NyPx) | (Dir::NyPx, Dir::NyNx) => settings.width_across,
        _ => unreachable!(),
    };

    let cw_width = match (flow.src, flow.dst) {
        (Dir::NxNy, Dir::NxPy) | (Dir::NxPy, Dir::NxNy) => settings.cw_width_along,
        (Dir::PyNx, Dir::PyPx) | (Dir::PyPx, Dir::PyNx) => settings.cw_width_across,
        (Dir::PxNy, Dir::PxPy) | (Dir::PxPy, Dir::PxNy) => settings.cw_width_along,
        (Dir::NyNx, Dir::NyPx) | (Dir::NyPx, Dir::NyNx) => settings.cw_width_across,
        _ => unreachable!(),
    };

    let cw_setback = match (flow.src, flow.dst) {
        (Dir::NxNy, Dir::NxPy) | (Dir::NxPy, Dir::NxNy) => settings.cw_setback_along,
        (Dir::PyNx, Dir::PyPx) | (Dir::PyPx, Dir::PyNx) => settings.cw_setback_across,
        (Dir::PxNy, Dir::PxPy) | (Dir::PxPy, Dir::PxNy) => settings.cw_setback_along,
        (Dir::NyNx, Dir::NyPx) | (Dir::NyPx, Dir::NyNx) => settings.cw_setback_across,
        _ => unreachable!(),
    };

    let far_side = match (flow.src, flow.dst) {
        (Dir::NxNy, Dir::NxPy)
        | (Dir::PyNx, Dir::PyPx)
        | (Dir::PxPy, Dir::PxNy)
        | (Dir::NyPx, Dir::NyNx) => true,
        (Dir::NxPy, Dir::NxNy)
        | (Dir::NyNx, Dir::NyPx)
        | (Dir::PxNy, Dir::PxPy)
        | (Dir::PyPx, Dir::PyNx) => false,
        _ => unreachable!(),
    };
    let far_side_dummy = if far_side { 1.0 } else { 0.0 };

    let distr = rand_distr::Uniform::new(0.0, 1.0);
    let diagonal = rand::Rng::sample(rng, distr) < flow.diagonal_prob;
    let diagonal_dummy = if diagonal { 1.0 } else { 0.0 };

    let center_side = x_init < cw_width * 0.5;
    let center_side_dummy = if center_side { 1.0 } else { 0.0 };

    // TODO: implement the following parameters
    let lt_veh_flow = 0.0;
    let forward_ped_flow = 0.0;
    let backward_ped_flow = 0.0;

    let tx = match (flow.src, flow.dst) {
        (Dir::NyNx, Dir::NyPx) => {
            let y = settings.width_across * 0.5;
            let x_min = -settings.cw_setback_across;
            let o = (m * Point2::new(x_min, y)) - Point2::origin();
            Isometry2::new(o, m.angle() + std::f64::consts::PI)
        }
        (Dir::PxNy, Dir::PxPy) => {
            let y = -settings.width_along * 0.5;
            let x_min = settings.cw_setback_along;
            let o = Point2::new(x_min, y) - Point2::origin();
            Isometry2::new(o, 0.0)
        }
        (Dir::PyPx, Dir::PyNx) => {
            let y = -settings.width_across * 0.5;
            let x_min = settings.cw_setback_across;
            let o = (m * Point2::new(x_min, y)) - Point2::origin();
            Isometry2::new(o, m.angle())
        }
        (Dir::NxPy, Dir::NxNy) => {
            let y = settings.width_along * 0.5;
            let x_min = -settings.cw_setback_along;
            let o = Point2::new(x_min, y) - Point2::origin();
            Isometry2::new(o, std::f64::consts::PI)
        }
        (Dir::NyPx, Dir::NyNx) => {
            let y = -settings.width_across * 0.5;
            let x_min = -settings.cw_setback_across - settings.cw_width_across;
            let o = (m * Point2::new(x_min, y)) - Point2::origin();
            Isometry2::new(o, m.angle())
        }
        (Dir::PxPy, Dir::PxNy) => {
            let y = settings.width_along * 0.5;
            let x_min = settings.cw_setback_along + settings.cw_width_along;
            let o = Point2::new(x_min, y) - Point2::origin();
            Isometry2::new(o, std::f64::consts::PI)
        }
        (Dir::PyNx, Dir::PyPx) => {
            let y = settings.width_across * 0.5;
            let x_min = settings.cw_setback_across + settings.cw_width_across;
            let o = (m * Point2::new(x_min, y)) - Point2::origin();
            Isometry2::new(o, m.angle() + std::f64::consts::PI)
        }
        (Dir::NxNy, Dir::NxPy) => {
            let y = -settings.width_along * 0.5;
            let x_min = -settings.cw_setback_along - settings.cw_width_along;
            let o = Point2::new(x_min, y) - Point2::origin();
            Isometry2::new(o, Default::default())
        }
        _ => unreachable!(),
    };

    // first half velocity
    let a = vector![7.47, 0.0, 0.720, 4.19, 1.93];
    let x = vector![v_init, 0.0, width, far_side_dummy, 1.0];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = vector![0.00391, 0.0, -0.00106, -0.00414, 0.00185, 0.0697];
    let y = vector![v_init, 0.0, width, far_side_dummy, a_green, 1.0];
    let scale = b.dot(&y).max(f64::EPSILON);
    let v_1 = rand_distr::Gamma::new(shape, scale).unwrap();
    let v_1 = rand::Rng::sample(rng, v_1);

    // last half velocity
    let a = vector![0.0, -2.10, 0.695, 4.10, 22.8];
    let x = vector![v_init, v_1, width, far_side_dummy, 1.0];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = vector![0.0, 0.0199, -0.0006, -0.00159, 0.0, 0.0256];
    let y = vector![v_init, v_1, width, far_side_dummy, a_green, 1.0];
    let scale = b.dot(&y).max(f64::EPSILON);
    let v_2 = rand_distr::Gamma::new(shape, scale).unwrap();
    let v_2 = rand::Rng::sample(rng, v_2);

    // first x
    let a = vector![0.210, -0.0200, -0.220, -1.03, -1.06, 0.100, -6.36, 0.0, 2.11];
    let x = vector![
        cw_width,
        cw_setback,
        far_side_dummy,
        diagonal_dummy,
        center_side_dummy,
        x_init,
        lt_veh_flow,
        forward_ped_flow,
        1.0
    ];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = vector![-0.0400, 0.0, 0.0, 0.0, -0.660, 2.31];
    let y = vector![
        width,
        cw_width,
        lt_veh_flow,
        forward_ped_flow,
        backward_ped_flow,
        1.0
    ];
    let scale = b.dot(&y).max(f64::EPSILON);
    let x_1 = rand_distr::Weibull::new(scale, shape).unwrap();
    let x_1 = rand::Rng::sample(rng, x_1).max(0.0).min(cw_width);

    // mid x
    let a = vector![-0.540, 0.0, 0.0, -0.390, 0.440, 0.830, 0.110, 2.16, 3.51];
    let x = vector![
        cw_width,
        cw_setback,
        far_side_dummy,
        diagonal_dummy,
        center_side_dummy,
        x_1,
        lt_veh_flow,
        forward_ped_flow,
        1.0
    ];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = vector![0.0, 1.13, 0.0, 0.0, -0.950, -1.86];
    let y = vector![
        width,
        cw_width,
        lt_veh_flow,
        forward_ped_flow,
        backward_ped_flow,
        1.0
    ];
    let scale = b.dot(&y).max(f64::EPSILON);
    let x_2 = rand_distr::Weibull::new(scale, shape).unwrap();
    let x_2 = rand::Rng::sample(rng, x_2).max(0.0).min(cw_width);

    // last x
    let a = vector![0.450, 0.0200, 0.150, -0.660, -0.220, 0.200, 0.0, 0.0, -1.19];
    let x = vector![
        cw_width,
        cw_setback,
        far_side_dummy,
        diagonal_dummy,
        center_side_dummy,
        x_2,
        lt_veh_flow,
        forward_ped_flow,
        1.0
    ];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = vector![0.0, 1.0, -10.5, 6.93, -1.69, -1.94];
    let y = vector![
        width,
        cw_width,
        lt_veh_flow,
        forward_ped_flow,
        forward_ped_flow + backward_ped_flow,
        1.0
    ];
    let scale = b.dot(&y).max(f64::EPSILON);
    let x_3 = rand_distr::Weibull::new(scale, shape).unwrap();
    let x_3 = rand::Rng::sample(rng, x_3).max(0.0).min(cw_width);

    let mut trajectory_series = vec![];
    let (mut x, mut y) = (0.0, 0.0);

    let dir = (point![x_1, width * 0.5] - point![x_init, 0.0]).normalize();
    while y <= width * 0.5 {
        x += dir.x * v_1 * STEP;
        y += dir.y * v_1 * STEP;
        let p = (tx * point![x, y]).into();
        trajectory_series.push(p);
    }
    let dir = (point![x_2, width] - point![x_1, width * 0.5]).normalize();
    while y <= width {
        x += dir.x * v_2 * STEP;
        y += dir.y * v_2 * STEP;
        let p = (tx * point![x, y]).into();
        trajectory_series.push(p);
    }

    let max_step = trajectory_series.len();

    Some(PedOutput {
        v_1,
        v_2,
        x_1,
        x_2,
        x_3,
        max_step,
        trajectory_series,
    })
}

pub fn compute_ig_ped(settings: &Settings, flow: &PedFlow) -> Option<PedOutput> {
    let rng = &mut rand::thread_rng();

    let m = Rotation2::new(settings.angle.to_radians());

    // TODO: implement the following parameters
    let t_blink = 0.0;

    let distr = rand_distr::Normal::new(flow.v_in_mean, flow.v_in_stdv).unwrap();
    let v_init = rand::Rng::sample(rng, distr);

    let distr = rand_distr::Normal::new(flow.x_in_mean, flow.x_in_stdv).unwrap();
    let x_init = rand::Rng::sample(rng, distr);

    let distr = rand_distr::Normal::new(flow.d_in_mean, flow.d_in_stdv).unwrap();
    let d_init = rand::Rng::sample(rng, distr);

    let width = match (flow.src, flow.dst) {
        (Dir::NxNy, Dir::NxPy) | (Dir::NxPy, Dir::NxNy) => settings.width_along,
        (Dir::PyNx, Dir::PyPx) | (Dir::PyPx, Dir::PyNx) => settings.width_across,
        (Dir::PxNy, Dir::PxPy) | (Dir::PxPy, Dir::PxNy) => settings.width_along,
        (Dir::NyNx, Dir::NyPx) | (Dir::NyPx, Dir::NyNx) => settings.width_across,
        _ => unreachable!(),
    };

    let cw_width = match (flow.src, flow.dst) {
        (Dir::NxNy, Dir::NxPy) | (Dir::NxPy, Dir::NxNy) => settings.cw_width_along,
        (Dir::PyNx, Dir::PyPx) | (Dir::PyPx, Dir::PyNx) => settings.cw_width_across,
        (Dir::PxNy, Dir::PxPy) | (Dir::PxPy, Dir::PxNy) => settings.cw_width_along,
        (Dir::NyNx, Dir::NyPx) | (Dir::NyPx, Dir::NyNx) => settings.cw_width_across,
        _ => unreachable!(),
    };

    let cw_setback = match (flow.src, flow.dst) {
        (Dir::NxNy, Dir::NxPy) | (Dir::NxPy, Dir::NxNy) => settings.cw_setback_along,
        (Dir::PyNx, Dir::PyPx) | (Dir::PyPx, Dir::PyNx) => settings.cw_setback_across,
        (Dir::PxNy, Dir::PxPy) | (Dir::PxPy, Dir::PxNy) => settings.cw_setback_along,
        (Dir::NyNx, Dir::NyPx) | (Dir::NyPx, Dir::NyNx) => settings.cw_setback_across,
        _ => unreachable!(),
    };

    let far_side = match (flow.src, flow.dst) {
        (Dir::NxNy, Dir::NxPy)
        | (Dir::PyNx, Dir::PyPx)
        | (Dir::PxPy, Dir::PxNy)
        | (Dir::NyPx, Dir::NyNx) => true,
        (Dir::NxPy, Dir::NxNy)
        | (Dir::NyNx, Dir::NyPx)
        | (Dir::PxNy, Dir::PxPy)
        | (Dir::PyPx, Dir::PyNx) => false,
        _ => unreachable!(),
    };
    let far_side_dummy = if far_side { 1.0 } else { 0.0 };

    let distr = rand_distr::Uniform::new(0.0, 1.0);
    let diagonal = rand::Rng::sample(rng, distr) < flow.diagonal_prob;
    let diagonal_dummy = if diagonal { 1.0 } else { 0.0 };

    let center_side = x_init < cw_width * 0.5;
    let center_side_dummy = if center_side { 1.0 } else { 0.0 };

    // TODO: implement the following parameters
    let lt_veh_flow = 0.0;
    let forward_ped_flow = 0.0;
    let backward_ped_flow = 0.0;

    let tx = match (flow.src, flow.dst) {
        (Dir::NyNx, Dir::NyPx) => {
            let y = settings.width_across * 0.5;
            let x_min = -settings.cw_setback_across;
            let o = (m * Point2::new(x_min, y)) - Point2::origin();
            Isometry2::new(o, m.angle() + std::f64::consts::PI)
        }
        (Dir::PxNy, Dir::PxPy) => {
            let y = -settings.width_along * 0.5;
            let x_min = settings.cw_setback_along;
            let o = Point2::new(x_min, y) - Point2::origin();
            Isometry2::new(o, 0.0)
        }
        (Dir::PyPx, Dir::PyNx) => {
            let y = -settings.width_across * 0.5;
            let x_min = settings.cw_setback_across;
            let o = (m * Point2::new(x_min, y)) - Point2::origin();
            Isometry2::new(o, m.angle())
        }
        (Dir::NxPy, Dir::NxNy) => {
            let y = settings.width_along * 0.5;
            let x_min = -settings.cw_setback_along;
            let o = Point2::new(x_min, y) - Point2::origin();
            Isometry2::new(o, std::f64::consts::PI)
        }
        (Dir::NyPx, Dir::NyNx) => {
            let y = -settings.width_across * 0.5;
            let x_min = -settings.cw_setback_across - settings.cw_width_across;
            let o = (m * Point2::new(x_min, y)) - Point2::origin();
            Isometry2::new(o, m.angle())
        }
        (Dir::PxPy, Dir::PxNy) => {
            let y = settings.width_along * 0.5;
            let x_min = settings.cw_setback_along + settings.cw_width_along;
            let o = Point2::new(x_min, y) - Point2::origin();
            Isometry2::new(o, std::f64::consts::PI)
        }
        (Dir::PyNx, Dir::PyPx) => {
            let y = settings.width_across * 0.5;
            let x_min = settings.cw_setback_across + settings.cw_width_across;
            let o = (m * Point2::new(x_min, y)) - Point2::origin();
            Isometry2::new(o, m.angle() + std::f64::consts::PI)
        }
        (Dir::NxNy, Dir::NxPy) => {
            let y = -settings.width_along * 0.5;
            let x_min = -settings.cw_setback_along - settings.cw_width_along;
            let o = Point2::new(x_min, y) - Point2::origin();
            Isometry2::new(o, Default::default())
        }
        _ => unreachable!(),
    };

    // contact velocity
    let a = nalgebra::matrix![0.256, 24.1];
    let x = nalgebra::matrix![d_init, 1.0];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = nalgebra::matrix![0.0379, 0.0218];
    let y = nalgebra::matrix![v_init, 1.0];
    let scale = b.dot(&y).max(f64::EPSILON);
    let v_0 = rand_distr::Gamma::new(shape, scale).unwrap();
    let v_0 = rand::Rng::sample(rng, v_0);

    // first half velocity
    let a = nalgebra::vector![3.88, 0.129, -3.51];
    let x = nalgebra::vector![v_0, width, 1.0];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = nalgebra::vector![-0.0144, 0.0158, 0.170];
    let y = nalgebra::vector![v_0, t_blink, 1.0];
    let scale = b.dot(&y).max(f64::EPSILON);
    let c = nalgebra::vector![-0.000055, 0.777];
    let z = nalgebra::vector![forward_ped_flow + backward_ped_flow, 1.0];
    let shift = c.dot(&z).max(f64::EPSILON);
    let v_1 = rand_distr::Gamma::new(shape, scale).unwrap();
    let v_1 = rand::Rng::sample(rng, v_1) + shift;

    // last half velocity
    let a = nalgebra::vector![0.580, 6.67];
    let x = nalgebra::vector![v_1, 1.0];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = nalgebra::vector![0.0862, -0.00333];
    let y = nalgebra::vector![v_1, 1.0];
    let scale = b.dot(&y).max(f64::EPSILON);
    let c = nalgebra::vector![0.218, -0.0597, 0.499];
    let z = nalgebra::vector![v_1, far_side_dummy, 1.0];
    let shift = c.dot(&z).max(f64::EPSILON);
    let v_2 = rand_distr::Gamma::new(shape, scale).unwrap();
    let v_2 = rand::Rng::sample(rng, v_2) + shift;

    // first x
    let a = nalgebra::vector![0.210, -0.0200, -0.220, -1.03, -1.06, 0.100, -6.36, 0.0, 2.11];
    let x = nalgebra::vector![
        cw_width,
        cw_setback,
        far_side_dummy,
        diagonal_dummy,
        center_side_dummy,
        x_init,
        lt_veh_flow,
        forward_ped_flow,
        1.0
    ];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = nalgebra::vector![-0.0400, 0.0, 0.0, 0.0, -0.660, 2.31];
    let y = nalgebra::vector![
        width,
        cw_width,
        lt_veh_flow,
        forward_ped_flow,
        backward_ped_flow,
        1.0
    ];
    let scale = b.dot(&y).max(f64::EPSILON);
    let x_1 = rand_distr::Weibull::new(scale, shape).unwrap();
    let x_1 = rand::Rng::sample(rng, x_1).max(0.0).min(cw_width);

    // mid x
    let a = nalgebra::vector![-0.540, 0.0, 0.0, -0.390, 0.440, 0.830, 0.110, 2.16, 3.51];
    let x = nalgebra::vector![
        cw_width,
        cw_setback,
        far_side_dummy,
        diagonal_dummy,
        center_side_dummy,
        x_1,
        lt_veh_flow,
        forward_ped_flow,
        1.0
    ];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = nalgebra::vector![0.0, 1.13, 0.0, 0.0, -0.950, -1.86];
    let y = nalgebra::vector![
        width,
        cw_width,
        lt_veh_flow,
        forward_ped_flow,
        backward_ped_flow,
        1.0
    ];
    let scale = b.dot(&y).max(f64::EPSILON);
    let x_2 = rand_distr::Weibull::new(scale, shape).unwrap();
    let x_2 = rand::Rng::sample(rng, x_2).max(0.0).min(cw_width);

    // last x
    let a = nalgebra::vector![0.450, 0.0200, 0.150, -0.660, -0.220, 0.200, 0.0, 0.0, -1.19];
    let x = nalgebra::vector![
        cw_width,
        cw_setback,
        far_side_dummy,
        diagonal_dummy,
        center_side_dummy,
        x_2,
        lt_veh_flow,
        forward_ped_flow,
        1.0
    ];
    let shape = a.dot(&x).max(f64::EPSILON);
    let b = nalgebra::vector![0.0, 1.0, -10.5, 6.93, -1.69, -1.94];
    let y = nalgebra::vector![
        width,
        cw_width,
        lt_veh_flow,
        forward_ped_flow,
        forward_ped_flow + backward_ped_flow,
        1.0
    ];
    let scale = b.dot(&y).max(f64::EPSILON);
    let x_3 = rand_distr::Weibull::new(scale, shape).unwrap();
    let x_3 = rand::Rng::sample(rng, x_3).max(0.0).min(cw_width);

    let mut trajectory_series = vec![];
    let (mut x, mut y) = (0.0, 0.0);

    let dir = (point![x_1, width * 0.5] - point![x_init, 0.0]).normalize();
    while y <= width * 0.5 {
        x += dir.x * v_1 * STEP;
        y += dir.y * v_1 * STEP;
        let p = (tx * point![x, y]).into();
        trajectory_series.push(p);
    }
    let dir = (point![x_2, width] - point![x_1, width * 0.5]).normalize();
    while y <= width {
        x += dir.x * v_2 * STEP;
        y += dir.y * v_2 * STEP;
        let p = (tx * point![x, y]).into();
        trajectory_series.push(p);
    }

    let max_step = trajectory_series.len();

    Some(PedOutput {
        v_1,
        v_2,
        x_1,
        x_2,
        x_3,
        max_step,
        trajectory_series,
    })
}
