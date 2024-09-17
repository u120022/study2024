use eframe::egui;

use crate::*;

#[derive(Debug, Clone, Default)]
pub struct FieldComponent {
    var: FieldVar,
    sim: Option<Simulator>,
}

impl FieldComponent {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let r = nalgebra::Rotation2::new(self.var.angle.to_radians());
        let mut lines = vec![];
        const ROAD_LENGTH: f64 = 30.0;
        const SUBDIVISION: usize = 64;

        // along center line
        let p0: [f64; 2] = nalgebra::Point2::new(0.0, -ROAD_LENGTH).into();
        let p1: [f64; 2] = nalgebra::Point2::new(0.0, ROAD_LENGTH).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GREEN);
        lines.push(line);
        // across center line
        let p0: [f64; 2] = (r * nalgebra::Point2::new(0.0, -ROAD_LENGTH)).into();
        let p1: [f64; 2] = (r * nalgebra::Point2::new(0.0, ROAD_LENGTH)).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GREEN);
        lines.push(line);

        // along positive boundary
        let x = self.var.width_along * 0.5;
        let p0: [f64; 2] = nalgebra::Point2::new(x, -ROAD_LENGTH).into();
        let p1: [f64; 2] = nalgebra::Point2::new(x, ROAD_LENGTH).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);
        // along negative boundary
        let x = -self.var.width_along * 0.5;
        let p0: [f64; 2] = nalgebra::Point2::new(x, -ROAD_LENGTH).into();
        let p1: [f64; 2] = nalgebra::Point2::new(x, ROAD_LENGTH).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);
        // across positive boundary
        let x = self.var.width_across * 0.5;
        let p0: [f64; 2] = (r * nalgebra::Point2::new(x, -ROAD_LENGTH)).into();
        let p1: [f64; 2] = (r * nalgebra::Point2::new(x, ROAD_LENGTH)).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);
        // across negative boundary
        let x = -self.var.width_across * 0.5;
        let p0: [f64; 2] = (r * nalgebra::Point2::new(x, -ROAD_LENGTH)).into();
        let p1: [f64; 2] = (r * nalgebra::Point2::new(x, ROAD_LENGTH)).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);

        // along lane line
        for &x in &self.var.lane_along {
            let p0: [f64; 2] = nalgebra::Point2::new(x, -ROAD_LENGTH).into();
            let p1: [f64; 2] = nalgebra::Point2::new(x, ROAD_LENGTH).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::RED);
            lines.push(line);
        }
        // across lane line
        for &x in &self.var.lane_across {
            let p0: [f64; 2] = (r * nalgebra::Point2::new(x, -ROAD_LENGTH)).into();
            let p1: [f64; 2] = (r * nalgebra::Point2::new(x, ROAD_LENGTH)).into();
            let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::RED);
            lines.push(line);
        }

        // along negative hard nose
        let p0: [f64; 2] = nalgebra::Point2::new(0.0, -ROAD_LENGTH).into();
        let p1: [f64; 2] = nalgebra::Point2::new(0.0, -self.var.hn_along).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);
        // along positive hard nose
        let p0: [f64; 2] = nalgebra::Point2::new(0.0, ROAD_LENGTH).into();
        let p1: [f64; 2] = nalgebra::Point2::new(0.0, self.var.hn_along).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);
        // across negative line
        let p0: [f64; 2] = (r * nalgebra::Point2::new(0.0, -ROAD_LENGTH)).into();
        let p1: [f64; 2] = (r * nalgebra::Point2::new(0.0, -self.var.hn_across)).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);
        // across positive line
        let p0: [f64; 2] = (r * nalgebra::Point2::new(0.0, ROAD_LENGTH)).into();
        let p1: [f64; 2] = (r * nalgebra::Point2::new(0.0, self.var.hn_across)).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::GRAY);
        lines.push(line);

        // along positive crosswalk
        let x = self.var.width_along * 0.5;
        let y_min = self.var.cw_setback_along;
        let y_max = self.var.cw_setback_along + self.var.cw_width_along;
        let p0: [f64; 2] = nalgebra::Point2::new(-x, y_min).into();
        let p1: [f64; 2] = nalgebra::Point2::new(x, y_min).into();
        let p2: [f64; 2] = nalgebra::Point2::new(x, y_max).into();
        let p3: [f64; 2] = nalgebra::Point2::new(-x, y_max).into();
        let line = egui_plot::Line::new(vec![p0, p1, p2, p3, p0]).color(egui::Color32::YELLOW);
        lines.push(line);
        // along negative crosswalk
        let x = self.var.width_along * 0.5;
        let y_min = -self.var.cw_setback_along;
        let y_max = -self.var.cw_setback_along - self.var.cw_width_along;
        let p0: [f64; 2] = nalgebra::Point2::new(-x, y_min).into();
        let p1: [f64; 2] = nalgebra::Point2::new(x, y_min).into();
        let p2: [f64; 2] = nalgebra::Point2::new(x, y_max).into();
        let p3: [f64; 2] = nalgebra::Point2::new(-x, y_max).into();
        let line = egui_plot::Line::new(vec![p0, p1, p2, p3, p0]).color(egui::Color32::YELLOW);
        lines.push(line);
        // across positive crosswalk
        let x = self.var.width_across * 0.5;
        let y_min = self.var.cw_setback_across;
        let y_max = self.var.cw_setback_across + self.var.cw_width_across;
        let p0: [f64; 2] = (r * nalgebra::Point2::new(-x, y_min)).into();
        let p1: [f64; 2] = (r * nalgebra::Point2::new(x, y_min)).into();
        let p2: [f64; 2] = (r * nalgebra::Point2::new(x, y_max)).into();
        let p3: [f64; 2] = (r * nalgebra::Point2::new(-x, y_max)).into();
        let line = egui_plot::Line::new(vec![p0, p1, p2, p3, p0]).color(egui::Color32::YELLOW);
        lines.push(line);
        // across negative crosswalk
        let x = self.var.width_across * 0.5;
        let y_min = -self.var.cw_setback_across;
        let y_max = -self.var.cw_setback_across - self.var.cw_width_across;
        let p0: [f64; 2] = (r * nalgebra::Point2::new(-x, y_min)).into();
        let p1: [f64; 2] = (r * nalgebra::Point2::new(x, y_min)).into();
        let p2: [f64; 2] = (r * nalgebra::Point2::new(x, y_max)).into();
        let p3: [f64; 2] = (r * nalgebra::Point2::new(-x, y_max)).into();
        let line = egui_plot::Line::new(vec![p0, p1, p2, p3, p0]).color(egui::Color32::YELLOW);
        lines.push(line);

        // along positive stop line
        let x = self.var.width_along * 0.5;
        let p0: [f64; 2] = nalgebra::Point2::new(-x, self.var.sl_setback_along).into();
        let p1: [f64; 2] = nalgebra::Point2::new(x, self.var.sl_setback_along).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::YELLOW);
        lines.push(line);
        // along negative stop line
        let x = self.var.width_along * 0.5;
        let p0: [f64; 2] = nalgebra::Point2::new(-x, -self.var.sl_setback_along).into();
        let p1: [f64; 2] = nalgebra::Point2::new(x, -self.var.sl_setback_along).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::YELLOW);
        lines.push(line);
        // across positive crosswalk
        let x = self.var.width_across * 0.5;
        let p0: [f64; 2] = (r * nalgebra::Point2::new(-x, self.var.sl_setback_across)).into();
        let p1: [f64; 2] = (r * nalgebra::Point2::new(x, self.var.sl_setback_across)).into();
        let line = egui_plot::Line::new(vec![p0, p1]).color(egui::Color32::YELLOW);
        lines.push(line);
        // across negative crosswalk
        let x = self.var.width_across * 0.5;
        let p0: [f64; 2] = (r * nalgebra::Point2::new(-x, -self.var.sl_setback_across)).into();
        let p1: [f64; 2] = (r * nalgebra::Point2::new(x, -self.var.sl_setback_across)).into();
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
                if -ROAD_LENGTH <= x && x <= ROAD_LENGTH && -ROAD_LENGTH <= y && y <= ROAD_LENGTH {
                    points.push([x, y]);
                }
            }
            points
        }
        let mut points = vec![];
        // radius border
        let x = -self.var.width_along * 0.5 - self.var.radius;
        let p0: [f64; 2] = nalgebra::Point2::new(x, -ROAD_LENGTH).into();
        let p1: [f64; 2] = nalgebra::Point2::new(x, ROAD_LENGTH).into();
        let x = -self.var.width_along * 0.5 - self.var.radius;
        let q0: [f64; 2] = (r * nalgebra::Point2::new(x, -ROAD_LENGTH)).into();
        let q1: [f64; 2] = (r * nalgebra::Point2::new(x, ROAD_LENGTH)).into();
        let point = egui_plot::Points::new(radius_border(p0, p1, q0, q1, self.var.radius))
            .color(egui::Color32::GRAY);
        points.push(point);
        // radius border
        let x = -self.var.width_along * 0.5 - self.var.radius;
        let p0: [f64; 2] = nalgebra::Point2::new(x, -ROAD_LENGTH).into();
        let p1: [f64; 2] = nalgebra::Point2::new(x, ROAD_LENGTH).into();
        let x = self.var.width_along * 0.5 + self.var.radius;
        let q0: [f64; 2] = (r * nalgebra::Point2::new(x, -ROAD_LENGTH)).into();
        let q1: [f64; 2] = (r * nalgebra::Point2::new(x, ROAD_LENGTH)).into();
        let point = egui_plot::Points::new(radius_border(p0, p1, q0, q1, self.var.radius))
            .color(egui::Color32::GRAY);
        points.push(point);
        // radius border
        let x = self.var.width_along * 0.5 + self.var.radius;
        let p0: [f64; 2] = nalgebra::Point2::new(x, -ROAD_LENGTH).into();
        let p1: [f64; 2] = nalgebra::Point2::new(x, ROAD_LENGTH).into();
        let x = -self.var.width_along * 0.5 - self.var.radius;
        let q0: [f64; 2] = (r * nalgebra::Point2::new(x, -ROAD_LENGTH)).into();
        let q1: [f64; 2] = (r * nalgebra::Point2::new(x, ROAD_LENGTH)).into();
        let point = egui_plot::Points::new(radius_border(p0, p1, q0, q1, self.var.radius))
            .color(egui::Color32::GRAY);
        points.push(point);
        // radius border
        let x = self.var.width_along * 0.5 + self.var.radius;
        let p0: [f64; 2] = nalgebra::Point2::new(x, -ROAD_LENGTH).into();
        let p1: [f64; 2] = nalgebra::Point2::new(x, ROAD_LENGTH).into();
        let x = self.var.width_along * 0.5 + self.var.radius;
        let q0: [f64; 2] = (r * nalgebra::Point2::new(x, -ROAD_LENGTH)).into();
        let q1: [f64; 2] = (r * nalgebra::Point2::new(x, ROAD_LENGTH)).into();
        let point = egui_plot::Points::new(radius_border(p0, p1, q0, q1, self.var.radius))
            .color(egui::Color32::GRAY);
        points.push(point);

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
                    self.sim = Some(Simulator::new(self.var.clone(), LtVehVar::default()));
                }
            });

            if let Some(sim) = &mut self.sim {
                sim.forward(0.016);

                for (id, (data, time, position)) in &sim.lt_veh_agent {
                    let point = egui_plot::Points::new(vec![*position])
                        .color(egui::Color32::BLUE)
                        .radius(2.0);
                    points.push(point);
                }
            }

            egui_plot::Plot::new("Field Plot")
                .view_aspect(1.0)
                .data_aspect(1.0)
                .allow_scroll(false)
                .show(ui, |plot_ui| {
                    lines.into_iter().for_each(|l| plot_ui.line(l));
                    points.into_iter().for_each(|p| plot_ui.points(p));
                });
        })
        .response
    }
}

#[derive(Debug, Clone)]
pub struct FieldVar {
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
}

impl Default for FieldVar {
    fn default() -> Self {
        FieldVar {
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
        }
    }
}
