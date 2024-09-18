use eframe::egui;

#[derive(Debug, Clone, Default)]
pub struct IgPedComponent {
    var: IgPedVar,
    data: Option<IgPedData>,
}

impl IgPedComponent {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.scope(|ui| {
            ui.heading("Inter-green Pedestrian");

            let widget = egui::Slider::new(&mut self.var.t_blink, 0.0..=60.0)
                .text("Elapsed time when blinking green light[s]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.var.v_blink, 0.0..=10.0)
                .text("Velocity when blinking green light[m/sec]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.var.l_init, 0.0..=10.0)
                .text("Distance from crosswalk[m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.var.x_init, -10.0..=10.0).text("Init x[m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.var.width, 0.0..=30.0).text("Road width[m]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.var.cw_width, 0.0..=10.0).text("Crosswalk width[m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.var.cw_setback, 0.0..=30.0)
                .text("Crosswalk setback[m]");
            ui.add(widget);

            let widget = egui::Checkbox::new(&mut self.var.far_side, "Is far-side");
            ui.add(widget);

            let widget = egui::Checkbox::new(&mut self.var.diagonal, "Is diagonal");
            ui.add(widget);

            let widget = egui::Checkbox::new(&mut self.var.center_side, "Is center-side");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.var.lt_veh_flow, 0.0..=1.0)
                .text("Left-turned vehicle flow[1/m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.var.forward_ped_flow, 0.0..=1.0)
                .text("Forward pedestrian flow[1/m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.var.backward_ped_flow, 0.0..=1.0)
                .text("Backward pedestrian flow[1/m]");
            ui.add(widget);

            if ui.button("Compute").clicked() {
                self.data = IgPedData::sample(&mut rand::thread_rng(), &self.var);
            }

            // plot
            if let Some(data) = &self.data {
                let mut text = String::new();
                text.push_str(&format!("v_1: {:.4}", data.v_1));
                text.push_str(&format!("v_2: {:.4}", data.v_2));
                text.push_str(&format!("x_1: {:.4}", data.x_1));
                text.push_str(&format!("x_2: {:.4}", data.x_2));
                text.push_str(&format!("x_3: {:.4}", data.x_3));
                ui.label(egui::RichText::new(text).monospace());

                ui.label("Velocity-Length Plot");
                let position_series = vec![
                    [0.0, data.v_0],
                    [self.var.width * 0.5, data.v_1],
                    [self.var.width, data.v_2],
                ];
                egui_plot::Plot::new("Velocity-Length Plot")
                    .view_aspect(2.0)
                    .allow_scroll(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(egui_plot::Line::new(position_series));
                    });

                ui.label("Position-Length Plot");
                let position_series = vec![
                    [0.0, data.x_1],
                    [self.var.width * 0.5, data.x_2],
                    [self.var.width, data.x_3],
                ];
                egui_plot::Plot::new("Position-Length Plot")
                    .view_aspect(2.0)
                    .allow_scroll(false)
                    .show(ui, |plot_ui| {
                        plot_ui.hline(egui_plot::HLine::new(0.0));
                        plot_ui.hline(egui_plot::HLine::new(self.var.cw_width));
                        plot_ui.line(egui_plot::Line::new(position_series));
                    });
            }
        })
        .response
    }
}

#[derive(Debug, Clone)]
pub struct IgPedVar {
    pub t_blink: f64,
    pub v_blink: f64,
    pub l_init: f64,
    pub x_init: f64,
    pub width: f64,
    pub cw_width: f64,
    pub cw_setback: f64,
    pub far_side: bool,
    pub diagonal: bool,
    pub center_side: bool,
    pub lt_veh_flow: f64,
    pub forward_ped_flow: f64,
    pub backward_ped_flow: f64,
}

impl Default for IgPedVar {
    fn default() -> Self {
        Self {
            t_blink: 0.6,
            v_blink: 1.0,
            l_init: 3.0,
            x_init: 1.0,
            width: 17.0,
            cw_width: 4.5,
            cw_setback: 13.0,
            far_side: false,
            diagonal: false,
            center_side: false,
            lt_veh_flow: 0.01,
            forward_ped_flow: 0.01,
            backward_ped_flow: 0.01,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IgPedData {
    pub v_0: f64,
    pub v_1: f64,
    pub v_2: f64,
    pub x_1: f64,
    pub x_2: f64,
    pub x_3: f64,
}

impl IgPedData {
    pub fn sample(rng: &mut impl rand::Rng, var: &IgPedVar) -> Option<Self> {
        // contact velocity
        let a = nalgebra::matrix![0.256, 24.1];
        let x = nalgebra::matrix![var.l_init, 1.0];
        let shape = a.dot(&x).max(f64::EPSILON);
        let b = nalgebra::matrix![0.0379, 0.0218];
        let y = nalgebra::matrix![var.v_blink, 1.0];
        let scale = b.dot(&y).max(f64::EPSILON);
        let v_0 = rand_distr::Gamma::new(shape, scale).unwrap();
        let v_0 = rand::Rng::sample(rng, v_0);

        // first half velocity
        let a = nalgebra::vector![3.88, 0.129, -3.51];
        let x = nalgebra::vector![v_0, var.width, 1.0];
        let shape = a.dot(&x).max(f64::EPSILON);
        let b = nalgebra::vector![-0.0144, 0.0158, 0.170];
        let y = nalgebra::vector![v_0, var.t_blink, 1.0];
        let scale = b.dot(&y).max(f64::EPSILON);
        let c = nalgebra::vector![-0.000055, 0.777];
        let z = nalgebra::vector![var.forward_ped_flow + var.backward_ped_flow, 1.0];
        let shift = c.dot(&z).max(f64::EPSILON);
        let v_1 = rand_distr::Gamma::new(shape, scale).unwrap();
        let v_1 = rand::Rng::sample(rng, v_1) + shift;

        // last half velocity
        let a = nalgebra::vector![0.580, 6.67];
        let x = nalgebra::vector![v_1, 1.0];
        let shape = a.dot(&x).max(f64::EPSILON);
        let b = nalgebra::vector![0.0862, -0.00333];
        let y = nalgebra::vector![v_1, 1.0];
        let scale = b.dot(&y).max(f64::EPSILON);
        let c = nalgebra::vector![0.218, -0.0597, 0.499];
        let z = nalgebra::vector![v_1, if !var.far_side { 1.0 } else { 0.0 }, 1.0];
        let shift = c.dot(&z).max(f64::EPSILON);
        let v_2 = rand_distr::Gamma::new(shape, scale).unwrap();
        let v_2 = rand::Rng::sample(rng, v_2) + shift;

        // first x
        let a = nalgebra::vector![0.210, -0.0200, -0.220, -1.03, -1.06, 0.100, -6.36, 0.0, 2.11];
        let x = nalgebra::vector![
            var.cw_width,
            var.cw_setback,
            if var.far_side { 1.0 } else { 0.0 },
            if var.diagonal { 1.0 } else { 0.0 },
            if var.center_side { 1.0 } else { 0.0 },
            var.x_init,
            var.lt_veh_flow,
            var.forward_ped_flow,
            1.0
        ];
        let shape = a.dot(&x).max(f64::EPSILON);
        let b = nalgebra::vector![-0.0400, 0.0, 0.0, 0.0, -0.660, 2.31];
        let y = nalgebra::vector![
            var.width,
            var.cw_width,
            var.lt_veh_flow,
            var.forward_ped_flow,
            var.forward_ped_flow,
            1.0
        ];
        let scale = b.dot(&y).max(f64::EPSILON);
        let x_1 = rand_distr::Weibull::new(scale, shape).unwrap();
        let x_1 = rand::Rng::sample(rng, x_1).max(0.0).min(var.cw_width);

        // mid x
        let a = nalgebra::vector![-0.540, 0.0, 0.0, -0.390, 0.440, 0.830, 0.110, 2.16, 3.51];
        let x = nalgebra::vector![
            var.cw_width,
            var.cw_setback,
            if var.far_side { 1.0 } else { 0.0 },
            if var.diagonal { 1.0 } else { 0.0 },
            if var.center_side { 1.0 } else { 0.0 },
            x_1,
            var.lt_veh_flow,
            var.forward_ped_flow,
            1.0
        ];
        let shape = a.dot(&x).max(f64::EPSILON);
        let b = nalgebra::vector![0.0, 1.13, 0.0, 0.0, -0.950, -1.86];
        let y = nalgebra::vector![
            var.width,
            var.cw_width,
            var.lt_veh_flow,
            var.forward_ped_flow,
            var.forward_ped_flow,
            1.0
        ];
        let scale = b.dot(&y).max(f64::EPSILON);
        let x_2 = rand_distr::Weibull::new(scale, shape).unwrap();
        let x_2 = rand::Rng::sample(rng, x_2).max(0.0).min(var.cw_width);

        // last x
        let a = nalgebra::vector![0.450, 0.0200, 0.150, -0.660, -0.220, 0.200, 0.0, 0.0, -1.19];
        let x = nalgebra::vector![
            var.cw_width,
            var.cw_setback,
            if var.far_side { 1.0 } else { 0.0 },
            if var.diagonal { 1.0 } else { 0.0 },
            if var.center_side { 1.0 } else { 0.0 },
            x_2,
            var.lt_veh_flow,
            var.forward_ped_flow,
            1.0
        ];
        let shape = a.dot(&x).max(f64::EPSILON);
        let b = nalgebra::vector![0.0, 1.0, -10.5, 6.93, -1.69, -1.94];
        let y = nalgebra::vector![
            var.width,
            var.cw_width,
            var.lt_veh_flow,
            var.forward_ped_flow,
            var.forward_ped_flow + var.backward_ped_flow,
            1.0
        ];
        let scale = b.dot(&y).max(f64::EPSILON);
        let x_3 = rand_distr::Weibull::new(scale, shape).unwrap();
        let x_3 = rand::Rng::sample(rng, x_3).max(0.0).min(var.cw_width);

        Some(Self {
            v_0,
            v_1,
            v_2,
            x_1,
            x_2,
            x_3,
        })
    }
}
