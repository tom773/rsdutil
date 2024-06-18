# Rsdutil

Rsdutil is a fast GUI utility designed for use with Hyprland specifically. The utility will calculate, and display disk usage information.

## Features

* Uses GTK4 bindings, and Cairo for drawing
* Automatic windowrules for Hyprland.
* Libadwiata used for GNOME integration
* Rayon concurrency, batch processing, & libc for system calls = fast

### Installation

```
clone source
cd rsdutil
cargo build --release
```
### Usage

```
./target/release/rsdutil
```
