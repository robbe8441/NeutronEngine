

use neutron::window::{Window, events::*};
use neutron::rendering::prelude::*;


fn main() {
    let mut window = Window::new().unwrap();

    let instance = Instance::new().unwrap();
    let device = Device::new(instance.clone()).unwrap();

    while !window.should_close() {
        window.poll_events();
    }
}


