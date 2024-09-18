use crate::*;

#[derive(Debug, Clone, Default)]
pub struct SimulatorVar {
    pub angle: f64,
    pub radius: f64,
    pub width_along: f64,
    pub width_across: f64,
    pub lane_along: Vec<f64>,
    pub lane_across: Vec<f64>,
    pub hn_along: f64,
    pub hn_across: f64,
    pub cw_setback_along: f64,
    pub cw_setback_across: f64,
    pub cw_width_along: f64,
    pub cw_width_across: f64,
    pub sl_setback_along: f64,
    pub sl_setback_across: f64,
    pub lt_veh_v_in: f64,
    pub lt_veh_v_out: f64,
    pub lt_veh_padding: f64,
    pub lt_veh_large: bool,
    pub rt_veh_v_in: f64,
    pub rt_veh_v_out: f64,
    pub rt_veh_hn_in: f64,
    pub rt_veh_hn_out: f64,
    pub ped_a_green: f64,
    pub ped_v_init: f64,
    pub ped_x_init: f64,
    pub ped_far_side: bool,
    pub ped_diagonal: bool,
    pub ped_center_side: bool,
    pub ped_lt_veh_flow: f64,
    pub ped_forward_ped_flow: f64,
    pub ped_backward_ped_flow: f64,
    pub ig_ped_t_blink: f64,
    pub ig_ped_v_blink: f64,
    pub ig_ped_l_init: f64,
    pub ig_ped_x_init: f64,
    pub ig_ped_far_side: bool,
    pub ig_ped_diagonal: bool,
    pub ig_ped_center_side: bool,
    pub ig_ped_lt_car_flow: f64,
    pub ig_ped_forward_ped_flow: f64,
    pub ig_ped_backward_ped_flow: f64,
}

impl From<SimulatorVar> for FieldVar {
    fn from(value: SimulatorVar) -> Self {
        Self {
            angle: value.angle,
            radius: value.radius,
            width_along: value.width_along,
            width_across: value.width_across,
            lane_along: value.lane_along,
            lane_across: value.lane_across,
            hn_along: value.hn_along,
            hn_across: value.hn_across,
            cw_setback_along: value.cw_setback_along,
            cw_setback_across: value.cw_setback_across,
            cw_width_along: value.cw_width_along,
            cw_width_across: value.cw_width_across,
            sl_setback_along: value.sl_setback_along,
            sl_setback_across: value.sl_setback_across,
        }
    }
}

impl From<SimulatorVar> for LtVehVar {
    fn from(value: SimulatorVar) -> Self {
        Self {
            v_in: value.lt_veh_v_in,
            v_out: value.lt_veh_v_out,
            angle: value.angle,
            radius: value.radius,
            padding: value.lt_veh_padding,
            large: value.lt_veh_large,
        }
    }
}

impl From<SimulatorVar> for RtVehVar {
    fn from(value: SimulatorVar) -> Self {
        Self {
            v_in: value.rt_veh_v_in,
            v_out: value.rt_veh_v_out,
            angle: value.angle,
            hn_in: value.rt_veh_hn_in,
            hn_out: value.rt_veh_hn_out,
        }
    }
}

impl From<SimulatorVar> for PedVar {
    fn from(value: SimulatorVar) -> Self {
        Self {
            a_green: value.ped_a_green,
            v_init: value.ped_v_init,
            x_init: value.ped_x_init,
            width: value.width_along,
            cw_width: value.cw_width_along,
            cw_setback: value.cw_setback_along,
            far_side: value.ped_far_side,
            diagonal: value.ped_diagonal,
            center_side: value.ped_center_side,
            lt_veh_flow: value.ped_lt_veh_flow,
            forward_ped_flow: value.ped_forward_ped_flow,
            backward_ped_flow: value.ped_backward_ped_flow,
        }
    }
}

impl From<SimulatorVar> for IgPedVar {
    fn from(value: SimulatorVar) -> Self {
        Self {
            t_blink: value.ig_ped_t_blink,
            v_blink: value.ig_ped_v_blink,
            l_init: value.ig_ped_l_init,
            x_init: value.ig_ped_x_init,
            width: value.width_along,
            cw_width: value.cw_width_along,
            cw_setback: value.cw_setback_along,
            far_side: value.ig_ped_far_side,
            diagonal: value.ig_ped_diagonal,
            center_side: value.ig_ped_center_side,
            lt_car_flow: value.ig_ped_lt_car_flow,
            forward_ped_flow: value.ig_ped_forward_ped_flow,
            backward_ped_flow: value.ig_ped_backward_ped_flow,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SimulatorData {
    lt_veh_spawn_ready: f64,
    lt_veh_id: usize,
    lt_veh_agent: std::collections::HashMap<usize, (LtVehData, usize, [f64; 2])>,
}

impl SimulatorData {
    pub fn forward(&mut self, var: &SimulatorVar, delta_time: f64) {
        // spawn left-turning vehicle
        if self.lt_veh_spawn_ready <= 0.0 {
            if let Some(data) = LtVehData::sample(&mut rand::thread_rng(), &var.clone().into()) {
                self.lt_veh_agent.insert(
                    self.lt_veh_id,
                    (data, Default::default(), Default::default()),
                );
                self.lt_veh_id += 1;

                self.lt_veh_spawn_ready = 4.0;
            }
        }
        self.lt_veh_spawn_ready -= delta_time;

        // left-turning point
        let r = nalgebra::Rotation2::new(var.angle.to_radians());
        let x = -var.width_along * 0.5;
        let p0: [f64; 2] = nalgebra::Point2::new(x, -1.0).into();
        let p1: [f64; 2] = nalgebra::Point2::new(x, 1.0).into();
        let x = *var.lane_across.last().unwrap();
        let q0: [f64; 2] = (r * nalgebra::Point2::new(x, -1.0)).into();
        let q1: [f64; 2] = (r * nalgebra::Point2::new(x, 1.0)).into();
        let o = math::intersection_point(p0, p1, q0, q1);

        for (_, (data, time, position)) in self.lt_veh_agent.iter_mut() {
            if *time >= data.trajectory_series.len() {
                continue;
            }

            position[0] = data.trajectory_series[*time][0] + o[0] + var.lt_veh_padding;
            position[1] = data.trajectory_series[*time][1] + o[1];

            *time += (delta_time / LtVehData::STEP) as usize;
        }
    }
}
