use std::thread;
use crate::orientation::processor;

pub fn initialize() {
    thread::spawn(move || loop {
        processor::orientation_processor::start();
    }).join().map_err(|err| println!("{:?}", err)).ok();
}
