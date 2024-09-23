mod forward;
mod plot;
mod settings;
mod widget;

use egui_miniquad as egui_mq;
use miniquad as mq;

struct State {
    egui_mq: egui_miniquad::EguiMq,
    mq_ctx: Box<dyn mq::RenderingBackend>,
    widget: widget::Widget,
    _thread_handle: std::thread::JoinHandle<()>,
}

impl State {
    fn new() -> Self {
        let mut mq_ctx = mq::window::new_rendering_backend();
        let mut widget = widget::Widget::new();
        Self {
            egui_mq: egui_mq::EguiMq::new(&mut *mq_ctx),
            mq_ctx,
            _thread_handle: widget.spawn_simulation(),
            widget,
        }
    }
}

impl mq::EventHandler for State {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.mq_ctx
            .begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        self.mq_ctx.end_render_pass();

        self.egui_mq.run(&mut *self.mq_ctx, |_mq_ctx, egui_ctx| {
            self.widget.show(egui_ctx);
        });

        // Draw things behind egui here

        self.egui_mq.draw(&mut *self.mq_ctx);

        // Draw things in front of egui here

        self.mq_ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        #[cfg(target_os = "windows")]
        let (dx, dy) = (dx / 120.0, dy / 120.0);

        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_down_event(mb, x, y);
    }

    fn mouse_button_up_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_up_event(mb, x, y);
    }

    fn char_event(&mut self, character: char, _keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.key_down_event(keycode, keymods);
    }

    fn key_up_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}

fn main() {
    fern::Dispatch::default()
        .format(|o, msg, record| {
            o.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                msg
            ));
        })
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let conf = mq::conf::Conf {
        window_title: "safety-traffic-simulation".into(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        icon: None,
        ..Default::default()
    };

    mq::start(conf, || Box::new(State::new()));
}
