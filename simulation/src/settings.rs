#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir {
    NxPy,
    NxNy,
    PxPy,
    PxNy,
    NyNx,
    NyPx,
    PyNx,
    PyPx,
}

#[derive(Clone, PartialEq, Debug)]
pub struct VehFlow {
    pub src_dir: Dir,
    pub src_lane: usize,
    pub dst_dir: Dir,
    pub dst_lane: usize,
    pub density: f64,
    pub v_in_mean: f64,
    pub v_in_stdv: f64,
    pub v_out_mean: f64,
    pub v_out_stdv: f64,
    pub padding_out_mean: f64,
    pub padding_out_stdv: f64,
    pub large_prob: f64,
}

impl Default for VehFlow {
    fn default() -> Self {
        Self {
            src_dir: Dir::PxNy,
            src_lane: 0,
            dst_dir: Dir::NxNy,
            dst_lane: 0,
            density: 0.1,
            v_in_mean: 10.0,
            v_in_stdv: 1.0,
            v_out_mean: 10.0,
            v_out_stdv: 1.0,
            padding_out_mean: 1.75,
            padding_out_stdv: 0.1,
            large_prob: 0.1,
        }
    }
}

impl VehFlow {
    fn show_inside(&mut self, ui: &mut egui::Ui, id_source: &str) {
        ui.collapsing(format!("Vehicle flow {id_source}"), |ui| {
            egui::ComboBox::from_label(format!("Source direction {id_source}"))
                .selected_text(format!("{:?}", self.src_dir))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.src_dir, Dir::NxPy, "-X+Y");
                    ui.selectable_value(&mut self.src_dir, Dir::NxNy, "-X-Y");
                    ui.selectable_value(&mut self.src_dir, Dir::PxPy, "+X+Y");
                    ui.selectable_value(&mut self.src_dir, Dir::PxNy, "+X-Y");
                    ui.selectable_value(&mut self.src_dir, Dir::NyNx, "-Y-X");
                    ui.selectable_value(&mut self.src_dir, Dir::NyPx, "-Y+X");
                    ui.selectable_value(&mut self.src_dir, Dir::PyNx, "+Y-X");
                    ui.selectable_value(&mut self.src_dir, Dir::PyPx, "+Y+X");
                });
            ui.label("Source lane");
            ui.add(egui::DragValue::new(&mut self.src_lane));

