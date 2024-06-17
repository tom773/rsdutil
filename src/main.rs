use gtk::prelude::*;
use std::env;
use std::collections::HashMap;
use std::process::Command;
use adw::{Application, ApplicationWindow};
use gtk::{glib, DrawingArea, Grid, CssProvider, Overlay};
use gtk::gdk::Display;

const APP_ID: &str = "rsdutil";

// Move to utils
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

// Leave in main.rs
fn main() -> glib::ExitCode {
    let _ = init();
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    // Connect to "activate" signal of `app`
    app.connect_startup(|_|{
        load_css();
    });
    app.connect_activate(|app| {
        build_ui(app);
    });
    // Run the application
    app.run()
}

// Move to utils
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
// Leave in main.rs
fn build_ui(app: &adw::Application){
    
    // Example set of disk space
    let mut disk_space = HashMap::new();
    disk_space.insert("/root", 0.2);
    disk_space.insert("/usr", 0.2);
    disk_space.insert("/var", 0.3);
    disk_space.insert("/home", 0.6);

    let grid = Grid::new();
    grid.set_widget_name("grid");
    grid.set_row_spacing(20);
    grid.set_column_spacing(20);
    grid.set_valign(gtk::Align::Center);
    grid.set_halign(gtk::Align::Center);

    for (i, (label_text, progress)) in disk_space.iter().enumerate() {
        let row = i / 2;
        let col = i % 2;
        let overlay = create_pbar(label_text.to_string(), *progress);
        grid.attach(&overlay, col as i32, row as i32, 1, 1);
    }

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.set_widget_name("pb");
    vbox.append(&grid);
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("rsdutil")
        .content(&vbox)
        .build();
    // Present window
    window.present();
}

// Refactor into utils
fn create_pbar(label_text: String, progress: f64) -> Overlay{
    
    let drawing_area = DrawingArea::new();
    drawing_area.set_widget_name("drawing-area");
    drawing_area.set_size_request(220, 260);
    drawing_area.set_draw_func(move |_, cr, _, _| {
        pbar(cr, progress, &label_text); // Adjust the progress value here
    });


    let overlay = Overlay::new();
    overlay.set_child(Some(&drawing_area));
    overlay.set_valign(gtk::Align::Center);
    overlay.set_halign(gtk::Align::Center);

    return overlay;
    
}

// Refactor into utils
fn pbar(cr: &gtk::cairo::Context, progress: f64, label_text: &str) {
    let width: f64 = 220.0;
    let height: f64 = 220.0;
    let line_width = 40.0;

    cr.set_source_rgba(0.0, 0.0, 0.0, 0.0); // Background color
    let _ = cr.paint();

    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let radius = (width.min(height) / 2.0) - line_width;

    cr.set_source_rgb(0.8, 0.8, 0.8);
    cr.set_line_width(line_width);
    cr.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI);
    let _ = cr.stroke();

    cr.set_source_rgb(0.0, 0.6, 0.0); 
    cr.arc(
        center_x,
        center_y,
        radius,
        -std::f64::consts::PI / 2.0,
        -std::f64::consts::PI / 2.0 + 2.0 * std::f64::consts::PI * progress,
    );
    cr.stroke().unwrap();
    draw_text(cr, label_text, center_x, center_y);

    let text_y = center_y + radius + line_width/2.0 + 20.0;
    let percentage_text = format!("{:.0}%", progress * 100.0);
    // Draw the percentage text
    draw_text(cr, &percentage_text, center_x, text_y);
}

// Refactor into utils
fn draw_text(cr: &gtk::cairo::Context, text: &str, x: f64, y: f64) {
    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.select_font_face("JetBrains Mono", gtk::cairo::FontSlant::Normal, gtk::cairo::FontWeight::Normal);
    cr.set_font_size(20.0);

    let extents = cr.text_extents(text).unwrap();
    cr.move_to(x - extents.width() / 2.0, y + extents.height() / 2.0);
    cr.show_text(text).unwrap();
    cr.stroke().unwrap();
}
