mod gui;

use gui::MyApp;

fn main() {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    // Handle the Result properly
    if let Err(e) = eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    ) {
        eprintln!("Error: {}", e);
    }
}