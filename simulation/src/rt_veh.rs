use eframe::egui;

use crate::math;

#[derive(Debug, Clone, Default)]
pub struct RtVehComponent {
    var: RtVehVar,
    data: Option<RtVehData>,
}

impl RtVehComponent {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.scope(|ui| {
            ui.heading("Right-Turned Vehicle");

            let widget =
                egui::Slider::new(&mut self.var.v_in, 0.0..=100.0).text("Inflow velocity[m/sec]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.var.v_out, 0.0..=100.0).text("Outflow velocity[m/sec]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.var.angle, 0.0..=180.0).text("Intersection angle[deg]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.var.hn_in, 0.0..=10.0)
                .text("Inflow hard nose length[m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.var.hn_out, 0.0..=10.0)
                .text("Outflow hard nose length[m]");
            ui.add(widget);

            if ui.button("Compute").clicked() {
                self.data = RtVehData::sample(&mut rand::thread_rng(), &self.var);
            }

            // plot
            if let Some(data) = &self.data {
                let mut text = String::new();
                text.push_str(&format!("c_in: {:.4}\n", data.c_in));
                text.push_str(&format!("c_out: {:.4}\n", data.c_out));
                text.push_str(&format!("v_min: {:.4}\n", data.v_min));
                text.push_str(&format!("x_min: {:.4}\n", data.x_min));
                text.push_str(&format!("t_min: {:.4}\n", data.t_min));
                text.push_str(&format!("t_exit: {:.4}\n", data.t_exit));
                text.push_str(&format!("t_o: {:.4}\n", data.t_o));
                text.push_str(&format!("x_o: {:.4}\n", data.x_o));
                text.push_str(&format!("max_step: {}\n", data.max_step));
                ui.label(egui::RichText::new(text).monospace());

                ui.label("Velocity-Time Plot");
                egui_plot::Plot::new("Velocity-Time Plot")
                    .view_aspect(2.0)
                    .allow_scroll(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(egui_plot::Line::new(data.velocity_series.clone()));
                        plot_ui.vline(egui_plot::VLine::new(data.t_o));
                    });

                ui.label("Position-Time Plot");
                egui_plot::Plot::new("Position-Time Plot")
                    .view_aspect(2.0)
                    .allow_scroll(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(egui_plot::Line::new(data.position_series.clone()));
                        plot_ui.vline(egui_plot::VLine::new(data.t_o));
                        plot_ui.hline(egui_plot::HLine::new(data.x_o));
                    });

                ui.label("Curvature-Position Plot");
                egui_plot::Plot::new("Curvature-Position Plot")
                    .view_aspect(2.0)
                    .allow_scroll(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(egui_plot::Line::new(data.curvature_series.clone()));
                        plot_ui.vline(egui_plot::VLine::new(data.x_o));
                    });

                ui.label("Trajectory XY Plot");
                egui_plot::Plot::new("Trajectory XY Plot")
                    .view_aspect(2.0)
                    .allow_scroll(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(egui_plot::Line::new(data.trajectory_series.clone()));
                    });
            }
        })
        .response
    }
}

#[derive(Debug, Clone)]
pub struct RtVehVar {
    pub v_in: f64,
    pub v_out: f64,
    pub angle: f64,
    pub hn_in: f64,
    pub hn_out: f64,
}

