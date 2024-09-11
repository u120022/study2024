use eframe::egui;

#[derive(Debug, Clone, Default)]
pub struct LeftTurnedComponent {
    left_turned_var: LeftTurnedVar,
    left_turned_data: Option<LeftTurnedData>,
}

impl LeftTurnedComponent {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.scope(|ui| {
            ui.heading("Variable");

            let widget = egui::Slider::new(&mut self.left_turned_var.v_in, 0.0..=100.0)
                .text("inflow velocity[m/sec]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.left_turned_var.v_out, 0.0..=100.0)
                .text("outflow velocity[m/sec]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.left_turned_var.angle, 0.0..=180.0)
                .text("intersection angle[deg]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.left_turned_var.radius, 0.0..=30.0)
                .text("border radius[m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.left_turned_var.padding, 0.0..=10.0)
                .text("distance from border[m]");
            ui.add(widget);

            let widget = egui::Checkbox::new(&mut self.left_turned_var.large, "large car type");
            ui.add(widget);

            if ui.button("Compute").clicked() {
                let left_turned_distr = LeftTurnedDistr::new(&self.left_turned_var);
                let left_turned_data = LeftTurnedData::sample(
                    &mut rand::thread_rng(),
                    &self.left_turned_var,
                    &left_turned_distr,
                );
                self.left_turned_data = Some(left_turned_data);
            }

            // plot
            if let Some(data) = &self.left_turned_data {
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
struct LeftTurnedVar {
    v_in: f64,
    v_out: f64,
    angle: f64,
    radius: f64,
    padding: f64,
    large: bool,
}

impl Default for LeftTurnedVar {
    fn default() -> Self {
        Self {
            v_in: 20.0,
            v_out: 20.0,
            angle: 90.0,
            radius: 17.0,
            padding: 1.0,
            large: false,
        }
    }
}

#[derive(Debug, Clone)]
struct LeftTurnedDistr {
    c_in: rand_distr::Gamma<f64>,
    c_out: rand_distr::Gamma<f64>,
    v_min: rand_distr::Normal<f64>,
    x_min: rand_distr::Normal<f64>,
    r_min: rand_distr::Normal<f64>,
}

impl LeftTurnedDistr {
    fn new(var: &LeftTurnedVar) -> Self {
        // c_in parameter
        let a = nalgebra::vector![2.09, 0.256, -0.0155, 0.0, -0.168, 0.0];
        let x = nalgebra::vector![1.0, var.v_in, var.angle, var.radius, var.padding, var.v_out];
        let shape = a.dot(&x).max(f64::EPSILON);
        let b = nalgebra::vector![0.0573, -0.00173, -0.00109, 0.00219, 0.0];
        let y = nalgebra::vector![1.0, var.v_in, var.radius, var.padding, var.v_out];
        let scale = b.dot(&y).max(f64::EPSILON);
        let c_in = rand_distr::Gamma::new(shape, scale).unwrap();

        // c_out parameter
        let a = nalgebra::vector![1.40, 0.0, 0.0, 0.0, 0.0633, -0.0224];
        let x = nalgebra::vector![1.0, var.v_in, var.angle, var.radius, var.padding, var.v_out];
        let shape = a.dot(&x).max(f64::EPSILON);
        let b = nalgebra::vector![0.0772, 0.0, 0.0, 0.0, -0.00355];
        let y = nalgebra::vector![1.0, var.v_in, var.radius, var.padding, var.v_out];
        let scale = b.dot(&y).max(f64::EPSILON);
        let c_out = rand_distr::Gamma::new(shape, scale).unwrap();

        // v_min parameter
        let a = nalgebra::vector![-0.301, 0.0908, 0.0607, 0.0387, 0.233, -0.496];
        let x = nalgebra::vector![
            1.0,
            var.v_in,
            var.radius,
            var.angle,
            var.padding,
            if var.large { 1.0 } else { 0.0 }
        ];
        let mean = a.dot(&x);
        let b = nalgebra::vector![0.665, 0.0, 0.0419];
        let y = nalgebra::vector![1.0, var.radius, var.padding];
        let std_dev = b.dot(&y).max(f64::EPSILON);
        let v_min = rand_distr::Normal::new(mean, std_dev).unwrap();

        // x_min parameter
        let a = nalgebra::vector![1.42, 0.0, 0.586, 0.0896, 0.577, 0.0];
        let x = nalgebra::vector![
            1.0,
            var.v_in,
            var.radius,
            var.angle,
            var.padding,
            if var.large { 1.0 } else { 0.0 }
        ];
        let mean = a.dot(&x);
        let b = nalgebra::vector![0.135, 0.144, 0.336];
        let y = nalgebra::vector![1.0, var.radius, var.padding];
        let std_dev = b.dot(&y).max(f64::EPSILON);
        let x_min = rand_distr::Normal::new(mean, std_dev).unwrap();

        // r_min parameter
        let a = nalgebra::vector![0.127, 0.390, 0.862, -6.46];
        let x = nalgebra::vector![var.angle, var.radius, var.padding, 1.0,];
        let mean = a.dot(&x);
        let b = nalgebra::vector![0.0363, 0.0624, 0.118, -2.86];
        let y = nalgebra::vector![var.angle, var.radius, var.padding, 1.0,];
        let std_dev = b.dot(&y).max(f64::EPSILON);
        let r_min = rand_distr::Normal::new(mean, std_dev).unwrap();

        Self {
            c_in,
            c_out,
            v_min,
            x_min,
            r_min,
        }
    }
}

#[derive(Debug, Clone)]
struct LeftTurnedData {
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

impl LeftTurnedData {
    const STEP: f64 = 0.001;
    const MAX_TIME: f64 = 100.0;

    fn sample(rng: &mut impl rand::Rng, var: &LeftTurnedVar, distr: &LeftTurnedDistr) -> Self {
        let c_in = rand::Rng::sample(rng, distr.c_in);
        let c_out = rand::Rng::sample(rng, distr.c_out);
        let v_min = rand::Rng::sample(rng, distr.v_min);
        let x_min = rand::Rng::sample(rng, distr.x_min);
        let r_min = rand::Rng::sample(rng, distr.r_min);

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

        // curvature
        let a = nalgebra::vector![-1.65, 0.0404, 0.334, 0.0, 0.461, 0.369];
        let x = nalgebra::vector![
            1.0,
            var.angle,
            var.radius,
            if var.large { 1.0 } else { 0.0 },
            var.padding,
            v_min
        ];
        let a1 = a.dot(&x);
        let a = nalgebra::vector![2.33, 0.0, 0.335, 2.05, 1.04, 0.268];
        let x = nalgebra::vector![
            1.0,
            var.angle,
            var.radius,
            if var.large { 1.0 } else { 0.0 },
            var.padding,
            v_min
        ];
        let a2 = a.dot(&x);
        let l_clothoid1 = r_min.recip() / a1.powi(2).recip();
        let angle_clothoid1 = 0.5 * a1.powi(2).recip() * l_clothoid1.powi(2);
        let l_clothoid2 = r_min.recip() / a2.powi(2).recip();
        let angle_clothoid2 = 0.5 * a2.powi(2).recip() * l_clothoid2.powi(2);
        let angle_arc = var.angle.to_radians() - (angle_clothoid1 + angle_clothoid2);
        let l_arc = angle_arc / r_min.recip();
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

        Self {
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
        }
    }
}
