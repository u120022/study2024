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
    v_in: f64,
    v_out: f64,
    angle: f64,
    radius: f64,
    padding: f64,
    large: bool,
}

impl Default for App {
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

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sumulation");

            let widget =
                egui::Slider::new(&mut self.v_in, 0.0..=100.0).text("inflow velocity[m/sec]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.v_out, 0.0..=100.0).text("outflow velocity[m/sec]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.angle, 0.0..=180.0).text("intersection angle[deg]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.radius, 0.0..=30.0).text("border radius[m]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.padding, 0.0..=10.0).text("distance from border[m]");
            ui.add(widget);

            let widget = egui::Checkbox::new(&mut self.large, "large car type");
            ui.add(widget);
            let dummy_large = if self.large { 1.0 } else { 0.0 };

            // c_in parameter
            let a = nalgebra::vector![2.09, 0.256, -0.0155, 0.0, -0.168, 0.0];
            let x = nalgebra::vector![
                1.0,
                self.v_in,
                self.angle,
                self.radius,
                self.padding,
                self.v_out
            ];
            let shape = a.dot(&x).max(f64::EPSILON);
            let b = nalgebra::vector![0.0573, -0.00173, -0.00109, 0.00219, 0.0];
            let y = nalgebra::vector![1.0, self.v_in, self.radius, self.padding, self.v_out];
            let scale = b.dot(&y).max(f64::EPSILON);
            ui.label(format!("c_out G({shape}, {scale})"));
            let distr_c_in = rand_distr::Gamma::new(shape, scale).unwrap();

            // c_out parameter
            let a = nalgebra::vector![1.40, 0.0, 0.0, 0.0, 0.0633, -0.0224];
            let x = nalgebra::vector![
                1.0,
                self.v_in,
                self.angle,
                self.radius,
                self.padding,
                self.v_out
            ];
            let shape = a.dot(&x).max(f64::EPSILON);
            let b = nalgebra::vector![0.0772, 0.0, 0.0, 0.0, -0.00355];
            let y = nalgebra::vector![1.0, self.v_in, self.radius, self.padding, self.v_out];
            let scale = b.dot(&y).max(f64::EPSILON);
            ui.label(format!("c_out G({shape}, {scale})"));
            let distr_c_out = rand_distr::Gamma::new(shape, scale).unwrap();

            // v_min parameter
            let a = nalgebra::vector![-0.301, 0.0908, 0.0607, 0.0387, 0.233, -0.496];
            let x = nalgebra::vector![
                1.0,
                self.v_in,
                self.radius,
                self.angle,
                self.padding,
                dummy_large
            ];
            let mean = a.dot(&x);
            let b = nalgebra::vector![0.665, 0.0, 0.0419];
            let y = nalgebra::vector![1.0, self.radius, self.padding];
            let std_dev = b.dot(&y).max(f64::EPSILON);
            ui.label(format!("v_min N({mean}, {std_dev})"));
            let distr_v_min = rand_distr::Normal::new(mean, std_dev).unwrap();

            let mut rng = rand::thread_rng();
            let c_in = rand::Rng::sample(&mut rng, distr_c_in);
            let c_out = rand::Rng::sample(&mut rng, distr_c_out);
            let v_min = rand::Rng::sample(&mut rng, distr_v_min);
            ui.label(format!("c_in: {c_in}, c_out: {c_out}, v_min: {v_min}"));

            // inflow
            let t_min = (2.0 / c_in * (self.v_in - v_min)).cbrt();
            let a = c_in;
            let b = -3.0 / 2.0 * a * t_min;
            let c = 0.0;
            let d = self.v_in;
            let mut t = 0.0;
            let mut series = vec![];
            while t <= t_min {
                if t > 100.0 {
                    break;
                }
                let point = [t, a * t.powi(3) + b * t.powi(2) + c * t + d];
                series.push(point);
                t += 0.01;
            }
            let line_inflow = egui_plot::Line::new(series);

            // outflow
            let t_next = (2.0 / -c_out * (v_min - self.v_out)).cbrt();
            let a = -c_out;
            let b = -3.0 / 2.0 * a * t_next;
            let c = 0.0;
            let d = v_min;
            let mut t = 0.0;
            let mut series = vec![];
            while t <= t_next {
                if t > 100.0 {
                    break;
                }
                let point = [t + t_min, a * t.powi(3) + b * t.powi(2) + c * t + d];
                series.push(point);
                t += 0.01;
            }
            let line_outflow = egui_plot::Line::new(series);

            // plot
            egui_plot::Plot::new("Velocity-Time Plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(line_inflow);
                    plot_ui.line(line_outflow);
                });
        });
    }
}
