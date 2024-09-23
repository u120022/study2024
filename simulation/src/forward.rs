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
    pub next_spawns: ahash::AHashMap<String, f64>,
    pub trajectories: Vec<(Vec<[f64; 2]>, usize)>,
}

impl Forward {
    pub fn new(settings: settings::Settings) -> Self {
        Self {
            settings,
            veh_signals: ahash::AHashMap::new(),
            ped_signals: ahash::AHashMap::new(),
            elapsed_time: 0.0,
            next_spawns: Default::default(),
            trajectories: Default::default(),
        }
    }

    pub fn forward(&mut self, delta_secs: f64) {
        let rng = &mut rand::thread_rng();

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

        // left-turn vehicle
        for i in 0..self.settings.lt_veh_flows.len() {
            let flow = &self.settings.lt_veh_flows[i];
            let signal = self.veh_signals.get(&[flow.src_dir, flow.dst_dir]).unwrap();

            if signal == &VehSignalState::Red {
                continue;
            }

            let id = format!("lt_veh_{i}");
            let next_spawn = self.next_spawns.entry(id).or_insert(0.0);
            *next_spawn -= delta_secs;
            if *next_spawn > 0.0 {
                continue;
            }

            if let Some(output) = compute::compute_lt_veh(&self.settings, flow) {
                self.trajectories.push((output.trajectory_series, 0));
            }
            let distr = rand_distr::Exp::new(flow.density * flow.v_in_mean).unwrap();
            *next_spawn = rand::Rng::sample(rng, distr);
        }

        // right-turn vehicle
        for i in 0..self.settings.rt_veh_flows.len() {
            let flow = &self.settings.rt_veh_flows[i];
            let signal = self.veh_signals.get(&[flow.src_dir, flow.dst_dir]).unwrap();

            if signal == &VehSignalState::Red {
                continue;
            }

            let id = format!("rt_veh_{i}");
            let next_spawn = self.next_spawns.entry(id).or_insert(0.0);
            *next_spawn -= delta_secs;
            if *next_spawn > 0.0 {
                continue;
            }

            if let Some(output) = compute::compute_rt_veh(&self.settings, flow) {
                self.trajectories.push((output.trajectory_series, 0));
            }
            let distr = rand_distr::Exp::new(flow.density * flow.v_in_mean).unwrap();
            *next_spawn = rand::Rng::sample(rng, distr);
        }

        // pedestrian
        for i in 0..self.settings.ped_flows.len() {
            let flow = &self.settings.ped_flows[i];
            let signal0 = self.ped_signals.get(&[flow.src, flow.dst]);
            let signal1 = self.ped_signals.get(&[flow.dst, flow.src]);
            let signal = Option::or(signal0, signal1).unwrap();

            if signal != &PedSignalState::Green {
                continue;
            }

            let id = format!("ped_{i}");
            let next_spawn = self.next_spawns.entry(id).or_insert(0.0);
            *next_spawn -= delta_secs;
            if *next_spawn > 0.0 {
                continue;
            }

            if let Some(output) = compute::compute_ped(&self.settings, flow) {
                self.trajectories.push((output.trajectory_series, 0));
            }
            let distr = rand_distr::Exp::new(flow.density * flow.v_in_mean).unwrap();
            *next_spawn = rand::Rng::sample(rng, distr);
        }

        // inter-green pedestrian
        for i in 0..self.settings.ig_ped_flows.len() {
            let flow = &self.settings.ped_flows[i];
            let signal0 = self.ped_signals.get(&[flow.src, flow.dst]);
            let signal1 = self.ped_signals.get(&[flow.dst, flow.src]);
            let signal = Option::or(signal0, signal1).unwrap();

            if signal != &PedSignalState::Blink {
                continue;
            }

            let id = format!("ig_ped_{i}");
            let next_spawn = self.next_spawns.entry(id).or_insert(0.0);
            *next_spawn -= delta_secs;
            if *next_spawn > 0.0 {
                continue;
            }

            if let Some(output) = compute::compute_ig_ped(&self.settings, flow) {
                self.trajectories.push((output.trajectory_series, 0));
            }
            let distr = rand_distr::Exp::new(flow.density * flow.v_in_mean).unwrap();
            *next_spawn = rand::Rng::sample(rng, distr);
        }

        let mut remove_stack = vec![];
        for i in 0..self.trajectories.len() {
            let (trajectory, step) = &mut self.trajectories[i];

            *step += delta_secs.div_euclid(compute::STEP) as usize;

            if *step >= trajectory.len() {
                remove_stack.push(i);
            }
        }
        while let Some(i) = remove_stack.pop() {
            self.trajectories.swap_remove(i);
        }

        self.elapsed_time += delta_secs;
    }

    pub fn show_simulation_inside(&mut self, ui: &mut egui::Ui) {
        let mut points = vec![];
        for (trajectory, step) in &self.trajectories {
            let point = egui_plot::Points::new(trajectory[*step])
                .color(egui::Color32::RED)
                .radius(2.0);
            points.push(point);
        }

        self.settings.show_simulation_inside(ui, |plot_ui| {
            points.into_iter().for_each(|v| plot_ui.points(v));
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
