use eframe::egui;
use std::path::PathBuf;

struct Ed {
    ruta: Option<PathBuf>,
    texto: String,
    aviso: String,
}

impl Ed {
    fn new(ruta: Option<PathBuf>) -> Ed {
        let mut texto = String::new();
        let mut aviso = String::new();

        if let Some(r) = &ruta {
            if !r.exists() {
                aviso = "archivo nuevo".to_string();
            } else if let Ok(contenido) = std::fs::read_to_string(r) {
                texto = contenido;
            } else {
                aviso = "no se pudo abrir el archivo".to_string();
            }
        } else {
            aviso = "sin archivo".to_string();
        }

        Ed { ruta, texto, aviso }
    }
}

impl eframe::App for Ed {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::bottom("estado").show(ui, |ui| {
            ui.label(&self.aviso);
        });

        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let cuadro = egui::TextEdit
                    ::multiline(&mut self.texto)
                    .code_editor()
                    .desired_width(f32::INFINITY);
                ui.add(cuadro);
            });
        });
    }
}

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    let mut ruta = None;
    if args.len() > 1 {
        ruta = Some(PathBuf::from(&args[1]));
    }

    let editor = Ed::new(ruta);

    eframe::run_native(
        "ed",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(editor)))
    )
}
