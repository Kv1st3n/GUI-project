mod gui;

use eframe::NativeOptions;
use gui::MyApp;

fn main() {
    let options = NativeOptions {

        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    );
}
