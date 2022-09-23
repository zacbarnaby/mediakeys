#![windows_subsystem = "windows"]

use std::thread;

mod tray_icon;
mod hotkeys;

pub use crate::tray_icon::init_tray_icon;
pub use crate::hotkeys::init_hotkeys;

fn main() {

  thread::spawn(|| {
    init_hotkeys();
  });
  
  init_tray_icon();
}  
