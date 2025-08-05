use client::app::MyApp;
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "HELLOWORLD",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    );
}
