pub mod engine;

pub fn hello_world() {
    let settings = engine::Settings::default();
    engine::run_app("test", settings, Box::new(|_ctx| Box::new(HelloWorld::default())))
}

#[derive(Default)]
struct HelloWorld {}

impl engine::Application for HelloWorld {
    fn update(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hello world")
        });
    }
}