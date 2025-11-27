mod gui;

use eframe::NativeOptions;
use gui::MyApp;

fn main() {
    let options = NativeOptions {
        initial_window_size: Some(eframe::egui::Vec2::new(400.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    );
}
