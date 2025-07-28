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
    grpc_client: BookClient<Channel>,
}

impl Default for MyApp {
    fn default() -> Self {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to build tokio runtime!");
        let grpc_client = rt
            .block_on(BookClient::new("[::1]", 50051, None, None, None))
            .expect("Failed to create gRPC client!");
        Self {
            rt,
            grpc_client,
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
                self.file_dialog.pick_file();
            }
            self.file_dialog.update(ctx);
            if let Some(path) = self.file_dialog.take_picked() {
                self.picked_file = Some(path.clone());
                println!("User selected: {:?}", self.picked_file);
                let cloned_path = path.clone();
                // TODO: wrap grpc_client in Arc<tokio::sync::Mutex<_>>
                self.rt.spawn(async move {
                    self.grpc_client.add_book(
                        cloned_path.to_str().unwrap().to_owned(),
                        "C:\\Users\\lucaa\\Projects\\ironscribe\\TESTING".into(),
                    )
                });
            }
        });
    }
}
