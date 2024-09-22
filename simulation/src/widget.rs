use crate::settings;

pub struct Widget {
    pub setting: settings::Settings,
}

impl Widget {
    pub fn new() -> Self {
        Self {
            setting: Default::default(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) -> egui::InnerResponse<()> {
        let widget = egui::SidePanel::left("settings")
            .show_separator_line(false)
            .resizable(false);
        widget.show(ctx, |ui| {
            ui.heading("Parametr Settings");

            self.setting.show_settings_inside(ui);
        });

        let widget = egui::TopBottomPanel::bottom("schedule")
            .show_separator_line(false)
            .resizable(false);
        widget.show(ctx, |ui| {
            ui.heading("TLS Schedule Plot");

            self.setting.show_schedule_inside(ui);
        });

        let widget = egui::CentralPanel::default();
        widget.show(ctx, |ui| {
            ui.heading("Simulation Plot");

            self.setting.show_simulation_inside(ui);
        })
    }
}
