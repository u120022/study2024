use crate::settings;

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
}

impl Forward {
    pub fn new(settings: settings::Settings) -> Self {
        Self {
            settings,
            veh_signals: ahash::AHashMap::new(),
            ped_signals: ahash::AHashMap::new(),
            elapsed_time: 0.0,
        }
    }

    pub fn forward(&mut self, delta_secs: f64) {
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

        self.elapsed_time += delta_secs;
    }

    pub fn show_simulation_inside(&mut self, ui: &mut egui::Ui) {
        self.settings.show_simulation_inside(ui, |_| {});
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
