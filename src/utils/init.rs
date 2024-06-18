use std::env;
use std::process::Command;
use gtk::gdk::Display;
use gtk::CssProvider;

pub fn init() -> Result<(), Box<dyn std::error::Error>>{
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

pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("../gui/gui.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
