use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;
#[path = "orientation/Orientation.rs"] mod Orientation;

pub fn initialize() {
    thread::spawn(move || loop {
        Orientation::orientation::start();
    }).join();
}
