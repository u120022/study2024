use eframe::egui;

#[derive(Debug, Clone, Default)]
pub struct RightTurnedComponent {
    var: RightTurnedVar,
    data: Option<RightTurnedData>,
}

impl RightTurnedComponent {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.scope(|ui| {
            ui.heading("Variable");

            let widget =
                egui::Slider::new(&mut self.var.v_in, 0.0..=100.0).text("inflow velocity[m/sec]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.var.v_out, 0.0..=100.0).text("outflow velocity[m/sec]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.var.angle, 0.0..=180.0).text("intersection angle[deg]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.var.hn_in, 0.0..=10.0).text("inflow head nose[m]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.var.hn_out, 0.0..=10.0).text("outflow head nose[m]");
            ui.add(widget);

            if ui.button("Compute").clicked() {
                let distr = RightTurnedDistr::new(&self.var);
                self.data = RightTurnedData::sample(&mut rand::thread_rng(), &self.var, &distr);
            }

            // plot
            if let Some(data) = &self.data {
                ui.heading("Result");

                ui.label(format!("c_in: {:.4}", data.c_in));
                ui.label(format!("c_out: {:.4}", data.c_out));
                ui.label(format!("v_min: {:.4}", data.v_min));
                ui.label(format!("x_min: {:.4}", data.x_min));
                ui.label(format!("t_min: {:.4}", data.t_min));
                ui.label(format!("t_exit: {:.4}", data.t_exit));
                ui.label(format!("t_o: {:.4}", data.t_o));
                ui.label(format!("x_o: {:.4}", data.x_o));
                ui.label(format!("max_step: {}", data.max_step));

                ui.separator();

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
struct RightTurnedVar {
    v_in: f64,
    v_out: f64,
    angle: f64,
    hn_in: f64,
    hn_out: f64,
}

impl Default for RightTurnedVar {
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
struct RightTurnedDistr {
    c_in: rand_distr::Gamma<f64>,
    c_out: rand_distr::Gamma<f64>,
    v_min: rand_distr::Normal<f64>,
    x_min: rand_distr::Normal<f64>,
}

impl RightTurnedDistr {
    fn new(var: &RightTurnedVar) -> Self {
        // c_in parameter
        let a = nalgebra::vector![0.320, -0.0150];
        let x = nalgebra::vector![var.v_in, var.angle];
        let shape = a.dot(&x).max(f64::EPSILON);
        let b = nalgebra::vector![0.000334, 0.0];
        let y = nalgebra::vector![var.v_in, var.hn_out];
        let scale = b.dot(&y).max(f64::EPSILON);
        let c_in = rand_distr::Gamma::new(shape, scale).unwrap();

        // c_out parameter
        let a = nalgebra::vector![0.0275, 0.0108];
        let x = nalgebra::vector![var.v_in, var.angle];
        let shape = a.dot(&x).max(f64::EPSILON);
        let b = nalgebra::vector![0.000228, 0.00222];
        let y = nalgebra::vector![var.v_in, var.hn_out];
        let scale = b.dot(&y).max(f64::EPSILON);
        let c_out = rand_distr::Gamma::new(shape, scale).unwrap();

        // v_min parameter
        let a = nalgebra::vector![0.488, 0.0236, 0.0325];
        let x = nalgebra::vector![var.v_in, var.angle, var.hn_in];
        let mean = a.dot(&x);
        let b = nalgebra::vector![0.0261, 0.00689, 0.0];
        let y = nalgebra::vector![var.v_in, var.angle, var.hn_in];
        let std_dev = b.dot(&y).max(f64::EPSILON);
        let v_min = rand_distr::Normal::new(mean, std_dev).unwrap();

        // x_min parameter
        let a = nalgebra::vector![0.917, 0.150, 0.218];
        let x = nalgebra::vector![var.v_in, var.angle, var.hn_in];
        let mean = a.dot(&x);
        let b = nalgebra::vector![-0.438, 0.0975, 0.101];
        let y = nalgebra::vector![var.v_in, var.angle, var.hn_in];
        let std_dev = b.dot(&y).max(f64::EPSILON);
        let x_min = rand_distr::Normal::new(mean, std_dev).unwrap();

        Self {
            c_in,
            c_out,
            v_min,
            x_min,
        }
    }
}

#[derive(Debug, Clone)]
struct RightTurnedData {
    c_in: f64,
    c_out: f64,
    v_min: f64,
    x_min: f64,
    t_min: f64,
    t_exit: f64,
    t_o: f64,
    x_o: f64,
    max_step: usize,
    velocity_series: Vec<[f64; 2]>,
    position_series: Vec<[f64; 2]>,
    curvature_series: Vec<[f64; 2]>,
    trajectory_series: Vec<[f64; 2]>,
}

impl RightTurnedData {
    const STEP: f64 = 0.001;
    const MAX_TIME: f64 = 100.0;

    fn sample(
        rng: &mut impl rand::Rng,
        var: &RightTurnedVar,
        distr: &RightTurnedDistr,
    ) -> Option<Self> {
        let c_in = rand::Rng::sample(rng, distr.c_in);
        let c_out = rand::Rng::sample(rng, distr.c_out);
        let v_min = rand::Rng::sample(rng, distr.v_min);
        let x_min = rand::Rng::sample(rng, distr.x_min);

        let mut velocity_series = vec![];

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
