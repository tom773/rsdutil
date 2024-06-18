use gtk::prelude::*;
use std::collections::HashMap;
use adw::ApplicationWindow;
use gtk::{DrawingArea, Grid, Overlay};

pub fn build_ui(app: &adw::Application, ds: &HashMap<&str, f64>, ts: &f64, tf: &f64){
    
    // Example set of disk space

    let grid = Grid::new();
    grid.set_widget_name("grid");
    grid.set_row_spacing(20);
    grid.set_column_spacing(20);
    grid.set_valign(gtk::Align::Center);
    grid.set_halign(gtk::Align::Center);

    for (i, (label_text, progress)) in ds.iter().enumerate() {
        let row = i / 2;
        let col = i % 2;
        let overlay = create_pbar(label_text.to_string(), *progress, *progress/ *tf);
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

pub fn create_pbar(label_text: String, used: f64, progress: f64) -> Overlay{
    
    let drawing_area = DrawingArea::new();
    drawing_area.set_widget_name("drawing-area");
    drawing_area.set_size_request(220, 260);
    drawing_area.set_draw_func(move |_, cr, _, _| {
        pbar(cr, progress, &label_text, &used); // Adjust the progress value here
    });


    let overlay = Overlay::new();
    overlay.set_child(Some(&drawing_area));
    overlay.set_valign(gtk::Align::Center);
    overlay.set_halign(gtk::Align::Center);

    return overlay;
    
}

// Refactor into utils
pub fn pbar(cr: &gtk::cairo::Context, progress: f64, label_text: &str, used: &f64) {
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
    let percentage_text = format!("{:.2}GB", used);
    // Draw the percentage text
    draw_text(cr, &percentage_text, center_x, text_y);
}

fn draw_text(cr: &gtk::cairo::Context, text: &str, x: f64, y: f64) {
    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.select_font_face("JetBrains Mono", gtk::cairo::FontSlant::Normal, gtk::cairo::FontWeight::Normal);
    cr.set_font_size(20.0);

    let extents = cr.text_extents(text).unwrap();
    cr.move_to(x - extents.width() / 2.0, y + extents.height() / 2.0);
    cr.show_text(text).unwrap();
    cr.stroke().unwrap();
}
