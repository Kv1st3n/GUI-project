use eframe::egui;

pub struct MyApp {
    label: String,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            label: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to your to-do list!");

            // Adds a text input
            ui.horizontal(|ui| {
                ui.label("Add thing to do: ");

                ui.text_edit_singleline(&mut self.label);
            });
        });
    }
}
