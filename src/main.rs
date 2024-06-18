use gtk::prelude::*;
use adw::Application;
use gtk::glib;

mod utils;
mod gui;

const APP_ID: &str = "rsdutil";


// Leave in main.rs
fn main() -> glib::ExitCode {
    let _ = utils::init::init();
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    let (ds, ts, tf) = utils::utils::disk();
    // Connect to "activate" signal of `app`
    app.connect_startup(|_|{
        utils::init::load_css();
    });
    app.connect_activate(move |app| {
        gui::ui::build_ui(app, &ds, &ts, &tf);
    });
    // Run the application
    app.run()
}
