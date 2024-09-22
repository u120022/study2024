use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehFlowDir {
    Nx2Px(usize, usize),
    Px2Nx(usize, usize),

    Ny2Py(usize, usize),
    Py2Ny(usize, usize),

    Nx2Py(usize, usize),
    Py2Px(usize, usize),
    Px2Ny(usize, usize),
    Ny2Nx(usize, usize),

    Nx2Ny(usize, usize),
    Ny2Px(usize, usize),
    Px2Py(usize, usize),
    Py2Nx(usize, usize),
}

#[derive(Debug, Clone)]
pub struct VehFlow {
    pub dir: VehFlowDir,
    pub density: f64,
    pub v_in_mean: f64,
    pub v_in_stdv: f64,
    pub v_out_mean: f64,
    pub v_out_stdv: f64,
    pub padding_out_mean: f64,
    pub padding_out_stdv: f64,
    pub large_prob: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PedFlowDir {
    NxNy2PxNy,
    PxNy2NxNy,

    PxNy2PxPy,
    PxPy2PxNy,

    PxPy2NxPy,
    NxPy2PxPy,

    NxPy2NxNy,
    NxNy2NxPy,
}

impl PedFlowDir {
    fn reverse(&self) -> PedFlowDir {
        match self {
            PedFlowDir::NxNy2PxNy => PedFlowDir::PxNy2NxNy,
            PedFlowDir::PxNy2NxNy => PedFlowDir::NxNy2PxNy,

            PedFlowDir::PxNy2PxPy => PedFlowDir::PxPy2PxNy,
            PedFlowDir::PxPy2PxNy => PedFlowDir::PxNy2PxPy,

            PedFlowDir::PxPy2NxPy => PedFlowDir::NxPy2PxPy,
            PedFlowDir::NxPy2PxPy => PedFlowDir::PxPy2NxPy,

            PedFlowDir::NxPy2NxNy => PedFlowDir::NxNy2NxPy,
            PedFlowDir::NxNy2NxPy => PedFlowDir::NxPy2NxNy,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PedFlow {
    pub dir: PedFlowDir,
    pub density: f64,
    pub v_in_mean: f64,
    pub v_in_stdv: f64,
    pub x_in_mean: f64,
    pub x_in_stdv: f64,
    pub d_in_mean: f64,
    pub d_in_stdv: f64,
    pub diagonal_prob: f64,
}

#[derive(Debug, Clone)]
pub struct SimVar {
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
    pub lt_veh_flows: Vec<VehFlow>,
    pub rt_veh_flows: Vec<VehFlow>,
    pub ped_flows: Vec<PedFlow>,
    pub ig_ped_flows: Vec<PedFlow>,
}

impl Default for SimVar {
    fn default() -> Self {
        Self {
            angle: 90.0,
            radius: 14.0,
            width_along: 17.0,
            width_across: 17.0,
            lane_along: vec![-6.25, -2.75, 2.75, 6.25],
            lane_across: vec![-6.25, -2.75, 2.75, 6.25],
            hn_along: 10.0,
            hn_across: 10.0,
            cw_setback_along: 13.0,
            cw_setback_across: 13.0,
            cw_width_along: 4.5,
            cw_width_across: 4.5,
            sl_setback_along: 19.0,
            sl_setback_across: 19.0,
            lt_veh_flows: vec![
                VehFlow {
                    dir: VehFlowDir::Nx2Py(3, 3),
                    density: 0.1,
                    v_in_mean: 10.0,
                    v_in_stdv: 1.0,
                    v_out_mean: 10.0,
                    v_out_stdv: 1.0,
                    padding_out_mean: 1.75,
                    padding_out_stdv: 0.1,
                    large_prob: 0.1,
                },
                VehFlow {
                    dir: VehFlowDir::Py2Px(0, 3),
                    density: 0.1,
                    v_in_mean: 10.0,
                    v_in_stdv: 1.0,
                    v_out_mean: 10.0,
                    v_out_stdv: 1.0,
                    padding_out_mean: 1.75,
                    padding_out_stdv: 0.1,
                    large_prob: 0.1,
                },
                VehFlow {
                    dir: VehFlowDir::Px2Ny(0, 0),
                    density: 0.1,
                    v_in_mean: 10.0,
                    v_in_stdv: 1.0,
                    v_out_mean: 10.0,
                    v_out_stdv: 1.0,
                    padding_out_mean: 1.75,
                    padding_out_stdv: 0.1,
                    large_prob: 0.1,
                },
                VehFlow {
                    dir: VehFlowDir::Ny2Nx(3, 0),
                    density: 0.1,
                    v_in_mean: 10.0,
                    v_in_stdv: 1.0,
                    v_out_mean: 10.0,
                    v_out_stdv: 1.0,
                    padding_out_mean: 1.75,
                    padding_out_stdv: 0.1,
                    large_prob: 0.1,
                },
            ],
            rt_veh_flows: vec![
                VehFlow {
                    dir: VehFlowDir::Nx2Ny(2, 1),
                    density: 0.1,
                    v_in_mean: 10.0,
                    v_in_stdv: 1.0,
                    v_out_mean: 10.0,
                    v_out_stdv: 1.0,
                    padding_out_mean: 1.75,
                    padding_out_stdv: 0.1,
                    large_prob: 0.1,
                },
                VehFlow {
                    dir: VehFlowDir::Ny2Px(2, 2),
                    density: 0.1,
                    v_in_mean: 10.0,
                    v_in_stdv: 1.0,
                    v_out_mean: 10.0,
                    v_out_stdv: 1.0,
                    padding_out_mean: 1.75,
                    padding_out_stdv: 0.1,
                    large_prob: 0.1,
                },
                VehFlow {
                    dir: VehFlowDir::Px2Py(1, 2),
                    density: 0.1,
                    v_in_mean: 10.0,
                    v_in_stdv: 1.0,
                    v_out_mean: 10.0,
                    v_out_stdv: 1.0,
                    padding_out_mean: 1.75,
                    padding_out_stdv: 0.1,
                    large_prob: 0.1,
                },
                VehFlow {
                    dir: VehFlowDir::Py2Nx(1, 1),
                    density: 0.1,
                    v_in_mean: 10.0,
                    v_in_stdv: 1.0,
                    v_out_mean: 10.0,
                    v_out_stdv: 1.0,
                    padding_out_mean: 1.75,
                    padding_out_stdv: 0.1,
                    large_prob: 0.1,
                },
            ],
            ped_flows: vec![
                PedFlow {
                    dir: PedFlowDir::NxNy2PxNy,
                    density: 0.1,
                    v_in_mean: 1.0,
                    v_in_stdv: 0.1,
                    x_in_mean: 0.0,
                    x_in_stdv: 0.1,
                    d_in_mean: 0.0,
                    d_in_stdv: 0.1,
                    diagonal_prob: 0.1,
                },
                PedFlow {
                    dir: PedFlowDir::PxNy2PxPy,
                    density: 0.1,
                    v_in_mean: 1.0,
                    v_in_stdv: 0.1,
                    x_in_mean: 0.0,
                    x_in_stdv: 0.1,
                    d_in_mean: 0.0,
                    d_in_stdv: 0.1,
                    diagonal_prob: 0.1,
                },
                PedFlow {
                    dir: PedFlowDir::PxPy2NxPy,
                    density: 0.1,
                    v_in_mean: 1.0,
                    v_in_stdv: 0.1,
                    x_in_mean: 0.0,
                    x_in_stdv: 0.1,
                    d_in_mean: 0.0,
                    d_in_stdv: 0.1,
                    diagonal_prob: 0.1,
                },
                PedFlow {
                    dir: PedFlowDir::NxPy2NxNy,
                    density: 0.1,
                    v_in_mean: 1.0,
                    v_in_stdv: 0.1,
                    x_in_mean: 0.0,
                    x_in_stdv: 0.1,
                    d_in_mean: 0.0,
                    d_in_stdv: 0.1,
                    diagonal_prob: 0.1,
                },
                PedFlow {
                    dir: PedFlowDir::PxNy2NxNy,
                    density: 0.1,
                    v_in_mean: 1.0,
                    v_in_stdv: 0.1,
                    x_in_mean: 0.0,
                    x_in_stdv: 0.1,
                    d_in_mean: 0.0,
                    d_in_stdv: 0.1,
                    diagonal_prob: 0.1,
                },
                PedFlow {
                    dir: PedFlowDir::PxPy2PxNy,
                    density: 0.1,
                    v_in_mean: 1.0,
                    v_in_stdv: 0.1,
                    x_in_mean: 0.0,
                    x_in_stdv: 0.1,
                    d_in_mean: 0.0,
                    d_in_stdv: 0.1,
                    diagonal_prob: 0.1,
                },
                PedFlow {
                    dir: PedFlowDir::NxPy2PxPy,
                    density: 0.1,
                    v_in_mean: 1.0,
                    v_in_stdv: 0.1,
                    x_in_mean: 0.0,
                    x_in_stdv: 0.1,
                    d_in_mean: 0.0,
                    d_in_stdv: 0.1,
                    diagonal_prob: 0.1,
                },
                PedFlow {
                    dir: PedFlowDir::NxNy2NxPy,
                    density: 0.1,
                    v_in_mean: 1.0,
                    v_in_stdv: 0.1,
                    x_in_mean: 0.0,
                    x_in_stdv: 0.1,
                    d_in_mean: 0.0,
                    d_in_stdv: 0.1,
                    diagonal_prob: 0.1,
                },
            ],
            ig_ped_flows: vec![],
        }
    }
}

impl SimVar {
    // fn gen_field_var(&self) -> FieldVar {
    //     FieldVar {
    //         angle: self.angle,
    //         radius: self.radius,
    //         width_along: self.width_along,
    //         width_across: self.width_across,
    //         lane_along: self.lane_along.clone(),
    //         lane_across: self.lane_across.clone(),
    //         hn_along: self.hn_along,
    //         hn_across: self.hn_across,
    //         cw_setback_along: self.cw_setback_along,
    //         cw_setback_across: self.cw_setback_across,
    //         cw_width_along: self.cw_width_along,
    //         cw_width_across: self.cw_width_across,
    //         sl_setback_along: self.sl_setback_along,
    //         sl_setback_across: self.sl_setback_across,
    //     }
    // }

    fn gen_lt_veh_var(&self, rng: &mut impl rand::Rng, index: usize) -> LtVehVar {
        let flow = &self.lt_veh_flows[index];

        let distr = rand_distr::Normal::new(flow.v_in_mean, flow.v_in_stdv).unwrap();
        let v_in = rand::Rng::sample(rng, distr);

        let distr = rand_distr::Normal::new(flow.v_out_mean, flow.v_out_stdv).unwrap();
        let v_out = rand::Rng::sample(rng, distr);

        let distr = rand_distr::Normal::new(flow.padding_out_mean, flow.padding_out_stdv).unwrap();
        let padding = rand::Rng::sample(rng, distr);

        let distr = rand_distr::Uniform::new(0.0, 1.0);
        let large = rand::Rng::sample(rng, distr) < flow.large_prob;

        let angle = match flow.dir {
            VehFlowDir::Nx2Py(_, _) => self.angle,
            VehFlowDir::Py2Px(_, _) => 180.0 - self.angle,
            VehFlowDir::Px2Ny(_, _) => self.angle,
            VehFlowDir::Ny2Nx(_, _) => 180.0 - self.angle,
            _ => unreachable!(),
        };

        LtVehVar {
            v_in,
            v_out,
            angle,
            radius: self.radius,
            padding,
            large,
        }
    }

    fn gen_rt_veh_var(&self, rng: &mut impl rand::Rng, index: usize) -> RtVehVar {
        let flow = &self.rt_veh_flows[index];

        let distr = rand_distr::Normal::new(flow.v_in_mean, flow.v_in_stdv).unwrap();
        let v_in = rand::Rng::sample(rng, distr);

        let distr = rand_distr::Normal::new(flow.v_out_mean, flow.v_out_stdv).unwrap();
        let v_out = rand::Rng::sample(rng, distr);

        let distr = rand_distr::Normal::new(flow.padding_out_mean, flow.padding_out_stdv).unwrap();
        let padding = rand::Rng::sample(rng, distr);

        let (hn_in, hn_out) = match flow.dir {
            VehFlowDir::Nx2Ny(_, _) => (self.hn_along, self.hn_across),
            VehFlowDir::Ny2Px(_, _) => (self.hn_across, self.hn_along),
            VehFlowDir::Px2Py(_, _) => (self.hn_along, self.hn_across),
            VehFlowDir::Py2Nx(_, _) => (self.hn_across, self.hn_along),
            _ => unreachable!(),
        };

        let angle = match flow.dir {
            VehFlowDir::Nx2Ny(_, _) => 180.0 - self.angle,
            VehFlowDir::Ny2Px(_, _) => self.angle,
            VehFlowDir::Px2Py(_, _) => 180.0 - self.angle,
            VehFlowDir::Py2Nx(_, _) => self.angle,
            _ => unreachable!(),
        };

        RtVehVar {
            v_in,
            v_out,
            angle,
            hn_in,
            hn_out,
            padding,
        }
    }

    fn gen_ped_var(&self, rng: &mut impl rand::Rng, index: usize) -> PedVar {
        let flow = &self.ped_flows[index];

        let distr = rand_distr::Uniform::new(0.0, 1.0);
        let a_green = rand::Rng::sample(rng, distr);

        let distr = rand_distr::Normal::new(flow.v_in_mean, flow.v_in_stdv).unwrap();
        let v_init = rand::Rng::sample(rng, distr);

        let distr = rand_distr::Normal::new(flow.x_in_mean, flow.x_in_stdv).unwrap();
        let x_init = rand::Rng::sample(rng, distr);

        let width = match flow.dir {
            PedFlowDir::NxNy2PxNy | PedFlowDir::PxNy2NxNy => self.width_across,
            PedFlowDir::PxNy2PxPy | PedFlowDir::PxPy2PxNy => self.width_along,
            PedFlowDir::PxPy2NxPy | PedFlowDir::NxPy2PxPy => self.width_across,
            PedFlowDir::NxPy2NxNy | PedFlowDir::NxNy2NxPy => self.width_along,
        };

        let cw_width = match flow.dir {
            PedFlowDir::NxNy2PxNy | PedFlowDir::PxNy2NxNy => self.cw_width_across,
            PedFlowDir::PxNy2PxPy | PedFlowDir::PxPy2PxNy => self.cw_width_along,
            PedFlowDir::PxPy2NxPy | PedFlowDir::NxPy2PxPy => self.cw_width_across,
            PedFlowDir::NxPy2NxNy | PedFlowDir::NxNy2NxPy => self.cw_width_along,
        };

        let cw_setback = match flow.dir {
            PedFlowDir::NxNy2PxNy | PedFlowDir::PxNy2NxNy => self.cw_setback_across,
            PedFlowDir::PxNy2PxPy | PedFlowDir::PxPy2PxNy => self.cw_setback_along,
            PedFlowDir::PxPy2NxPy | PedFlowDir::NxPy2PxPy => self.cw_setback_across,
            PedFlowDir::NxPy2NxNy | PedFlowDir::NxNy2NxPy => self.cw_setback_along,
        };

        let far_side = match flow.dir {
            PedFlowDir::PxNy2NxNy
            | PedFlowDir::PxPy2PxNy
            | PedFlowDir::NxPy2PxPy
            | PedFlowDir::NxNy2NxPy => true,
            PedFlowDir::NxNy2PxNy
            | PedFlowDir::PxNy2PxPy
            | PedFlowDir::PxPy2NxPy
            | PedFlowDir::NxPy2NxNy => false,
        };

        let distr = rand_distr::Uniform::new(0.0, 1.0);
        let diagonal = rand::Rng::sample(rng, distr) < flow.diagonal_prob;

        let center_side = x_init < cw_width * 0.5;

        let mut lt_veh_flow = 0.0;
        for sub_flow in &self.lt_veh_flows {
            match (sub_flow.dir, flow.dir) {
                (VehFlowDir::Nx2Py(_, _), PedFlowDir::NxPy2PxPy)
                | (VehFlowDir::Nx2Py(_, _), PedFlowDir::PxPy2NxPy) => {
                    lt_veh_flow += sub_flow.density;
                }
                (VehFlowDir::Py2Px(_, _), PedFlowDir::PxPy2PxNy)
                | (VehFlowDir::Py2Px(_, _), PedFlowDir::PxNy2PxPy) => {
                    lt_veh_flow += sub_flow.density;
                }
                (VehFlowDir::Px2Ny(_, _), PedFlowDir::PxNy2NxNy)
                | (VehFlowDir::Px2Ny(_, _), PedFlowDir::NxNy2PxNy) => {
                    lt_veh_flow += sub_flow.density;
                }
                (VehFlowDir::Ny2Nx(_, _), PedFlowDir::NxNy2NxPy)
                | (VehFlowDir::Ny2Nx(_, _), PedFlowDir::NxPy2NxNy) => {
                    lt_veh_flow += sub_flow.density;
                }
                _ => {}
            }
        }

        let mut forward_ped_flow = 0.0;
        let mut backward_ped_flow = 0.0;
        for sub_flow in &self.ped_flows {
            if sub_flow.dir == flow.dir {
                forward_ped_flow += sub_flow.density;
            }
            if sub_flow.dir == flow.dir.reverse() {
                backward_ped_flow += sub_flow.density;
            }
        }

        PedVar {
            a_green,
            v_init,
            x_init,
            width,
            cw_width,
            cw_setback,
            far_side,
            diagonal,
            center_side,
            lt_veh_flow,
            forward_ped_flow,
            backward_ped_flow,
        }
    }

    fn gen_id_ped_var(&self, rng: &mut impl rand::Rng, index: usize) -> IgPedVar {
        let flow = &self.ig_ped_flows[index];

        let distr = rand_distr::Normal::new(0.0, 10.0).unwrap();
        let t_blink = rand::Rng::sample(rng, distr);

        let distr = rand_distr::Normal::new(flow.v_in_mean, flow.v_in_stdv).unwrap();
        let v_blink = rand::Rng::sample(rng, distr);

        let distr = rand_distr::Normal::new(flow.x_in_mean, flow.x_in_stdv).unwrap();
        let x_init = rand::Rng::sample(rng, distr);

        let distr = rand_distr::Normal::new(flow.d_in_mean, flow.d_in_stdv).unwrap();
        let l_init = rand::Rng::sample(rng, distr);

        let width = match flow.dir {
            PedFlowDir::NxNy2PxNy | PedFlowDir::PxNy2NxNy => self.width_across,
            PedFlowDir::PxNy2PxPy | PedFlowDir::PxPy2PxNy => self.width_along,
            PedFlowDir::PxPy2NxPy | PedFlowDir::NxPy2PxPy => self.width_across,
            PedFlowDir::NxPy2NxNy | PedFlowDir::NxNy2NxPy => self.width_along,
        };

        let cw_width = match flow.dir {
            PedFlowDir::NxNy2PxNy | PedFlowDir::PxNy2NxNy => self.cw_width_across,
            PedFlowDir::PxNy2PxPy | PedFlowDir::PxPy2PxNy => self.cw_width_along,
            PedFlowDir::PxPy2NxPy | PedFlowDir::NxPy2PxPy => self.cw_width_across,
            PedFlowDir::NxPy2NxNy | PedFlowDir::NxNy2NxPy => self.cw_width_along,
        };

        let cw_setback = match flow.dir {
            PedFlowDir::NxNy2PxNy | PedFlowDir::PxNy2NxNy => self.cw_setback_across,
            PedFlowDir::PxNy2PxPy | PedFlowDir::PxPy2PxNy => self.cw_setback_along,
            PedFlowDir::PxPy2NxPy | PedFlowDir::NxPy2PxPy => self.cw_setback_across,
            PedFlowDir::NxPy2NxNy | PedFlowDir::NxNy2NxPy => self.cw_setback_along,
        };

        let far_side = match flow.dir {
            PedFlowDir::PxNy2NxNy
            | PedFlowDir::PxPy2PxNy
            | PedFlowDir::NxPy2PxPy
            | PedFlowDir::NxNy2NxPy => true,
            PedFlowDir::NxNy2PxNy
            | PedFlowDir::PxNy2PxPy
            | PedFlowDir::PxPy2NxPy
            | PedFlowDir::NxPy2NxNy => false,
        };

        let distr = rand_distr::Uniform::new(0.0, 1.0);
        let diagonal = rand::Rng::sample(rng, distr) < flow.diagonal_prob;

        let center_side = x_init < cw_width * 0.5;

        let mut lt_veh_flow = 0.0;
        for sub_flow in &self.lt_veh_flows {
            match (sub_flow.dir, flow.dir) {
                (VehFlowDir::Nx2Py(_, _), PedFlowDir::NxPy2PxPy)
                | (VehFlowDir::Nx2Py(_, _), PedFlowDir::PxPy2NxPy) => {
                    lt_veh_flow += sub_flow.density;
                }
                (VehFlowDir::Py2Px(_, _), PedFlowDir::PxPy2PxNy)
                | (VehFlowDir::Py2Px(_, _), PedFlowDir::PxNy2PxPy) => {
                    lt_veh_flow += sub_flow.density;
                }
                (VehFlowDir::Px2Ny(_, _), PedFlowDir::PxNy2NxNy)
                | (VehFlowDir::Px2Ny(_, _), PedFlowDir::NxNy2PxNy) => {
                    lt_veh_flow += sub_flow.density;
                }
                (VehFlowDir::Ny2Nx(_, _), PedFlowDir::NxNy2NxPy)
                | (VehFlowDir::Ny2Nx(_, _), PedFlowDir::NxPy2NxNy) => {
                    lt_veh_flow += sub_flow.density;
                }
                _ => unreachable!(),
            }
        }

        let mut forward_ped_flow = 0.0;
        let mut backward_ped_flow = 0.0;
        for sub_flow in &self.ped_flows {
            if sub_flow.dir == flow.dir {
                forward_ped_flow += sub_flow.density;
            }
            if sub_flow.dir == flow.dir.reverse() {
                backward_ped_flow += sub_flow.density;
            }
        }

        IgPedVar {
            t_blink,
            v_blink,
            l_init,
            x_init,
            width,
            cw_width,
            cw_setback,
            far_side,
            diagonal,
            center_side,
            lt_veh_flow,
            forward_ped_flow,
            backward_ped_flow,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct LtVehGroup {
    spawn_next_secs: f64,
    spawn_wait_secs: f64,
    var_vec: Vec<LtVehVar>,
    data_vec: Vec<LtVehData>,
    step_vec: Vec<usize>,
    transform_vec: Vec<nalgebra::Isometry2<f64>>,
}

#[derive(Debug, Clone, Default)]
struct RtVehGroup {
    spawn_next_secs: f64,
    spawn_wait_secs: f64,
    var_vec: Vec<RtVehVar>,
    data_vec: Vec<RtVehData>,
    step_vec: Vec<usize>,
    transform_vec: Vec<nalgebra::Isometry2<f64>>,
}

#[derive(Debug, Clone, Default)]
struct PedGroup {
    spawn_next_secs: f64,
    spawn_wait_secs: f64,
    var_vec: Vec<PedVar>,
    data_vec: Vec<PedData>,
    step_vec: Vec<usize>,
    transform_vec: Vec<nalgebra::Isometry2<f64>>,
}

#[derive(Debug, Clone)]
pub struct SimRuntime {
    var: SimVar,
    lt_veh_sim: Vec<LtVehGroup>,
    rt_veh_sim: Vec<RtVehGroup>,
    ped_sim: Vec<PedGroup>,
}

impl SimRuntime {
    pub fn new(var: SimVar) -> Self {
        let lt_veh_sim = vec![Default::default(); var.lt_veh_flows.len()];
        let rt_veh_sim = vec![Default::default(); var.rt_veh_flows.len()];
        let ped_sim = vec![Default::default(); var.ped_flows.len()];

        Self {
            var,
            lt_veh_sim,
            rt_veh_sim,
            ped_sim,
        }
    }

    pub fn forward(&mut self, delta_secs: f64) {
        const ROAD_LENGTH: f64 = 30.0;

        let mut rng = rand::thread_rng();
        let r = nalgebra::Rotation2::new(self.var.angle.to_radians());

        // left-turned vehicle
        let flow_len = self.var.lt_veh_flows.len();
        for i in 0..flow_len {
            let flow = &self.var.lt_veh_flows[i];

            let sim = &mut self.lt_veh_sim[i];

            sim.spawn_wait_secs += delta_secs;

            if sim.spawn_wait_secs > sim.spawn_next_secs {
                let var = self.var.gen_lt_veh_var(&mut rng, i);
                let Some(data) = LtVehData::sample(&mut rng, &var) else {
                    continue;
                };
                let step = (data.t_o / LtVehData::STEP) as usize;

                let transform = match flow.dir {
                    VehFlowDir::Nx2Py(src, _) => {
                        let y = self.var.lane_along[src];
                        let p0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
                        let p1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();

                        let y = self.var.width_across * 0.5 - var.padding;
                        let q0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
                        let q1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();

                        let origin = math::intersection_point(p0, p1, q0, q1);
                        nalgebra::Isometry2::new(origin.into(), Default::default())
                    }
                    VehFlowDir::Py2Px(src, _) => {
                        let y = self.var.lane_across[src];
                        let p0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
                        let p1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();

                        let y = self.var.width_along * 0.5 - var.padding;
                        let q0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
                        let q1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();

                        let origin = math::intersection_point(p0, p1, q0, q1);
                        nalgebra::Isometry2::new(origin.into(), r.angle() + std::f64::consts::PI)
                    }
                    VehFlowDir::Px2Ny(src, _) => {
                        let y = self.var.lane_along[src];
                        let p0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
                        let p1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();

                        let y = -(self.var.width_across * 0.5 - var.padding);
                        let q0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
                        let q1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();

                        let origin = math::intersection_point(p0, p1, q0, q1);
                        nalgebra::Isometry2::new(origin.into(), std::f64::consts::PI)
                    }
                    VehFlowDir::Ny2Nx(src, _) => {
                        let y = self.var.lane_across[src];
                        let p0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
                        let p1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();

                        let y = -(self.var.width_along * 0.5 - var.padding);
                        let q0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
                        let q1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();

                        let origin = math::intersection_point(p0, p1, q0, q1);
                        nalgebra::Isometry2::new(origin.into(), r.angle())
                    }
                    _ => unreachable!(),
                };

                sim.var_vec.push(var);
                sim.data_vec.push(data);
                sim.step_vec.push(step);
                sim.transform_vec.push(transform);

                let lambda = flow.density * flow.v_in_mean;
                let distr = rand_distr::Exp::new(lambda).unwrap();
                sim.spawn_next_secs = rand::Rng::sample(&mut rng, distr);
                sim.spawn_wait_secs = 0.0;
            }

            let data_len = sim.data_vec.len();

            for i in 0..data_len {
                sim.step_vec[i] += (delta_secs / LtVehData::STEP) as usize;
            }

            let mut remove_queue = vec![];
            for i in 0..data_len {
                if sim.step_vec[i] >= sim.data_vec[i].max_step {
                    remove_queue.push(i);
                }
            }
            for i in remove_queue.into_iter().rev() {
                sim.var_vec.swap_remove(i);
                sim.data_vec.swap_remove(i);
                sim.step_vec.swap_remove(i);
            }
        }

        // right-turned vehicle
        let flow_len = self.var.rt_veh_flows.len();
        for i in 0..flow_len {
            let flow = &self.var.rt_veh_flows[i];

            let sim = &mut self.rt_veh_sim[i];

            sim.spawn_wait_secs += delta_secs;

            if sim.spawn_wait_secs > sim.spawn_next_secs {
                let var = self.var.gen_rt_veh_var(&mut rng, i);
                let Some(data) = RtVehData::sample(&mut rng, &var) else {
                    continue;
                };
                let step = (data.t_o / RtVehData::STEP) as usize;

                let transform = match flow.dir {
                    VehFlowDir::Nx2Ny(src, _) => {
                        let y = self.var.lane_along[src];
                        let p0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
                        let p1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();

                        let y = -(self.var.width_across * 0.5 - var.padding);
                        let q0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
                        let q1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();

                        let origin = math::intersection_point(p0, p1, q0, q1);
                        nalgebra::Isometry2::new(origin.into(), Default::default())
                    }
                    VehFlowDir::Ny2Px(src, _) => {
                        let y = self.var.lane_across[src];
                        let p0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
                        let p1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();

                        let y = self.var.width_along * 0.5 - var.padding;
                        let q0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
                        let q1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();

                        let origin = math::intersection_point(p0, p1, q0, q1);
                        nalgebra::Isometry2::new(origin.into(), r.angle())
                    }
                    VehFlowDir::Px2Py(src, _) => {
                        let y = self.var.lane_along[src];
                        let p0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
                        let p1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();

                        let y = self.var.width_across * 0.5 - var.padding;
                        let q0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
                        let q1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();

                        let origin = math::intersection_point(p0, p1, q0, q1);
                        nalgebra::Isometry2::new(origin.into(), std::f64::consts::PI)
                    }
                    VehFlowDir::Py2Nx(src, _) => {
                        let y = self.var.lane_across[src];
                        let p0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
                        let p1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();

                        let y = -(self.var.width_along * 0.5 - var.padding);
                        let q0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
                        let q1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();

                        let origin = math::intersection_point(p0, p1, q0, q1);
                        nalgebra::Isometry2::new(origin.into(), r.angle() + std::f64::consts::PI)
                    }
                    _ => unreachable!(),
                };

                sim.var_vec.push(var);
                sim.data_vec.push(data);
                sim.step_vec.push(step);
                sim.transform_vec.push(transform);

                let lambda = flow.density * flow.v_in_mean;
                let distr = rand_distr::Exp::new(lambda).unwrap();
                sim.spawn_next_secs = rand::Rng::sample(&mut rng, distr);
                sim.spawn_wait_secs = 0.0;
            }

            let data_len = sim.data_vec.len();

            for i in 0..data_len {
                sim.step_vec[i] += (delta_secs / LtVehData::STEP) as usize;
            }

            let mut remove_queue = vec![];
            for i in 0..data_len {
                if sim.step_vec[i] >= sim.data_vec[i].max_step {
                    remove_queue.push(i);
                }
            }
            for i in remove_queue.into_iter().rev() {
                sim.var_vec.swap_remove(i);
                sim.data_vec.swap_remove(i);
                sim.step_vec.swap_remove(i);
            }
        }

        // pedestrian
        let flow_len = self.var.ped_flows.len();
        for i in 0..flow_len {
            let flow = &self.var.ped_flows[i];

            let sim = &mut self.ped_sim[i];

            sim.spawn_wait_secs += delta_secs;

            if sim.spawn_wait_secs > sim.spawn_next_secs {
                let var = self.var.gen_ped_var(&mut rng, i);
                let Some(data) = PedData::sample(&mut rng, &var) else {
                    continue;
                };
                let step = 0;

                let transform = match flow.dir {
                    PedFlowDir::NxNy2PxNy => {
                        let y = self.var.width_across * 0.5;
                        let x_min = -self.var.cw_setback_across;
                        let origin: [f64; 2] = (r * nalgebra::Point2::new(x_min, y)).into();
                        nalgebra::Isometry2::new(origin.into(), r.angle() + std::f64::consts::PI)
                    }
                    PedFlowDir::PxNy2PxPy => {
                        let y = -self.var.width_along * 0.5;
                        let x_min = self.var.cw_setback_along;
                        let origin: [f64; 2] = nalgebra::Point2::new(x_min, y).into();
                        nalgebra::Isometry2::new(origin.into(), Default::default())
                    }
                    PedFlowDir::PxPy2NxPy => {
                        let y = -self.var.width_across * 0.5;
                        let x_min = self.var.cw_setback_across;
                        let origin: [f64; 2] = (r * nalgebra::Point2::new(x_min, y)).into();
                        nalgebra::Isometry2::new(origin.into(), r.angle())
                    }
                    PedFlowDir::NxPy2NxNy => {
                        let y = self.var.width_along * 0.5;
                        let x_min = -self.var.cw_setback_along;
                        let origin: [f64; 2] = nalgebra::Point2::new(x_min, y).into();
                        nalgebra::Isometry2::new(origin.into(), std::f64::consts::PI)
                    }

                    PedFlowDir::PxNy2NxNy => {
                        let y = -self.var.width_across * 0.5;
                        let x_min = -self.var.cw_setback_across - self.var.cw_width_across;
                        let origin: [f64; 2] = (r * nalgebra::Point2::new(x_min, y)).into();
                        nalgebra::Isometry2::new(origin.into(), r.angle())
                    }
                    PedFlowDir::PxPy2PxNy => {
                        let y = self.var.width_along * 0.5;
                        let x_min = self.var.cw_setback_along + self.var.cw_width_along;
                        let origin: [f64; 2] = nalgebra::Point2::new(x_min, y).into();
                        nalgebra::Isometry2::new(origin.into(), std::f64::consts::PI)
                    }
                    PedFlowDir::NxPy2PxPy => {
                        let y = self.var.width_across * 0.5;
                        let x_min = self.var.cw_setback_across + self.var.cw_width_across;
                        let origin: [f64; 2] = (r * nalgebra::Point2::new(x_min, y)).into();
                        nalgebra::Isometry2::new(origin.into(), r.angle() + std::f64::consts::PI)
                    }
                    PedFlowDir::NxNy2NxPy => {
                        let y = -self.var.width_along * 0.5;
                        let x_min = -self.var.cw_setback_along - self.var.cw_width_along;
                        let origin: [f64; 2] = nalgebra::Point2::new(x_min, y).into();
                        nalgebra::Isometry2::new(origin.into(), Default::default())
                    }
                };

                sim.var_vec.push(var);
                sim.data_vec.push(data);
                sim.step_vec.push(step);
                sim.transform_vec.push(transform);

                let lambda = flow.density * flow.v_in_mean;
                let distr = rand_distr::Exp::new(lambda).unwrap();
                sim.spawn_next_secs = rand::Rng::sample(&mut rng, distr);
                sim.spawn_wait_secs = 0.0;
            }

            let data_len = sim.data_vec.len();

            for i in 0..data_len {
                sim.step_vec[i] += (delta_secs / PedData::STEP) as usize;
            }

            let mut remove_queue = vec![];
            for i in 0..data_len {
                if sim.step_vec[i] >= sim.data_vec[i].max_step {
                    remove_queue.push(i);
                }
            }
            for i in remove_queue.into_iter().rev() {
                sim.var_vec.swap_remove(i);
                sim.data_vec.swap_remove(i);
                sim.step_vec.swap_remove(i);
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SimComponent {
    var: SimVar,
    runtime: Option<SimRuntime>,
}

impl SimComponent {
    pub fn forward(&mut self, delta_secs: f64) {
        if let Some(runtime) = &mut self.runtime {
            runtime.forward(delta_secs);
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        const ROAD_LENGTH: f64 = 30.0;
        const SUBDIVISION: usize = 64;

        let mut lines = vec![];
        let mut points = vec![];

        if let Some(runtime) = &self.runtime {
            let var = &runtime.var;

            let r = nalgebra::Rotation2::new(var.angle.to_radians());

            // along center line
            let p0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, 0.0).into();
            let p1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, 0.0).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GREEN);
            lines.push(line);
            // across center line
            let p0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, 0.0)).into();
            let p1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, 0.0)).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GREEN);
            lines.push(line);

            // along positive boundary
            let y = var.width_along * 0.5;
            let p0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
            let p1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
            lines.push(line);
            // along negative boundary
            let y = -var.width_along * 0.5;
            let p0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
            let p1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
            lines.push(line);
            // across positive boundary
            let y = var.width_across * 0.5;
            let p0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
            let p1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
            lines.push(line);
            // across negative boundary
            let y = -var.width_across * 0.5;
            let p0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
            let p1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
            lines.push(line);

            // along lane line
            for &y in &var.lane_along {
                let p0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
                let p1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();
                let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::RED);
                lines.push(line);
            }
            // across lane line
            for &y in &var.lane_across {
                let p0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
                let p1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
                let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::RED);
                lines.push(line);
            }

            // along negative hard nose
            let p0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, 0.0).into();
            let p1: [f64; 2] = nalgebra::Point2::new(-var.hn_along, 0.0).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
            lines.push(line);
            // along positive hard nose
            let p0: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, 0.0).into();
            let p1: [f64; 2] = nalgebra::Point2::new(var.hn_along, 0.0).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
            lines.push(line);
            // across negative line
            let p0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, 0.0)).into();
            let p1: [f64; 2] = (r * nalgebra::Point2::new(-var.hn_across, 0.0)).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
            lines.push(line);
            // across positive line
            let p0: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, 0.0)).into();
            let p1: [f64; 2] = (r * nalgebra::Point2::new(var.hn_across, 0.0)).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
            lines.push(line);

            // along positive crosswalk
            let y = var.width_along * 0.5;
            let x_min = var.cw_setback_along;
            let x_max = var.cw_setback_along + var.cw_width_along;
            let p0: [f64; 2] = nalgebra::Point2::new(x_min, -y).into();
            let p1: [f64; 2] = nalgebra::Point2::new(x_min, y).into();
            let p2: [f64; 2] = nalgebra::Point2::new(x_max, y).into();
            let p3: [f64; 2] = nalgebra::Point2::new(x_max, -y).into();
            let line = egui_plot::Line::new(vec![p0, p1, p2, p3, p0]).color(egui::Color32::YELLOW);
            lines.push(line);
            // along negative crosswalk
            let y = var.width_along * 0.5;
            let x_min = -var.cw_setback_along;
            let x_max = -var.cw_setback_along - var.cw_width_along;
            let p0: [f64; 2] = nalgebra::Point2::new(x_min, -y).into();
            let p1: [f64; 2] = nalgebra::Point2::new(x_min, y).into();
            let p2: [f64; 2] = nalgebra::Point2::new(x_max, y).into();
            let p3: [f64; 2] = nalgebra::Point2::new(x_max, -y).into();
            let line = egui_plot::Line::new(vec![p0, p1, p2, p3, p0]).color(egui::Color32::YELLOW);
            lines.push(line);
            // across positive crosswalk
            let y = var.width_across * 0.5;
            let x_min = var.cw_setback_across;
            let x_max = var.cw_setback_across + var.cw_width_across;
            let p0: [f64; 2] = (r * nalgebra::Point2::new(x_min, -y)).into();
            let p1: [f64; 2] = (r * nalgebra::Point2::new(x_min, y)).into();
            let p2: [f64; 2] = (r * nalgebra::Point2::new(x_max, y)).into();
            let p3: [f64; 2] = (r * nalgebra::Point2::new(x_max, -y)).into();
            let line = egui_plot::Line::new(vec![p0, p1, p2, p3, p0]).color(egui::Color32::YELLOW);
            lines.push(line);
            // across negative crosswalk
            let y = var.width_across * 0.5;
            let x_min = -var.cw_setback_across;
            let x_max = -var.cw_setback_across - var.cw_width_across;
            let p0: [f64; 2] = (r * nalgebra::Point2::new(x_min, -y)).into();
            let p1: [f64; 2] = (r * nalgebra::Point2::new(x_min, y)).into();
            let p2: [f64; 2] = (r * nalgebra::Point2::new(x_max, y)).into();
            let p3: [f64; 2] = (r * nalgebra::Point2::new(x_max, -y)).into();
            let line = egui_plot::Line::new(vec![p0, p1, p2, p3, p0]).color(egui::Color32::YELLOW);
            lines.push(line);

            // along positive stop-line
            let y = var.width_along * 0.5;
            let p0: [f64; 2] = nalgebra::Point2::new(var.sl_setback_along, -y).into();
            let p1: [f64; 2] = nalgebra::Point2::new(var.sl_setback_along, y).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::YELLOW);
            lines.push(line);
            // along negative stop-line
            let y = var.width_along * 0.5;
            let p0: [f64; 2] = nalgebra::Point2::new(-var.sl_setback_along, -y).into();
            let p1: [f64; 2] = nalgebra::Point2::new(-var.sl_setback_along, y).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::YELLOW);
            lines.push(line);
            // across positive stop-line
            let y = var.width_across * 0.5;
            let p0: [f64; 2] = (r * nalgebra::Point2::new(var.sl_setback_across, -y)).into();
            let p1: [f64; 2] = (r * nalgebra::Point2::new(var.sl_setback_across, y)).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::YELLOW);
            lines.push(line);
            // across negative stop-line
            let y = var.width_across * 0.5;
            let p0: [f64; 2] = (r * nalgebra::Point2::new(-var.sl_setback_across, -y)).into();
            let p1: [f64; 2] = (r * nalgebra::Point2::new(-var.sl_setback_across, y)).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::YELLOW);
            lines.push(line);

            // radius border
            fn radius_border(
                p0: [f64; 2],
                p1: [f64; 2],
                q0: [f64; 2],
                q1: [f64; 2],
                radius: f64,
            ) -> Vec<[f64; 2]> {
                let o = math::intersection_point(p0, p1, q0, q1);
                let mut points = vec![];
                for i in 0..=SUBDIVISION {
                    let v = i as f64 / SUBDIVISION as f64 * 2.0 * std::f64::consts::PI;
                    let x = o[0] + radius * v.cos();
                    let y = o[1] + radius * v.sin();
                    if -ROAD_LENGTH <= x
                        && x <= ROAD_LENGTH
                        && -ROAD_LENGTH <= y
                        && y <= ROAD_LENGTH
                    {
                        points.push([x, y]);
                    }
                }
                points
            }
            // radius border NxNy
            let y = -(var.width_along * 0.5 + var.radius);
            let p0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
            let p1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();
            let y = -(var.width_across * 0.5 + var.radius);
            let q0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
            let q1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
            let point = egui_plot::Points::new(radius_border(p0, p1, q0, q1, var.radius))
                .color(egui::Color32::GRAY);
            points.push(point);
            // radius border PxNy
            let y = var.width_along * 0.5 + var.radius;
            let p0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
            let p1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();
            let y = -(var.width_across * 0.5 + var.radius);
            let q0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
            let q1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
            let point = egui_plot::Points::new(radius_border(p0, p1, q0, q1, var.radius))
                .color(egui::Color32::GRAY);
            points.push(point);
            // radius border PxPy
            let y = var.width_along * 0.5 + var.radius;
            let p0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
            let p1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();
            let y = var.width_across * 0.5 + var.radius;
            let q0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
            let q1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
            let point = egui_plot::Points::new(radius_border(p0, p1, q0, q1, var.radius))
                .color(egui::Color32::GRAY);
            points.push(point);
            // radius border NxPy
            let y = -(var.width_along * 0.5 + var.radius);
            let p0: [f64; 2] = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
            let p1: [f64; 2] = nalgebra::Point2::new(ROAD_LENGTH, y).into();
            let y = var.width_across * 0.5 + var.radius;
            let q0: [f64; 2] = (r * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
            let q1: [f64; 2] = (r * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
            let point = egui_plot::Points::new(radius_border(p0, p1, q0, q1, var.radius))
                .color(egui::Color32::GRAY);
            points.push(point);

            // left-turned vehicle
            let flow_len = runtime.lt_veh_sim.len();
            for i in 0..flow_len {
                let flow = &runtime.lt_veh_sim[i];
                let data_len = flow.data_vec.len();

                for i in 0..data_len {
                    let data = &flow.data_vec[i];
                    let step = &flow.step_vec[i];
                    let tx = &flow.transform_vec[i];

                    let point = data.trajectory_series[*step];
                    let point: [f64; 2] = (tx * nalgebra::Point2::from(point)).into();
                    let point = egui_plot::Points::new(point).color(egui::Color32::WHITE);
                    points.push(point);
                }
            }

            // right-turned vehicle
            let flow_len = runtime.rt_veh_sim.len();
            for i in 0..flow_len {
                let flow = &runtime.rt_veh_sim[i];
                let data_len = flow.data_vec.len();

                for i in 0..data_len {
                    let data = &flow.data_vec[i];
                    let step = &flow.step_vec[i];
                    let tx = &flow.transform_vec[i];

                    let point = data.trajectory_series[*step];
                    let point: [f64; 2] = (tx * nalgebra::Point2::from(point)).into();
                    let point = egui_plot::Points::new(point).color(egui::Color32::WHITE);
                    points.push(point);
                }
            }

            // pedestrian
            let flow_len = runtime.ped_sim.len();
            for i in 0..flow_len {
                let flow = &runtime.ped_sim[i];
                let data_len = flow.data_vec.len();

                for i in 0..data_len {
                    let data = &flow.data_vec[i];
                    let step = &flow.step_vec[i];
                    let tx = &flow.transform_vec[i];

                    let point = data.trajectory_series[*step];
                    let point: [f64; 2] = (tx * nalgebra::Point2::from(point)).into();
                    let point = egui_plot::Points::new(point).color(egui::Color32::WHITE);
                    points.push(point);
                }
            }
        }

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                let widget = egui::Slider::new(&mut self.var.angle, 0.0..=180.0)
                    .text("Intersection angle[deg]");
                ui.add(widget);

                let widget =
                    egui::Slider::new(&mut self.var.radius, 0.0..=30.0).text("Border radius[m]");
                ui.add(widget);

                let widget = egui::Slider::new(&mut self.var.width_along, 0.0..=30.0)
                    .text("Along road width[m]");
                ui.add(widget);

                let widget = egui::Slider::new(&mut self.var.width_across, 0.0..=30.0)
                    .text("Across road width[m]");
                ui.add(widget);

                // along lane
                ui.horizontal(|ui| {
                    ui.label(format!("Along lane: {}", self.var.lane_along.len()));
                    if ui.button("Add along lane").clicked() {
                        self.var.lane_along.push(Default::default());
                    }
                    if ui.button("Remove along lane").clicked() {
                        self.var.lane_along.pop();
                    }
                });
                for lane in &mut self.var.lane_along {
                    ui.horizontal(|ui| {
                        let widget = egui::Slider::new(lane, -30.0..=30.0).text("Lane shift[m]");
                        ui.add(widget);
                    });
                }

                // across lane
                ui.horizontal(|ui| {
                    ui.label(format!("Across lane: {}", self.var.lane_across.len()));
                    if ui.button("Add along lane").clicked() {
                        self.var.lane_across.push(Default::default());
                    }
                    if ui.button("Remove along lane").clicked() {
                        self.var.lane_across.pop();
                    }
                });
                for lane in &mut self.var.lane_across {
                    ui.horizontal(|ui| {
                        let widget = egui::Slider::new(lane, -30.0..=30.0).text("Lane shift[m]");
                        ui.add(widget);
                    });
                }

                let widget = egui::Slider::new(&mut self.var.hn_along, 0.0..=30.0)
                    .text("Along hard nose[m]");
                ui.add(widget);

                let widget = egui::Slider::new(&mut self.var.hn_across, 0.0..=30.0)
                    .text("Across hard nose[m]");
                ui.add(widget);

                let widget = egui::Slider::new(&mut self.var.cw_setback_along, 0.0..=30.0)
                    .text("Along crosswalk setback[m]");
                ui.add(widget);

                let widget = egui::Slider::new(&mut self.var.cw_setback_across, 0.0..=30.0)
                    .text("Across crosswalk setback[m]");
                ui.add(widget);

                let widget = egui::Slider::new(&mut self.var.cw_width_along, 0.0..=30.0)
                    .text("Along crosswalk width[m]");
                ui.add(widget);

                let widget = egui::Slider::new(&mut self.var.cw_width_across, 0.0..=30.0)
                    .text("Across crosswalk width[m]");
                ui.add(widget);

                let widget = egui::Slider::new(&mut self.var.sl_setback_along, 0.0..=30.0)
                    .text("Along stop-line setback[m]");
                ui.add(widget);

                let widget = egui::Slider::new(&mut self.var.sl_setback_across, 0.0..=30.0)
                    .text("Across stop-line setback[m]");
                ui.add(widget);

                if ui.button("Simulate").clicked() {
                    self.runtime = Some(SimRuntime::new(self.var.clone()));
                }
            });

            egui_plot::Plot::new("Sim Plot")
                .view_aspect(1.0)
                .data_aspect(1.0)
                .allow_scroll(false)
                .show(ui, |plot_ui| {
                    lines.into_iter().for_each(|v| plot_ui.line(v));
                    points.into_iter().for_each(|v| plot_ui.points(v));
                });
        })
        .response
    }
}
