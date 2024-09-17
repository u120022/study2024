use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone)]
pub struct Simulator {
    field_var: FieldVar,
    lt_veh_var: LtVehVar,
    lt_veh_spawn_ready: f64,
    lt_veh_id: usize,
    pub lt_veh_agent: HashMap<usize, (LtVehData, usize, [f64; 2])>,
}

impl Simulator {
    pub fn new(field_var: FieldVar, lt_veh_var: LtVehVar) -> Self {
        Self {
            field_var,
            lt_veh_var,
            lt_veh_spawn_ready: Default::default(),
            lt_veh_id: Default::default(),
            lt_veh_agent: Default::default(),
        }
    }

    pub fn forward(&mut self, delta_time: f64) {
        // spawn left-turning vehicle
        if self.lt_veh_spawn_ready <= 0.0 {
            if let Some(data) = LtVehData::sample(&mut rand::thread_rng(), &self.lt_veh_var) {
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
        let r = nalgebra::Rotation2::new(self.field_var.angle.to_radians());
        let x = -self.field_var.width_along * 0.5;
        let p0: [f64; 2] = nalgebra::Point2::new(x, -1.0).into();
        let p1: [f64; 2] = nalgebra::Point2::new(x, 1.0).into();
        let x = *self.field_var.lane_across.last().unwrap();
        let q0: [f64; 2] = (r * nalgebra::Point2::new(x, -1.0)).into();
        let q1: [f64; 2] = (r * nalgebra::Point2::new(x, 1.0)).into();
        let o = math::intersection_point(p0, p1, q0, q1);

        for (_, (data, time, position)) in self.lt_veh_agent.iter_mut() {
            if *time >= data.trajectory_series.len() {
                continue;
            }

            position[0] = data.trajectory_series[*time][0] + o[0] + self.lt_veh_var.padding;
            position[1] = data.trajectory_series[*time][1] + o[1];

            *time += (delta_time / LtVehData::STEP) as usize;
        }
    }
}
