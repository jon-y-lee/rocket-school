#[path = "util/constants.rs"]
mod util;
mod orientation_task_initializer;
mod orientation;

fn main() {
    println!("Orientation Task Initializer");
    println!("{}", util::RAD_TO_DEG);

    orientation_task_initializer::initialize();
}
