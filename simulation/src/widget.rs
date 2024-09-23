use std::sync::Arc;

use parking_lot::Mutex;

use crate::forward;
use crate::settings;

pub const LOOP_WAIT: f64 = 0.016;
pub const TIME_SCALE: f64 = 10.0;

pub struct Widget {
    pub setting: settings::Settings,
    pub forward: Arc<Mutex<Option<forward::Forward>>>,
}

impl Widget {
    pub fn new() -> Self {
        Self {
            setting: Default::default(),
            forward: Default::default(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) -> egui::InnerResponse<()> {
        let widget = egui::SidePanel::left("settings").resizable(false);
        widget.show(ctx, |ui| {
            ui.heading("Parametr Settings");

            self.setting.show_settings_inside(ui);
        });

        let widget = egui::TopBottomPanel::bottom("schedule").resizable(false);
        widget.show(ctx, |ui| {
            ui.heading("TLS Schedule Plot");

            if let Some(forward) = self.forward.lock().as_mut() {
                forward.show_schedule_inside(ui);
            } else {
                self.setting.show_schedule_inside(ui, |_| {});
            }
        });

        let widget = egui::CentralPanel::default();
        widget.show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Simulation Plot");

                let widget = if self.forward.lock().is_some() {
                    egui::RichText::new("Active").color(egui::Color32::GREEN)
                } else {
                    egui::RichText::new("Inactive").color(egui::Color32::RED)
                };
                ui.label(widget);

                if ui.button("New Simulation").clicked() {
                    let forward = forward::Forward::new(self.setting.clone());
                    *self.forward.lock() = Some(forward);
                }

                if ui.button("Drop Simulation").clicked() {
                    *self.forward.lock() = None;
                }
            });

            if let Some(forward) = self.forward.lock().as_mut() {
                forward.show_simulation_inside(ui);
            } else {
                self.setting.show_simulation_inside(ui, |_| {});
            }
        })
    }

    pub fn spawn_simulation(&mut self) -> std::thread::JoinHandle<()> {
        let forward = self.forward.clone();

        let mut instant = None;
        std::thread::spawn(move || loop {
            'scope: {
                let container = &mut forward.lock();
                let Some(forward) = container.as_mut() else {
                    instant = None;
                    break 'scope;
                };

                let container = std::mem::replace(&mut instant, Some(std::time::Instant::now()));

                let Some(instant) = container.as_ref() else {
                    break 'scope;
                };

                let delta_time = instant.elapsed().as_secs_f64() * TIME_SCALE;
                forward.forward(delta_time);
            }
            std::thread::sleep(std::time::Duration::from_secs_f64(LOOP_WAIT));
        })
    }
}
