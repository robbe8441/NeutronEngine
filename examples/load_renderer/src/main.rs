

use neutron::window::{Window, events::*};
use neutron::rendering::prelude::*;


fn main() {
    let mut window = Window::new().unwrap();

    let instance = Instance::new(window.raw_display_handle()).unwrap();
    let surface_loader = SurfaceLoader::new(instance.clone(), window.clone());
    let device = Device::new(instance.clone(), surface_loader.clone()).unwrap();

    let surface = surface_loader.build(device.clone()).unwrap();

    let swapchain = Swapchain::new(surface.clone());

    while !window.should_close() {
        window.poll_events();
    }
}
