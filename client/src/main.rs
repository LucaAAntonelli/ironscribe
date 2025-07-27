use client::newfilesync::BookClient;
use eframe::{CreationContext, egui};
use egui_file_dialog::FileDialog;
use tonic::transport::Channel;

fn main() {
    let mut options = eframe::NativeOptions::default();
    eframe::run_native(
        "HELLOWORLD",
        options,
        Box::new(|cc| Ok(Box::<MyApp>::default())),
    );
}

struct MyApp {
    rt: tokio::runtime::Runtime,
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            rt: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            name: "Hello World".to_string(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Helllo '{}', age {}", self.name, self.age));
            let mut file_dialog = FileDialog::new();
            if ui.button("Select file").clicked() {
                println!("Send gRPC add book request");
                file_dialog.pick_file();
            }
            file_dialog.update(ctx);
            if let Some(path) = file_dialog.take_picked() {
                println!("User selected: {:?}", path);
            }
        });
    }
}
