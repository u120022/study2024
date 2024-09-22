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
