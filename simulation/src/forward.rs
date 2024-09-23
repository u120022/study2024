use crate::{compute, settings};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VehSignalState {
    Green,
    Yellow,
    Red,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PedSignalState {
    Green,
    Blink,
    Red,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Forward {
    pub settings: settings::Settings,
    pub veh_signals: ahash::AHashMap<[settings::Dir; 2], VehSignalState>,
    pub ped_signals: ahash::AHashMap<[settings::Dir; 2], PedSignalState>,
    pub elapsed_time: f64,
    pub trajectories: Vec<Vec<[f64; 2]>>,
}

impl Forward {
    pub fn new(settings: settings::Settings) -> Self {
        Self {
            settings,
            veh_signals: ahash::AHashMap::new(),
            ped_signals: ahash::AHashMap::new(),
            elapsed_time: 0.0,
            trajectories: Default::default(),
        }
    }

    pub fn forward(&mut self, delta_secs: f64) {
        // vehicle signals
        for i in 0..self.settings.veh_signals.len() {
            let signal = &self.settings.veh_signals[i];

            let s0 = signal.offset_secs;
            let s1 = signal.offset_secs + signal.green_secs;
            let s2 = signal.offset_secs + signal.green_secs + signal.yellow_secs;

            let intime = self.elapsed_time % signal.cycle_secs;
            if s0 <= intime && intime < s1 {
                self.veh_signals
                    .insert([signal.src_dir, signal.dst_dir], VehSignalState::Green);
            } else if s1 <= intime && intime < s2 {
                self.veh_signals
                    .insert([signal.src_dir, signal.dst_dir], VehSignalState::Yellow);
            } else {
                self.veh_signals
                    .insert([signal.src_dir, signal.dst_dir], VehSignalState::Red);
            }
        }

        // pedestrian signals
        for i in 0..self.settings.ped_signals.len() {
            let signal = &self.settings.ped_signals[i];

            let s0 = signal.offset_secs;
            let s1 = signal.offset_secs + signal.green_secs;
            let s2 = signal.offset_secs + signal.green_secs + signal.blink_secs;

            let intime = self.elapsed_time % signal.cycle_secs;
            if s0 <= intime && intime < s1 {
                self.ped_signals
                    .insert([signal.src_dir, signal.dst_dir], PedSignalState::Green);
            } else if s1 <= intime && intime < s2 {
                self.ped_signals
                    .insert([signal.src_dir, signal.dst_dir], PedSignalState::Blink);
            } else {
                self.ped_signals
                    .insert([signal.src_dir, signal.dst_dir], PedSignalState::Red);
            }
        }

        self.trajectories.clear();

        // left-turn vehicle
        for flow in &self.settings.lt_veh_flows {
            if let Some(output) = compute::compute_lt_veh(&self.settings, flow) {
                self.trajectories.push(output.trajectory_series);
            }
        }

        // right-turn vehicle
        for flow in &self.settings.rt_veh_flows {
            if let Some(output) = compute::compute_rt_veh(&self.settings, flow) {
                self.trajectories.push(output.trajectory_series);
            }
        }

        // pedestrian
        for flow in &self.settings.ped_flows {
            if let Some(output) = compute::compute_ped(&self.settings, flow) {
                self.trajectories.push(output.trajectory_series);
            }
        }

        // inter-green pedestrian
        for flow in &self.settings.ig_ped_flows {
            if let Some(output) = compute::compute_ig_ped(&self.settings, flow) {
                self.trajectories.push(output.trajectory_series);
            }
        }

        self.elapsed_time += delta_secs;
    }

    pub fn show_simulation_inside(&mut self, ui: &mut egui::Ui) {
        let mut lines = vec![];

        for trajectory in &self.trajectories {
            let line = egui_plot::Line::new(trajectory.clone());
            lines.push(line);
        }

        self.settings.show_simulation_inside(ui, |plot_ui| {
            lines.into_iter().for_each(|v| plot_ui.line(v));
        });
    }

    pub fn show_schedule_inside(&mut self, ui: &mut egui::Ui) {
        let mut points = vec![];

        for i in 0..self.settings.veh_signals.len() {
            let signal = &self.settings.veh_signals[i];
            let layer = i as f64;
            let intime = self.elapsed_time % signal.cycle_secs;
            points.push([intime, layer]);
        }

        for i in 0..self.settings.ped_signals.len() {
            let signal = &self.settings.ped_signals[i];
            let layer = (self.settings.veh_signals.len() + i) as f64;
            let intime = self.elapsed_time % signal.cycle_secs;
            points.push([intime, layer]);
        }

        self.settings.show_schedule_inside(ui, |plot_ui| {
            let points = egui_plot::Points::new(points).color(egui::Color32::RED);
            plot_ui.points(points);
        });
    }
}
