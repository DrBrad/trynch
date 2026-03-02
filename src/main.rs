#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod ui;
mod bus;
mod utils;

use crate::ui::gtk4::app::App;
use crate::utils::{keyboard, usb, camera};

fn main() {
    keyboard::run();
    usb::run();
    //camera::run();

    /*
    let mut reader = LogReader::open("logs/test.log").unwrap();

    for log in reader.logs() {
        match log {
            Ok((a)) => {

            }
            Err(e) => {
            }
        }
    }
    */


    let app = App::new();
    app.run();
}


