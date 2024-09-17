mod math;

mod field;
mod ig_ped;
mod lt_veh;
mod ped;
mod rt_veh;
mod simulator;

use eframe::egui;

use field::*;
use ig_ped::*;
use lt_veh::*;
use ped::*;
use rt_veh::*;
use simulator::*;

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
    state: AppState,
    field: FieldComponent,
    lt_veh: LtVehComponent,
    rt_veh: RtVehComponent,
    ped: PedComponent,
    ig_ped: IgPedComponent,
}

#[derive(Debug, Clone, Default)]
enum AppState {
    #[default]
    None,
    LtVeh,
    RtVeh,
    Ped,
    IgPed,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Field").clicked() {
                    self.state = AppState::None;
                }
                if ui.button("Left-turned Vehicle").clicked() {
                    self.state = AppState::LtVeh;
                }
                if ui.button("Right-turnd Vehicle").clicked() {
                    self.state = AppState::RtVeh;
                }
                if ui.button("Pedestrian").clicked() {
                    self.state = AppState::Ped;
                }
                if ui.button("Inter-green Pedestrian").clicked() {
                    self.state = AppState::IgPed;
                }
            });

            egui::ScrollArea::vertical().show(ui, |ui| match self.state {
                AppState::None => {
                    self.field.ui(ui);
                }
                AppState::LtVeh => {
                    self.lt_veh.ui(ui);
                }
                AppState::RtVeh => {
                    self.rt_veh.ui(ui);
                }
                AppState::Ped => {
                    self.ped.ui(ui);
                }
                AppState::IgPed => {
                    self.ig_ped.ui(ui);
                }
            });
        });
    }
}
