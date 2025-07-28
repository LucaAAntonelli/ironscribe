use std::path::PathBuf;

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
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            rt: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            file_dialog: FileDialog::new(),
            picked_file: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            if ui.button("Select file").clicked() {
                println!("Send gRPC add book request");
                self.file_dialog.pick_file();
            }
            self.file_dialog.update(ctx);
            if let Some(path) = self.file_dialog.take_picked() {
                self.picked_file = Some(path);
                println!("User selected: {:?}", self.picked_file);
            }
        });
    }
}
