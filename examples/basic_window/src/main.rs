use std::sync::Arc;

use neutron::*;
use rendering::prelude::{Device, Fence, Instance, Surface, Swapchain};
use winit::{event_loop::EventLoop, window::Window};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = Arc::new(Window::new(&event_loop).unwrap());

    window.set_title("Neutron");

    window.focus_window();

    let instance = Instance::from_display_handle(&event_loop).unwrap();

    let surface = Surface::new(instance.clone(), window).unwrap();

    let device = Device::new(instance.clone()).unwrap();

    let fence = Fence::new(device.clone());

    let swapchain = Swapchain::new(device.clone(), surface.clone()).unwrap();

    event_loop
        .run(|event, target| match event {
            winit::event::Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                winit::event::WindowEvent::CloseRequested => target.exit(),
                winit::event::WindowEvent::RedrawRequested => {
                    let (index, _) = swapchain.aquire_next_image();
                    swapchain.present(index, device.queue());
                }
                _ => {}
            },

            _ => {}
        })
        .unwrap()
}
