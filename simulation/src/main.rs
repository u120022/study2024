mod math;

mod field;
mod ig_ped;
mod lt_veh;
mod ped;
mod rt_veh;
mod sim;

use std::sync::{Arc, Mutex};

use eframe::egui;

use field::*;
use ig_ped::*;
use lt_veh::*;
use ped::*;
use rt_veh::*;
use sim::*;

fn main() -> eframe::Result {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    env_logger::init();

    eframe::run_native(
        "Simulation",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(App::new()))),
    )
}

#[derive(Debug, Clone, Default)]
enum AppState {
    #[default]
    Sim,
    Field,
    LtVeh,
    RtVeh,
    Ped,
    IgPed,
}

struct App {
    state: AppState,
    field: FieldComponent,
    lt_veh: LtVehComponent,
    rt_veh: RtVehComponent,
    ped: PedComponent,
    ig_ped: IgPedComponent,
    sim: Arc<Mutex<SimComponent>>,
    _thread: Option<std::thread::JoinHandle<()>>,
}

impl App {
    fn new() -> Self {
        let mut slf = Self {
            state: Default::default(),
            field: Default::default(),
            lt_veh: Default::default(),
            rt_veh: Default::default(),
            ped: Default::default(),
            ig_ped: Default::default(),
            sim: Default::default(),
            _thread: Default::default(),
        };

        let sim = slf.sim.clone();
        let mut instant = std::time::Instant::now();

        let thread = std::thread::spawn(move || loop {
            {
                let sim = &mut sim.lock().unwrap();
                let pre = std::mem::replace(&mut instant, std::time::Instant::now());
                sim.forward(pre.elapsed().as_secs_f64());
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        });
        slf._thread = Some(thread);

        slf
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Simulation").clicked() {
                    self.state = AppState::Sim;
                }
                if ui.button("Field").clicked() {
                    self.state = AppState::Field;
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
                AppState::Sim => {
                    let comp = &mut self.sim.lock().unwrap();
                    comp.ui(ui);
                }
                AppState::Field => {
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
