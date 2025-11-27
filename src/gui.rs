use eframe::egui;

pub struct MyApp {
    pub label: String,
    pub value: f32,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            label: String::new(),
            value: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("To do list");
        });
    }
}