            egui::ComboBox::from_label(format!("Destination direction {id_source}"))
                .selected_text(format!("{:?}", self.dst_dir))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.dst_dir, Dir::NxPy, "-X+Y");
                    ui.selectable_value(&mut self.dst_dir, Dir::NxNy, "-X-Y");
                    ui.selectable_value(&mut self.dst_dir, Dir::PxPy, "+X+Y");
                    ui.selectable_value(&mut self.dst_dir, Dir::PxNy, "+X-Y");
                    ui.selectable_value(&mut self.dst_dir, Dir::NyNx, "-Y-X");
                    ui.selectable_value(&mut self.dst_dir, Dir::NyPx, "-Y+X");
                    ui.selectable_value(&mut self.dst_dir, Dir::PyNx, "+Y-X");
                    ui.selectable_value(&mut self.dst_dir, Dir::PyPx, "+Y+X");
                });
            ui.label("Destination lane");
            ui.add(egui::DragValue::new(&mut self.dst_lane));

            let widget = egui::Slider::new(&mut self.density, 0.0..=0.1).text("Density [veh/m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.v_in_mean, 0.0..=10.0)
                .text("Inflow velocity mean [m/s]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.v_in_stdv, 0.0..=10.0)
                .text("Inflow velocity stdv [m/s]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.v_out_mean, 0.0..=10.0)
                .text("Outflow velocity mean [m/s]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.v_out_stdv, 0.0..=10.0)
                .text("Outflow velocity stdv [m/s]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.padding_out_mean, 0.0..=10.0)
                .text("Outflow padding mean [m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.padding_out_stdv, 0.0..=10.0)
                .text("Outflow padding stdv [m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.large_prob, 0.0..=1.0)
                .text("Large vehicle probability");
            ui.add(widget);
        });
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PedFlow {
    pub src: Dir,
    pub dst: Dir,
    pub density: f64,
    pub v_in_mean: f64,
    pub v_in_stdv: f64,
    pub x_in_mean: f64,
    pub x_in_stdv: f64,
    pub d_in_mean: f64,
    pub d_in_stdv: f64,
    pub diagonal_prob: f64,
}

impl Default for PedFlow {
    fn default() -> Self {
        Self {
            src: Dir::NxPy,
            dst: Dir::NxNy,
            density: 0.1,
            v_in_mean: 1.0,
            v_in_stdv: 0.1,
            x_in_mean: 0.0,
            x_in_stdv: 0.1,
            d_in_mean: 0.0,
            d_in_stdv: 0.1,
            diagonal_prob: 0.1,
        }
    }
}

impl PedFlow {
    fn show_inside(&mut self, ui: &mut egui::Ui, id_source: &str) {
        ui.collapsing(format!("Pedestrian flow {id_source}"), |ui| {
            egui::ComboBox::from_label(format!("Source direction {id_source}"))
                .selected_text(format!("{:?}", self.src))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.src, Dir::NxPy, "-X+Y");
                    ui.selectable_value(&mut self.src, Dir::NxNy, "-X-Y");
                    ui.selectable_value(&mut self.src, Dir::PxPy, "+X+Y");
                    ui.selectable_value(&mut self.src, Dir::PxNy, "+X-Y");
                    ui.selectable_value(&mut self.src, Dir::NyNx, "-Y-X");
                    ui.selectable_value(&mut self.src, Dir::NyPx, "-Y+X");
                    ui.selectable_value(&mut self.src, Dir::PyNx, "+Y-X");
                    ui.selectable_value(&mut self.src, Dir::PyPx, "+Y+X");
                });

            egui::ComboBox::from_label(format!("Destination direction {id_source}"))
                .selected_text(format!("{:?}", self.src))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.dst, Dir::NxPy, "-X+Y");
                    ui.selectable_value(&mut self.dst, Dir::NxNy, "-X-Y");
                    ui.selectable_value(&mut self.dst, Dir::PxPy, "+X+Y");
                    ui.selectable_value(&mut self.dst, Dir::PxNy, "+X-Y");
                    ui.selectable_value(&mut self.dst, Dir::NyNx, "-Y-X");
                    ui.selectable_value(&mut self.dst, Dir::NyPx, "-Y+X");
                    ui.selectable_value(&mut self.dst, Dir::PyNx, "+Y-X");
                    ui.selectable_value(&mut self.dst, Dir::PyPx, "+Y+X");
                });

            let widget = egui::Slider::new(&mut self.density, 0.0..=0.1).text("Density [ped/m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.v_in_mean, 0.0..=10.0)
                .text("Inflow velocity mean [m/s]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.v_in_stdv, 0.0..=10.0)
                .text("Inflow velocity stdv [m/s]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.x_in_mean, 0.0..=10.0).text("Inflow position mean [m]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.x_in_stdv, 0.0..=10.0).text("Inflow position stdv [m]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.d_in_mean, 0.0..=10.0).text("Inflow distance mean [m]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.d_in_stdv, 0.0..=10.0).text("Inflow distance stdv [m]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.diagonal_prob, 0.0..=1.0).text("Diagonal probability");
            ui.add(widget);
        });
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct VehSignal {
    pub src_dir: Dir,
    pub dst_dir: Dir,
    pub offset_secs: f64,
    pub green_secs: f64,
    pub yellow_secs: f64,
    pub red_secs: f64,
}

impl Default for VehSignal {
    fn default() -> Self {
        Self {
            src_dir: Dir::NxPy,
            dst_dir: Dir::PxPy,
            offset_secs: 0.0,
            green_secs: 110.0,
            yellow_secs: 5.0,
            red_secs: 125.0,
        }
    }
}

impl VehSignal {
    fn show_inside(&mut self, ui: &mut egui::Ui, id_source: &str) {
        ui.collapsing(format!("Vehicle signal {id_source}"), |ui| {
            egui::ComboBox::from_label(format!("Source direction {id_source}"))
                .selected_text(format!("{:?}", self.src_dir))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.src_dir, Dir::NxPy, "-X+Y");
                    ui.selectable_value(&mut self.src_dir, Dir::NxNy, "-X-Y");
                    ui.selectable_value(&mut self.src_dir, Dir::PxPy, "+X+Y");
                    ui.selectable_value(&mut self.src_dir, Dir::PxNy, "+X-Y");
                    ui.selectable_value(&mut self.src_dir, Dir::NyNx, "-Y-X");
                    ui.selectable_value(&mut self.src_dir, Dir::NyPx, "-Y+X");
                    ui.selectable_value(&mut self.src_dir, Dir::PyNx, "+Y-X");
                    ui.selectable_value(&mut self.src_dir, Dir::PyPx, "+Y+X");
                });

            egui::ComboBox::from_label(format!("Destination direction {id_source}"))
                .selected_text(format!("{:?}", self.dst_dir))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.dst_dir, Dir::NxPy, "-X+Y");
                    ui.selectable_value(&mut self.dst_dir, Dir::NxNy, "-X-Y");
                    ui.selectable_value(&mut self.dst_dir, Dir::PxPy, "+X+Y");
                    ui.selectable_value(&mut self.dst_dir, Dir::PxNy, "+X-Y");
                    ui.selectable_value(&mut self.dst_dir, Dir::NyNx, "-Y-X");
                    ui.selectable_value(&mut self.dst_dir, Dir::NyPx, "-Y+X");
                    ui.selectable_value(&mut self.dst_dir, Dir::PyNx, "+Y-X");
                    ui.selectable_value(&mut self.dst_dir, Dir::PyPx, "+Y+X");
                });

            let widget =
                egui::Slider::new(&mut self.offset_secs, 0.0..=600.0).text("Offset time[sec]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.green_secs, 0.0..=600.0).text("Green time[sec]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.yellow_secs, 0.0..=600.0).text("Yellow time[sec]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.red_secs, 0.0..=600.0).text("Red time[sec]");
            ui.add(widget);
        });
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PedSignal {
    pub src_dir: Dir,
    pub dst_dir: Dir,
    pub offset_secs: f64,
    pub green_secs: f64,
    pub blink_secs: f64,
    pub red_secs: f64,
}

impl Default for PedSignal {
    fn default() -> Self {
        Self {
            src_dir: Dir::NxNy,
            dst_dir: Dir::NxPy,
            offset_secs: 120.0,
            green_secs: 100.0,
            blink_secs: 10.0,
            red_secs: 130.0,
        }
    }
}

impl PedSignal {
    fn show_inside(&mut self, ui: &mut egui::Ui, id_source: &str) {
        ui.collapsing(format!("Pedestrian signal {id_source}"), |ui| {
            egui::ComboBox::from_label(format!("Source direction {id_source}"))
                .selected_text(format!("{:?}", self.src_dir))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.src_dir, Dir::NxPy, "-X+Y");
                    ui.selectable_value(&mut self.src_dir, Dir::NxNy, "-X-Y");
                    ui.selectable_value(&mut self.src_dir, Dir::PxPy, "+X+Y");
                    ui.selectable_value(&mut self.src_dir, Dir::PxNy, "+X-Y");
                    ui.selectable_value(&mut self.src_dir, Dir::NyNx, "-Y-X");
                    ui.selectable_value(&mut self.src_dir, Dir::NyPx, "-Y+X");
                    ui.selectable_value(&mut self.src_dir, Dir::PyNx, "+Y-X");
                    ui.selectable_value(&mut self.src_dir, Dir::PyPx, "+Y+X");
                });

            egui::ComboBox::from_label(format!("Destination direction {id_source}"))
                .selected_text(format!("{:?}", self.dst_dir))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.dst_dir, Dir::NxPy, "-X+Y");
                    ui.selectable_value(&mut self.dst_dir, Dir::NxNy, "-X-Y");
                    ui.selectable_value(&mut self.dst_dir, Dir::PxPy, "+X+Y");
                    ui.selectable_value(&mut self.dst_dir, Dir::PxNy, "+X-Y");
                    ui.selectable_value(&mut self.dst_dir, Dir::NyNx, "-Y-X");
                    ui.selectable_value(&mut self.dst_dir, Dir::NyPx, "-Y+X");
                    ui.selectable_value(&mut self.dst_dir, Dir::PyNx, "+Y-X");
                    ui.selectable_value(&mut self.dst_dir, Dir::PyPx, "+Y+X");
                });

            let widget =
                egui::Slider::new(&mut self.offset_secs, 0.0..=600.0).text("Offset time[sec]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.green_secs, 0.0..=600.0).text("Green time[sec]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.blink_secs, 0.0..=600.0).text("Blink time[sec]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.red_secs, 0.0..=600.0).text("Red time[sec]");
            ui.add(widget);
        });
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Settings {
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
    pub lt_veh_flows: Vec<VehFlow>,
    pub rt_veh_flows: Vec<VehFlow>,
    pub ped_flows: Vec<PedFlow>,
    pub ig_ped_flows: Vec<PedFlow>,
    pub veh_signals: Vec<VehSignal>,
    pub ped_signals: Vec<PedSignal>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
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
            lt_veh_flows: vec![
                VehFlow {
                    src_dir: Dir::NxPy,
                    src_lane: 3,
                    dst_dir: Dir::PyNx,
                    dst_lane: 3,
                    ..Default::default()
                },
                VehFlow {
                    src_dir: Dir::PyPx,
                    src_lane: 0,
                    dst_dir: Dir::PxPy,
                    dst_lane: 3,
                    ..Default::default()
                },
                VehFlow {
                    src_dir: Dir::PxNy,
                    src_lane: 0,
                    dst_dir: Dir::NyPx,
                    dst_lane: 0,
                    ..Default::default()
                },
                VehFlow {
                    src_dir: Dir::NyNx,
                    src_lane: 3,
                    dst_dir: Dir::NxNy,
                    dst_lane: 0,
                    ..Default::default()
                },
            ],
            rt_veh_flows: vec![
                VehFlow {
                    src_dir: Dir::NxPy,
                    src_lane: 2,
                    dst_dir: Dir::NyPx,
                    dst_lane: 1,
                    ..Default::default()
                },
                VehFlow {
                    src_dir: Dir::NyNx,
                    src_lane: 2,
                    dst_dir: Dir::PxPy,
                    dst_lane: 2,
                    ..Default::default()
                },
                VehFlow {
                    src_dir: Dir::PxNy,
                    src_lane: 1,
                    dst_dir: Dir::PyNx,
                    dst_lane: 2,
                    ..Default::default()
                },
                VehFlow {
                    src_dir: Dir::PyPx,
                    src_lane: 1,
                    dst_dir: Dir::NxNy,
                    dst_lane: 1,
                    ..Default::default()
                },
            ],
            ped_flows: vec![
                PedFlow {
                    src: Dir::NxPy,
                    dst: Dir::NxNy,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::PyNx,
                    dst: Dir::PyPx,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::PxPy,
                    dst: Dir::PxNy,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::NyNx,
                    dst: Dir::NyPx,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::NxNy,
                    dst: Dir::NxPy,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::PyPx,
                    dst: Dir::PyNx,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::PxNy,
                    dst: Dir::PxPy,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::NyPx,
                    dst: Dir::NyNx,
                    ..Default::default()
                },
            ],
            ig_ped_flows: vec![
                PedFlow {
                    src: Dir::NxPy,
                    dst: Dir::NxNy,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::PyNx,
                    dst: Dir::PyPx,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::PxPy,
                    dst: Dir::PxNy,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::NyNx,
                    dst: Dir::NyPx,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::NxNy,
                    dst: Dir::NxPy,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::PyPx,
                    dst: Dir::PyNx,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::PxNy,
                    dst: Dir::PxPy,
                    ..Default::default()
                },
                PedFlow {
                    src: Dir::NyPx,
                    dst: Dir::NyNx,
                    ..Default::default()
                },
            ],
            veh_signals: vec![
                VehSignal {
                    src_dir: Dir::NxPy,
                    dst_dir: Dir::PxPy,
                    offset_secs: 0.0,
                    ..Default::default()
                },
                VehSignal {
                    src_dir: Dir::PxNy,
                    dst_dir: Dir::NxNy,
                    offset_secs: 0.0,
                    ..Default::default()
                },
                VehSignal {
                    src_dir: Dir::NyPx,
                    dst_dir: Dir::PyPx,
                    offset_secs: 120.0,
                    ..Default::default()
                },
                VehSignal {
                    src_dir: Dir::PyNx,
                    dst_dir: Dir::NyNx,
                    offset_secs: 120.0,
                    ..Default::default()
                },
            ],
            ped_signals: vec![
                PedSignal {
                    src_dir: Dir::NxNy,
                    dst_dir: Dir::NxPy,
                    offset_secs: 120.0,
                    ..Default::default()
                },
                PedSignal {
                    src_dir: Dir::PxNy,
                    dst_dir: Dir::PxPy,
                    offset_secs: 120.0,
                    ..Default::default()
                },
                PedSignal {
                    src_dir: Dir::NyPx,
                    dst_dir: Dir::NyNx,
                    offset_secs: 0.0,
                    ..Default::default()
                },
                PedSignal {
                    src_dir: Dir::PyPx,
                    dst_dir: Dir::PyNx,
                    offset_secs: 0.0,
                    ..Default::default()
                },
            ],
        }
    }
}

