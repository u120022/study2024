use eframe::egui;

fn main() -> eframe::Result {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    env_logger::init();

    eframe::run_native(
        "Simulation",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::<App>::default())),
    )
}

struct App {
    left_turned_var: LeftTurnedVar,
    left_turned_data: Option<LeftTurnedData>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            left_turned_var: Default::default(),
            left_turned_data: Default::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sumulation");

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
                ui.label(format!(
                    "[Param] c_in: {:.2}, c_out: {:.2}, v_min: {:.2}, x_min: {:.2}, t_min: {:.2}, t_exit: {:.2}",
                    data.c_in, data.c_out, data.v_min, data.x_min, data.t_min, data.t_exit
                ));

                let velocity_line = egui_plot::Line::new(data.velocity_series.clone());
                egui_plot::Plot::new("Velocity-Time Plot")
                    .view_aspect(2.0)
                    .show(ui, |plot_ui| plot_ui.line(velocity_line));

                let position_line = egui_plot::Line::new(data.position_series.clone());
                egui_plot::Plot::new("Position-Time Plot")
                    .view_aspect(2.0)
                    .show(ui, |plot_ui| plot_ui.line(position_line));
            }
        });
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

        Self {
            c_in,
            c_out,
            v_min,
            x_min,
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
    velocity_series: Vec<[f64; 2]>,
    position_series: Vec<[f64; 2]>,
}

impl LeftTurnedData {
    fn sample(rng: &mut impl rand::Rng, var: &LeftTurnedVar, distr: &LeftTurnedDistr) -> Self {
        const STEP: f64 = 0.01;
        const MAX_TIME: f64 = 100.0;

        let c_in = rand::Rng::sample(rng, distr.c_in);
        let c_out = rand::Rng::sample(rng, distr.c_out);
        let v_min = rand::Rng::sample(rng, distr.v_min);

        let mut velocity_series = vec![];

        // inflow
        let t_min = (2.0 / c_in * (var.v_in - v_min)).cbrt();
        let a = c_in;
        let b = -3.0 / 2.0 * a * t_min;
        let c = 0.0;
        let d = var.v_in;
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
        let t_next = (2.0 / -c_out * (v_min - var.v_out)).cbrt();
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

        // position
        let mut x = 0.0;
        let mut x_min_ = 0.0;
        let mut position_series = vec![];
        for w in velocity_series.windows(2) {
            let [t0, v0] = w[0];
            let [t1, v1] = w[1];

            if t0 <= t_min && t_min < t1 {
                x_min_ = x;
            }

            x += (t1 - t0) * (v0 + v1) * 0.5;
            position_series.push([t1, x]);
        }

        // shift
        let x_min = rand::Rng::sample(rng, distr.x_min);
        let x_o = x_min_ - x_min;
        let mut t_o = 0.0;
        for w in position_series.windows(2) {
            let [t, x0] = w[0];
            let [_, x1] = w[1];

            if x0 <= x_o && x_o < x1 {
                t_o = t;
            }
        }
        for [t, _] in velocity_series.iter_mut() {
            *t -= t_o;
        }
        for [t, x] in position_series.iter_mut() {
            *t -= t_o;
            *x -= x_o;
        }

        Self {
            c_in,
            c_out,
            v_min,
            x_min,
            t_min,
            t_exit: t_min + t_next,
            velocity_series,
            position_series,
        }
    }
}
