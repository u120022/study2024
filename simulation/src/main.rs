mod left_turned;
mod right_turned;

use eframe::egui;

use left_turned::*;
use right_turned::*;

fn main() -> eframe::Result {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    env_logger::init();

    eframe::run_native(
        "Simulation",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::<App>::default())),
    )
}

#[derive(Debug, Clone, Default)]
struct App {
    left_turned_component: LeftTurnedComponent,
    right_turned_component: RightTurnedComponent,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // self.left_turned_component.ui(ui);
                self.right_turned_component.ui(ui);
            });
        });
    }
}