impl Settings {
    pub fn show_settings_inside(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let widget =
                egui::Slider::new(&mut self.angle, 0.0..=180.0).text("Intersection angle[deg]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.radius, 0.0..=30.0).text("Border radius[m]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.width_along, 0.0..=30.0).text("Along road width[m]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.width_across, 0.0..=30.0).text("Across road width[m]");
            ui.add(widget);

            // along lane
            ui.horizontal(|ui| {
                ui.label("Along lane");
                if ui.button("Add").clicked() {
                    self.lane_along.push(Default::default());
                }
                if ui.button("Remove").clicked() {
                    self.lane_along.pop();
                }
            });
            for lane in &mut self.lane_along {
                ui.horizontal(|ui| {
                    let widget = egui::Slider::new(lane, -30.0..=30.0).text("Lane shift[m]");
                    ui.add(widget);
                });
            }

            // across lane
            ui.horizontal(|ui| {
                ui.label("Across lane");
                if ui.button("Add").clicked() {
                    self.lane_across.push(Default::default());
                }
                if ui.button("Remove").clicked() {
                    self.lane_across.pop();
                }
            });
            for lane in &mut self.lane_across {
                ui.horizontal(|ui| {
                    let widget = egui::Slider::new(lane, -30.0..=30.0).text("Lane shift[m]");
                    ui.add(widget);
                });
            }

            let widget =
                egui::Slider::new(&mut self.hn_along, 0.0..=30.0).text("Along hard nose[m]");
            ui.add(widget);

            let widget =
                egui::Slider::new(&mut self.hn_across, 0.0..=30.0).text("Across hard nose[m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.cw_setback_along, 0.0..=30.0)
                .text("Along crosswalk setback[m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.cw_setback_across, 0.0..=30.0)
                .text("Across crosswalk setback[m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.cw_width_along, 0.0..=30.0)
                .text("Along crosswalk width[m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.cw_width_across, 0.0..=30.0)
                .text("Across crosswalk width[m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.sl_setback_along, 0.0..=30.0)
                .text("Along stop-line setback[m]");
            ui.add(widget);

            let widget = egui::Slider::new(&mut self.sl_setback_across, 0.0..=30.0)
                .text("Across stop-line setback[m]");
            ui.add(widget);

            // lt_veh_flows
            ui.horizontal(|ui| {
                ui.label("Left-turned vehicle flows");
                if ui.button("Add").clicked() {
                    self.lt_veh_flows.push(Default::default());
                }
                if ui.button("Remove").clicked() {
                    self.lt_veh_flows.pop();
                }
            });
            for (i, flow) in self.lt_veh_flows.iter_mut().enumerate() {
                flow.show_inside(ui, format!("lt_veh_flow_{i}").as_str());
            }

            // rt_veh_flows
            ui.horizontal(|ui| {
                ui.label("Right-turned vehicle flows");
                if ui.button("Add").clicked() {
                    self.rt_veh_flows.push(Default::default());
                }
                if ui.button("Remove").clicked() {
                    self.rt_veh_flows.pop();
                }
            });
            for (i, flow) in self.rt_veh_flows.iter_mut().enumerate() {
                flow.show_inside(ui, format!("rt_veh_flow_{i}").as_str());
            }

            // ped_flows
            ui.horizontal(|ui| {
                ui.label("Pedestrian flows");
                if ui.button("Add").clicked() {
                    self.ped_flows.push(Default::default());
                }
                if ui.button("Remove").clicked() {
                    self.ped_flows.pop();
                }
            });
            for (i, flow) in self.ped_flows.iter_mut().enumerate() {
                flow.show_inside(ui, format!("ped_flow_{i}").as_str());
            }

            // ig_ped_flows
            ui.horizontal(|ui| {
                ui.label("Inter-green pedestrian flows");
                if ui.button("Add").clicked() {
                    self.ig_ped_flows.push(Default::default());
                }
                if ui.button("Remove").clicked() {
                    self.ig_ped_flows.pop();
                }
            });
            for (i, flow) in self.ig_ped_flows.iter_mut().enumerate() {
                flow.show_inside(ui, format!("ig_ped_flow_{i}").as_str());
            }

            // veh_signals
            ui.horizontal(|ui| {
                ui.label("Vehicle signals");
                if ui.button("Add").clicked() {
                    self.veh_signals.push(Default::default());
                }
                if ui.button("Remove").clicked() {
                    self.veh_signals.pop();
                }
            });
            for (i, signal) in self.veh_signals.iter_mut().enumerate() {
                signal.show_inside(ui, format!("veh_signal_{i}").as_str());
            }

            // ped_signals
            ui.horizontal(|ui| {
                ui.label("Pedestrian signals");
                if ui.button("Add").clicked() {
                    self.ped_signals.push(Default::default());
                }
                if ui.button("Remove").clicked() {
                    self.ped_signals.pop();
                }
            });
            for (i, signal) in self.ped_signals.iter_mut().enumerate() {
                signal.show_inside(ui, format!("ped_signal_{i}").as_str());
            }
        });
    }
}
