mod gui;

use eframe::NativeOptions;
use gui::MyApp;

fn main() {
    // Set up NativeOptions with software (Glow) renderer
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow, // Software fallback
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    );
}
