use eframe::egui;

struct Ed {}

impl eframe::App for Ed {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("hola desde ed");
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "ed",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Ed {})))
    )
}
