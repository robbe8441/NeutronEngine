use std::sync::Arc;

use neutron::*;
use rendering::prelude::{
    BufferCreateInfo, BufferUsageFlags, CommandBuffer, CommandPool, Device, Fence, Instance,
    MemoryPropertyFlags, Subbuffer, Surface, Swapchain,
};
use winit::{event_loop::EventLoop, window::Window};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = Arc::new(Window::new(&event_loop).unwrap());

    window.set_title("Neutron");

    window.focus_window();

    let instance = Instance::from_display_handle(&event_loop).unwrap();

    let surface = Surface::new(instance.clone(), window.clone()).unwrap();

    let device = Device::new(instance.clone()).unwrap();

    let fence = Fence::new(device.clone());

    let data = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let buffer_create_info = BufferCreateInfo {
        usage: BufferUsageFlags::STORAGE_BUFFER,
        share_mode: rendering::prelude::BufferSharingMode::Exclusive,
        visibility: MemoryPropertyFlags::HOST_VISIBLE,
    };
    let buffer = Subbuffer::from_data(device.clone(), buffer_create_info, data).unwrap();

    let res = buffer.read();

    dbg!(res);

    let swapchain = Swapchain::new(device.clone(), surface.clone()).unwrap();

    let command_pool = CommandPool::new(device.clone()).unwrap();

    let command_buffer = CommandBuffer::new(command_pool.clone(), device.clone()).unwrap();

    command_buffer.begin();

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
                    // window.request_redraw();
                }
                _ => {}
            },

            _ => {}
        })
        .unwrap()
}
