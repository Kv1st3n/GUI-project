mod gui;

use gui::MyApp;

fn main() {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        "To do list",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    ) {
        eprintln!("Error: {}", e);
    }
}