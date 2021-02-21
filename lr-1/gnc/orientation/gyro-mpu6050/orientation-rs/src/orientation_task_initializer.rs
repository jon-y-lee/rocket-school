use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::util;
use crate::orientation::processor;

pub fn initialize() {
    thread::spawn(move || loop {
        processor::orientation_processor::start();
    }).join();
}
