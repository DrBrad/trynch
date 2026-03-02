mod ui;
mod bus;
mod utils;

use crate::ui::gtk4::app::App;
use crate::utils::{camera, keyboard, usb};

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


