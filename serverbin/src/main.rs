use serverlib::config::init_config;

use ui::app::App;
fn main() {
    if let Err(e) = init_config() {
        eprintln!("fatal: {e}");
        std::process::exit(1);
    }
    dioxus::launch(App);
}
