use eframe::egui;

pub struct MyApp {

}

impl MyApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Add things to do");
        });
    }
}