use gtk::prelude::*;
use std::env;
use std::process::Command;
use gtk::{glib, DrawingArea, ApplicationWindow, Button, CssProvider};
use gtk::gdk::Display;
use cairo_rs::Context;

const APP_ID: &str = "rsdutil";

fn init() -> Result<(), Box<dyn std::error::Error>>{
    // Add windowrule if on hyprland
    let windowrule = "float,title:^(rsdutil)$";
    if let Ok(desktop_env) = env::var("XDG_CURRENT_DESKTOP") {
        if desktop_env == "Hyprland" {
            let output = Command::new("hyprctl")
                .arg("keyword")
                .arg("windowrulev2")
                .arg(windowrule)
                .output()
                .expect("Failed to execute command");
            if output.status.success() {
                println!("Window rule set successfully");
            } else {
                println!(
                    "Failed to set window rule: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        } else {
            println!("Not running on Hyprland");
        }
    } else {
        println!("XDG_CURRENT_DESKTOP environment variable not set");
    }
    Ok(())
}
fn main() -> glib::ExitCode {
    let _ = init();
    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();
    // Connect to "activate" signal of `app`
    app.connect_startup(|_|{
        load_css();
    });
    app.connect_activate(|app| {
        build_ui();
    });
    // Run the application
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("./gui/gui.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &adw::Application){
    let button = Button::builder()
        .label("Click me!")
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();
    
    let drawing_area = DrawingArea::new();
    drawing_area.set_size_request(200, 200);
    drawing_area.connect_draw(move |_, cr| {
        pbar(&cr, 0.5);
    });
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("rsdutil")
        .child(&button)
        .build();
    // Present window
    window.present();
}

fn pbar(cr: &Context, progress: f64) {
    let width: f64 = 100.0;
    let height: f64 = 100.0;
    let line_width = 20.0;

    cr.set_source_rgb(1.0, 1.0, 1.0); // Background color
    cr.paint();

    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let radius = (width.min(height) / 2.0) - line_width;

    // Draw the background circle
    cr.set_source_rgb(0.8, 0.8, 0.8);
    cr.set_line_width(line_width);
    cr.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI);
    cr.stroke();

    // Draw the progress circle
    cr.set_source_rgb(0.0, 0.6, 0.0); // Progress color
    cr.arc(
        center_x,
        center_y,
        radius,
        -std::f64::consts::PI / 2.0,
        -std::f64::consts::PI / 2.0 + 2.0 * std::f64::consts::PI * progress,
    );
    cr.stroke(); 
}
