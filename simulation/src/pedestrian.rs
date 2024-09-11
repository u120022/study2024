use eframe::egui;

#[derive(Debug, Clone, Default)]
pub struct PedestrianComponent {
    var: PedestrianVar,
    data: Option<PedestrianData>,
}

impl PedestrianComponent {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.scope(|ui| {
            ui.heading("Variable");

            let widget =
                egui::Slider::new(&mut self.var.a_green, 0.0..=1.0).text("green light progress");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.var.v_0, 0.0..=10.0).text("enter velocity[m/sec]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.var.x_crossing, 0.0..=30.0).text("crossing length[m]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.var.w_crossing, 0.0..=10.0).text("crossing width[m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.var.w_crossing0, 0.0..=10.0)
                .text("crossing set-back width[m]");
            ui.add(widget);

            let widget = egui::Checkbox::new(&mut self.var.far_side, "Far-side");
            ui.add(widget);

            let widget = egui::Checkbox::new(&mut self.var.diagonal, "Diagonal");
            ui.add(widget);

            let widget = egui::Checkbox::new(&mut self.var.center_side, "Center-side");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.var.car_flow, 0.0..=100.0)
                .text("left-turned car flow[1/m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.var.pedestrian_flow, 0.0..=100.0)
                .text("pedestrian flow[1/m]");
            ui.add(widget);

            if ui.button("Compute").clicked() {
                self.data = PedestrianData::sample(&mut rand::thread_rng(), &self.var);
            }

            // plot
            if let Some(data) = &self.data {
                ui.heading("Result");

                ui.label(format!("v1: {:.4}", data.v_1));
                ui.label(format!("v2: {:.4}", data.v_2));
            }
        })
        .response
    }
}

#[derive(Debug, Clone)]
struct PedestrianVar {
    a_green: f64,
    v_0: f64,
    w_0: f64,
    x_crossing: f64,
    w_crossing: f64,
    w_crossing0: f64,
    far_side: bool,
    diagonal: bool,
    center_side: bool,
    car_flow: f64,
    pedestrian_flow: f64,
}

impl Default for PedestrianVar {
    fn default() -> Self {
        Self {
            a_green: 0.6,
            v_0: 1.0,
            w_0: 1.0,
            x_crossing: 10.0,
            w_crossing: 3.0,
            w_crossing0: 2.0,
            far_side: false,
            diagonal: false,
            center_side: false,
            car_flow: 0.0,
            pedestrian_flow: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
struct PedestrianData {
    v_1: f64,
    v_2: f64,
}

impl PedestrianData {
    fn sample(rng: &mut impl rand::Rng, var: &PedestrianVar) -> Option<Self> {
        // first half velocity
        let a = nalgebra::vector![7.47, 0.0, 0.720, 4.19, 1.93];
        let x = nalgebra::vector![
            var.v_0,
            0.0,
            var.x_crossing,
            if var.far_side { 1.0 } else { 0.0 },
            1.0
        ];
        let shape = a.dot(&x);
        let b = nalgebra::vector![0.00391, 0.0, -0.00106, -0.00414, 0.00185, 0.0697];
        let y = nalgebra::vector![
            var.v_0,
            0.0,
            var.x_crossing,
            if var.far_side { 1.0 } else { 0.0 },
            var.a_green,
            1.0
        ];
        let scale = b.dot(&y);
        let v_1 = rand_distr::Gamma::new(shape, scale).unwrap();
        let v_1 = rand::Rng::sample(rng, v_1);

        // last half velocity
        let a = nalgebra::vector![0.0, -2.10, 0.695, 4.10, 22.8];
        let x = nalgebra::vector![
            var.v_0,
            v_1,
            var.x_crossing,
            if var.far_side { 1.0 } else { 0.0 },
            1.0
        ];
        let shape = a.dot(&x);
        let b = nalgebra::vector![0.0, 0.0199, -0.0006, -0.00159, 0.0, 0.0256];
        let y = nalgebra::vector![
            var.v_0,
            v_1,
            var.x_crossing,
            if var.far_side { 1.0 } else { 0.0 },
            var.a_green,
            1.0
        ];
        let scale = b.dot(&y);
        let v_2 = rand_distr::Gamma::new(shape, scale).unwrap();
        let v_2 = rand::Rng::sample(rng, v_2);

        // near-side x
        let a = nalgebra::vector![0.210, -0.0200, -0.220, -1.03, -1.06, 0.100, -6.36, 0.0, 2.11];
        let x = nalgebra::vector![
            var.w_crossing,
            var.w_crossing0,
            if var.far_side { 1.0 } else { 0.0 },
            if var.diagonal { 1.0 } else { 0.0 },
            if var.center_side { 1.0 } else { 0.0 },
            var.w_0,
            var.car_flow,
            var.pedestrian_flow,
            1.0
        ];
        let shape = a.dot(&x);
        let b = nalgebra::vector![-0.0400, 0.0, 0.0, 0.0, -0.660, 2.31];
        let y = nalgebra::vector![
            var.x_crossing,
            var.w_crossing,
            var.car_flow,
            var.pedestrian_flow,
            var.pedestrian_flow,
            1.0
        ];
        let scale = b.dot(&y);
        let x_1 = rand_distr::Weibull::new(scale, shape).unwrap();
        let x_1 = rand::Rng::sample(rng, x_1);

        Some(Self { v_1, v_2 })
    }
}
