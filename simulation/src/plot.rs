use crate::*;

pub const ROAD_LENGTH: f64 = 64.0;

impl settings::Settings {
    pub fn show_simulation_inside(
        &mut self,
        ui: &mut egui::Ui,
        overlay_fn: impl FnOnce(&mut egui_plot::PlotUi),
    ) {
        let mut points = vec![];
        let mut lines = vec![];

        let m = nalgebra::Rotation2::new(self.angle.to_radians());

        // X center line
        let p0 = nalgebra::Point2::new(-ROAD_LENGTH, 0.0).into();
        let p1 = nalgebra::Point2::new(ROAD_LENGTH, 0.0).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GREEN);
        lines.push(line);
        // Y center line
        let p0 = (m * nalgebra::Point2::new(-ROAD_LENGTH, 0.0)).into();
        let p1 = (m * nalgebra::Point2::new(ROAD_LENGTH, 0.0)).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GREEN);
        lines.push(line);

        // Px boundary
        let y = self.width_along * 0.5;
        let p0 = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
        let p1 = nalgebra::Point2::new(ROAD_LENGTH, y).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);
        // Nx boundary
        let y = -self.width_along * 0.5;
        let p0 = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
        let p1 = nalgebra::Point2::new(ROAD_LENGTH, y).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);
        // Py boundary
        let y = self.width_across * 0.5;
        let p0 = (m * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
        let p1 = (m * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);
        // Ny boundary
        let y = -self.width_across * 0.5;
        let p0 = (m * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
        let p1 = (m * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);

        // X lane line
        for &y in &self.lane_along {
            let p0 = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
            let p1 = nalgebra::Point2::new(ROAD_LENGTH, y).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::RED);
            lines.push(line);
        }
        // Y lane line
        for &y in &self.lane_across {
            let p0 = (m * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
            let p1 = (m * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::RED);
            lines.push(line);
        }

        // Px hard nose
        let p0 = nalgebra::Point2::new(ROAD_LENGTH, 0.0).into();
        let p1 = nalgebra::Point2::new(self.hn_along, 0.0).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);
        // Nx hard nose
        let p0 = nalgebra::Point2::new(-ROAD_LENGTH, 0.0).into();
        let p1 = nalgebra::Point2::new(-self.hn_along, 0.0).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);
        // Py line
        let p0 = (m * nalgebra::Point2::new(ROAD_LENGTH, 0.0)).into();
        let p1 = (m * nalgebra::Point2::new(self.hn_across, 0.0)).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);
        // Ny line
        let p0 = (m * nalgebra::Point2::new(-ROAD_LENGTH, 0.0)).into();
        let p1 = (m * nalgebra::Point2::new(-self.hn_across, 0.0)).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);

        // Px crosswalk
        let y = self.width_along * 0.5;
        let x_min = self.cw_setback_along;
        let x_max = self.cw_setback_along + self.cw_width_along;
        let p0 = nalgebra::Point2::new(x_min, -y).into();
        let p1 = nalgebra::Point2::new(x_min, y).into();
        let p2 = nalgebra::Point2::new(x_max, y).into();
        let p3 = nalgebra::Point2::new(x_max, -y).into();
        let line = egui_plot::Line::new(vec![p0, p1, p2, p3, p0]).color(egui::Color32::YELLOW);
        lines.push(line);
        // Nx crosswalk
        let y = self.width_along * 0.5;
        let x_min = -self.cw_setback_along;
        let x_max = -self.cw_setback_along - self.cw_width_along;
        let p0 = nalgebra::Point2::new(x_min, -y).into();
        let p1 = nalgebra::Point2::new(x_min, y).into();
        let p2 = nalgebra::Point2::new(x_max, y).into();
        let p3 = nalgebra::Point2::new(x_max, -y).into();
        let line = egui_plot::Line::new(vec![p0, p1, p2, p3, p0]).color(egui::Color32::YELLOW);
        lines.push(line);
        // Py crosswalk
        let y = self.width_across * 0.5;
        let x_min = self.cw_setback_across;
        let x_max = self.cw_setback_across + self.cw_width_across;
        let p0 = (m * nalgebra::Point2::new(x_min, -y)).into();
        let p1 = (m * nalgebra::Point2::new(x_min, y)).into();
        let p2 = (m * nalgebra::Point2::new(x_max, y)).into();
        let p3 = (m * nalgebra::Point2::new(x_max, -y)).into();
        let line = egui_plot::Line::new(vec![p0, p1, p2, p3, p0]).color(egui::Color32::YELLOW);
        lines.push(line);
        // Ny crosswalk
        let y = self.width_across * 0.5;
        let x_min = -self.cw_setback_across;
        let x_max = -self.cw_setback_across - self.cw_width_across;
        let p0 = (m * nalgebra::Point2::new(x_min, -y)).into();
        let p1 = (m * nalgebra::Point2::new(x_min, y)).into();
        let p2 = (m * nalgebra::Point2::new(x_max, y)).into();
        let p3 = (m * nalgebra::Point2::new(x_max, -y)).into();
        let line = egui_plot::Line::new(vec![p0, p1, p2, p3, p0]).color(egui::Color32::YELLOW);
        lines.push(line);

        // Px stop-line
        let y = self.width_along * 0.5;
        let p0 = nalgebra::Point2::new(self.sl_setback_along, -y).into();
        let p1 = nalgebra::Point2::new(self.sl_setback_along, y).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::YELLOW);
        lines.push(line);
        // Nx stop-line
        let y = self.width_along * 0.5;
        let p0 = nalgebra::Point2::new(-self.sl_setback_along, -y).into();
        let p1 = nalgebra::Point2::new(-self.sl_setback_along, y).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::YELLOW);
        lines.push(line);
        // Py stop-line
        let y = self.width_across * 0.5;
        let p0 = (m * nalgebra::Point2::new(self.sl_setback_across, -y)).into();
        let p1 = (m * nalgebra::Point2::new(self.sl_setback_across, y)).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::YELLOW);
        lines.push(line);
        // Ny stop-line
        let y = self.width_across * 0.5;
        let p0 = (m * nalgebra::Point2::new(-self.sl_setback_across, -y)).into();
        let p1 = (m * nalgebra::Point2::new(-self.sl_setback_across, y)).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::YELLOW);
        lines.push(line);

        // PxPy radius border
        let y = self.width_along * 0.5 + self.radius;
        let p0 = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
        let p1 = nalgebra::Point2::new(ROAD_LENGTH, y).into();
        let y = -(self.width_along * 0.5 + self.radius);
        let q0 = (m * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
        let q1 = (m * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
        let point = egui_plot::Points::new(radius_border(p0, p1, q0, q1, self.radius))
            .color(egui::Color32::GRAY);
        points.push(point);
        // NxPy radius border
        let y = self.width_along * 0.5 + self.radius;
        let p0 = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
        let p1 = nalgebra::Point2::new(ROAD_LENGTH, y).into();
        let y = self.width_along * 0.5 + self.radius;
        let q0 = (m * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
        let q1 = (m * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
        let point = egui_plot::Points::new(radius_border(p0, p1, q0, q1, self.radius))
            .color(egui::Color32::GRAY);
        points.push(point);
        // NxNy radius border
        let y = -(self.width_along * 0.5 + self.radius);
        let p0 = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
        let p1 = nalgebra::Point2::new(ROAD_LENGTH, y).into();
        let y = self.width_along * 0.5 + self.radius;
        let q0 = (m * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
        let q1 = (m * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
        let point = egui_plot::Points::new(radius_border(p0, p1, q0, q1, self.radius))
            .color(egui::Color32::GRAY);
        points.push(point);
        // PxNy radius border
        let y = -(self.width_along * 0.5 + self.radius);
        let p0 = nalgebra::Point2::new(-ROAD_LENGTH, y).into();
        let p1 = nalgebra::Point2::new(ROAD_LENGTH, y).into();
        let y = -(self.width_along * 0.5 + self.radius);
        let q0 = (m * nalgebra::Point2::new(-ROAD_LENGTH, y)).into();
        let q1 = (m * nalgebra::Point2::new(ROAD_LENGTH, y)).into();
        let point = egui_plot::Points::new(radius_border(p0, p1, q0, q1, self.radius))
            .color(egui::Color32::GRAY);
        points.push(point);

        egui_plot::Plot::new("Simulation")
            .view_aspect(1.0)
            .data_aspect(1.0)
            .allow_scroll(false)
            .show(ui, |plot_ui| {
                lines.into_iter().for_each(|v| plot_ui.line(v));
                points.into_iter().for_each(|v| plot_ui.points(v));
                overlay_fn(plot_ui);
            });
    }

    pub fn show_schedule_inside(
        &mut self,
        ui: &mut egui::Ui,
        overlay_fn: impl FnOnce(&mut egui_plot::PlotUi),
    ) {
        // Vehicle signals
        let mut texts = vec![];
        let mut lines = vec![];
        for (i, signal) in self.veh_signals.iter().enumerate() {
            let layer = i as f64;

            let text = egui_plot::Text::new(
                [0.0, layer].into(),
                format!("Veh {:?}-{:?}", signal.src_dir, signal.dst_dir),
            );
            texts.push(text);

            let s0 = signal.offset_secs;
            let s1 = signal.offset_secs + signal.green_secs;
            let s2 = signal.offset_secs + signal.green_secs + signal.yellow_secs;

            let p0 = nalgebra::Point2::new(s0, layer).into();
            let p1 = nalgebra::Point2::new(s1, layer).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GREEN);
            lines.push(line);

            let p0 = nalgebra::Point2::new(s1, layer).into();
            let p1 = nalgebra::Point2::new(s2, layer).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::YELLOW);
            lines.push(line);
        }

        for (i, signal) in self.ped_signals.iter().enumerate() {
            let layer = (self.veh_signals.len() + i) as f64;

            let text = egui_plot::Text::new(
                [0.0, layer].into(),
                format!("Ped {:?}-{:?}", signal.src_dir, signal.dst_dir),
            );
            texts.push(text);

            let s0 = signal.offset_secs;
            let s1 = signal.offset_secs + signal.green_secs;
            let s2 = signal.offset_secs + signal.green_secs + signal.blink_secs;

            let p0 = nalgebra::Point2::new(s0, layer).into();
            let p1 = nalgebra::Point2::new(s1, layer).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GREEN);
            lines.push(line);

            let p0 = nalgebra::Point2::new(s1, layer).into();
            let p1 = nalgebra::Point2::new(s2, layer).into();
            let line = egui_plot::Line::new(vec![p0, p1])
                .color(egui::Color32::GREEN)
                .style(egui_plot::LineStyle::dotted_dense());
            lines.push(line);
        }

        egui_plot::Plot::new("TLS schedule")
            .height(100.0)
            .allow_scroll(false)
            .allow_zoom(false)
            .allow_drag(false)
            .show(ui, |plot_ui| {
                texts.into_iter().for_each(|v| plot_ui.text(v));
                lines.into_iter().for_each(|v| plot_ui.line(v));
                overlay_fn(plot_ui);
            });
    }
}

fn radius_border(
    p0: [f64; 2],
    p1: [f64; 2],
    q0: [f64; 2],
    q1: [f64; 2],
    radius: f64,
) -> Vec<[f64; 2]> {
    const SUBDIVISION: usize = 32;

    let o = intersection_point(p0, p1, q0, q1);
    let mut points = vec![];
    for i in 0..=SUBDIVISION {
        let v = i as f64 / SUBDIVISION as f64 * 2.0 * std::f64::consts::PI;
        let x = o[0] + radius * v.cos();
        let y = o[1] + radius * v.sin();
        if (-ROAD_LENGTH..=ROAD_LENGTH).contains(&x) && (-ROAD_LENGTH..=ROAD_LENGTH).contains(&y) {
            points.push([x, y]);
        }
    }
    points
}

pub fn intersection_point(p0: [f64; 2], p1: [f64; 2], q0: [f64; 2], q1: [f64; 2]) -> [f64; 2] {
    let a_p = p1[1] - p0[1];
    let b_p = -(p1[0] - p0[0]);
    let c_p = -(a_p * p0[0] + b_p * p0[1]);
    let a_q = q1[1] - q0[1];
    let b_q = -(q1[0] - q0[0]);
    let c_q = -(a_q * q0[0] + b_q * q0[1]);
    let x_0 = (b_p * c_q - b_q * c_p) / (a_p * b_q - a_q * b_p);
    let y_0 = (-a_p * c_q + a_q * c_p) / (a_p * b_q - a_q * b_p);
    [x_0, y_0]
}