impl Default for RtVehVar {
    fn default() -> Self {
        Self {
            v_in: 20.0,
            v_out: 20.0,
            angle: 90.0,
            hn_in: 5.0,
            hn_out: 5.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RtVehData {
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

impl RtVehData {
    pub const STEP: f64 = 0.001;
    pub const MAX_TIME: f64 = 100.0;

    pub fn sample(rng: &mut impl rand::Rng, var: &RtVehVar) -> Option<Self> {
        let mut velocity_series = vec![];

        // c_in parameter
        let a = nalgebra::vector![0.320, -0.0150];
        let x = nalgebra::vector![var.v_in, var.angle];
        let shape = a.dot(&x).max(f64::EPSILON);
        let b = nalgebra::vector![0.000334, 0.0];
        let y = nalgebra::vector![var.v_in, var.hn_out];
        let scale = b.dot(&y).max(f64::EPSILON);
        let c_in = rand_distr::Gamma::new(shape, scale).unwrap();
        let c_in = rand::Rng::sample(rng, c_in);

        // c_out parameter
        let a = nalgebra::vector![0.0275, 0.0108];
        let x = nalgebra::vector![var.v_in, var.angle];
        let shape = a.dot(&x).max(f64::EPSILON);
        let b = nalgebra::vector![0.000228, 0.00222];
        let y = nalgebra::vector![var.v_in, var.hn_out];
        let scale = b.dot(&y).max(f64::EPSILON);
        let c_out = rand_distr::Gamma::new(shape, scale).unwrap();
        let c_out = rand::Rng::sample(rng, c_out);

        // v_min parameter
        let a = nalgebra::vector![0.488, 0.0236, 0.0325];
        let x = nalgebra::vector![var.v_in, var.angle, var.hn_in];
        let mean = a.dot(&x);
        let b = nalgebra::vector![0.0261, 0.00689, 0.0];
        let y = nalgebra::vector![var.v_in, var.angle, var.hn_in];
        let std_dev = b.dot(&y).max(f64::EPSILON);
        let v_min = rand_distr::Normal::new(mean, std_dev).unwrap();
        let v_min = rand::Rng::sample(rng, v_min);

        // inflow
        let t_min = (2.0 / c_in * (var.v_in - v_min)).cbrt();
        let a = c_in;
        let b = -3.0 / 2.0 * a * t_min;
        let c = 0.0;
        let d = var.v_in;
        let mut t = 0.0;
        while t <= t_min {
            if t > Self::MAX_TIME {
                break;
            }
            let point = [t, a * t.powi(3) + b * t.powi(2) + c * t + d];
            velocity_series.push(point);
            t += Self::STEP;
        }

        // outflow
        let t_next = (2.0 / -c_out * (v_min - var.v_out)).cbrt();
        let a = -c_out;
        let b = -3.0 / 2.0 * a * t_next;
        let c = 0.0;
        let d = v_min;
        let mut t = 0.0;
        while t <= t_next {
            if t > Self::MAX_TIME {
                break;
            }
            let point = [t + t_min, a * t.powi(3) + b * t.powi(2) + c * t + d];
            velocity_series.push(point);
            t += Self::STEP;
        }

        let max_step = velocity_series.len();

        // x_min parameter
        let a = nalgebra::vector![0.917, 0.150, 0.218];
        let x = nalgebra::vector![var.v_in, var.angle, var.hn_in];
        let mean = a.dot(&x);
        let b = nalgebra::vector![-0.438, 0.0975, 0.101];
        let y = nalgebra::vector![var.v_in, var.angle, var.hn_in];
        let std_dev = b.dot(&y).max(f64::EPSILON);
        let x_min = rand_distr::Normal::new(mean, std_dev).unwrap();
        let x_min = rand::Rng::sample(rng, x_min);

        // position
        let mut x_min_ = 0.0;
        let mut position_series = vec![Default::default(); max_step];
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
        let a = nalgebra::vector![0.0282, 0.0807];
        let x = nalgebra::vector![var.angle, f64::min(var.hn_in, var.hn_out)];
        let shape = a.dot(&x);
        let b = nalgebra::vector![0.162, 1.43];
        let y = nalgebra::vector![var.angle, v_min];
        let scale = b.dot(&y).max(f64::EPSILON);
        let r_min = rand_distr::Weibull::new(scale, shape).unwrap();
        let r_min = rand::Rng::sample(rng, r_min);

        // curvature
        let a = nalgebra::vector![6.09, 0.985, 0.186, 0.235, 0.0];
        let x = nalgebra::vector![1.0, v_min, r_min, var.hn_in, var.hn_out];
        let a1 = a.dot(&x);
        let a = nalgebra::vector![6.81, 0.611, 0.313, 0.0, 0.188];
        let x = nalgebra::vector![1.0, v_min, r_min, var.hn_in, var.hn_out];
        let a2 = a.dot(&x);
        let l_clothoid1 = r_min.recip() / a1.powi(2).recip();
        let angle_clothoid1 = 0.5 * a1.powi(2).recip() * l_clothoid1.powi(2);
        let l_clothoid2 = r_min.recip() / a2.powi(2).recip();
        let angle_clothoid2 = 0.5 * a2.powi(2).recip() * l_clothoid2.powi(2);
        let angle_arc = var.angle.to_radians() - (angle_clothoid1 + angle_clothoid2);
        let l_arc = angle_arc / r_min.recip();
        if l_arc < 0.0 {
            return None;
        }
        let x_0 = x_o;
        let x_1 = x_0 + l_clothoid1;
        let x_2 = x_1 + l_arc;
        let x_3 = x_2 + l_clothoid2;
        let mut curvature_series = vec![Default::default(); max_step];
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
        let mut position = [0.0, 0.0];
        let mut trajectory_series = vec![Default::default(); max_step];
        for i in 0..max_step {
            let [_, v] = velocity_series[i];
            let [_, c] = curvature_series[i];

            let dx = v * Self::STEP;
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

        // // origin shift
        // for [t, _] in velocity_series.iter_mut() {
        //     *t -= t_o;
        // }
        // for [t, x] in position_series.iter_mut() {
        //     *t -= t_o;
        //     *x -= x_o;
        // }
        // for [x, _] in curvature_series.iter_mut() {
        //     *x -= x_o;
        // }

        // trajectory origin shift
        let start_origin = nalgebra::Vector2::new(trajectory_series[0][0], trajectory_series[0][1]);
        let start_dir = nalgebra::Vector2::new(
            trajectory_series[1][0] - trajectory_series[0][0],
            trajectory_series[1][1] - trajectory_series[0][1],
        )
        .normalize();
        let last_idx = trajectory_series.len() - 1;
        let end_origin = nalgebra::Vector2::new(
            trajectory_series[last_idx][0],
            trajectory_series[last_idx][1],
        );
        let end_dir = nalgebra::Vector2::new(
            trajectory_series[last_idx - 1][0] - trajectory_series[last_idx][0],
            trajectory_series[last_idx - 1][1] - trajectory_series[last_idx][1],
        )
        .normalize();
        let trajectory_origin = math::intersection_point(
            [start_origin.x, start_origin.y],
            [start_origin.x + start_dir.x, start_origin.y + start_dir.y],
            [end_origin.x, end_origin.y],
            [end_origin.x + end_dir.x, end_origin.y + end_dir.y],
        );
        for [x, y] in trajectory_series.iter_mut() {
            *x -= trajectory_origin[0];
            *y -= trajectory_origin[1];
        }

        Some(Self {
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
}
