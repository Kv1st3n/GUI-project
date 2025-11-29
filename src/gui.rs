use eframe::egui;

pub struct MyApp {
    label: String,
    is_completed: bool,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            label: String::new(),
            is_completed: false,
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

                let color = if self.is_completed {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::RED
                };

                let text_edit = egui::TextEdit::singleline(&mut self.label)
                    .text_color(color);
                ui.add(text_edit);

                ui.checkbox(&mut self.is_completed, "Mark as completed");

                if self.is_completed {
                    ui.colored_label(egui::Color32::GREEN, "To-do completed");
                } else {
                    ui.colored_label(egui::Color32::RED, "To-do not commpleted");
                }

            });
        });
    }
}
