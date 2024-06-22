use std::sync::Arc;

use neutron::*;
use rendering::prelude::{
    vk, BufferCreateInfo, BufferUsageFlags, CommandBuffer, CommandPool, DescriptorPool, DescriptorSetLayout, DescriptorSets, DescriptorType, Device, Fence, Instance, MemoryPropertyFlags, RenderPass, RenderingInfo, ShaderStageFlags, Subbuffer, Surface, Swapchain, VKDebugger
};
use winit::{event_loop::EventLoop, window::Window};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = Arc::new(Window::new(&event_loop).unwrap());

    window.set_title("Neutron");

    window.focus_window();

    let instance = Instance::from_display_handle(&event_loop).unwrap();

    let _debugger = VKDebugger::new(instance.clone());

    let surface = Surface::new(instance.clone(), window.clone()).unwrap();

    let device = Device::new(instance.clone()).unwrap();

    let _fence = Fence::new(device.clone());

    let swapchain = Swapchain::new(device.clone(), surface.clone()).unwrap();
    let command_pool = CommandPool::new(device.clone()).unwrap();
    let command_buffer = CommandBuffer::new(command_pool.clone(), device.clone()).unwrap();

    let _render_pass = RenderPass::new(device.clone(), swapchain.format()).unwrap();


    let descriptors = &[DescriptorType {
        binding: 0,
        count: 0,
        ty: neutron::rendering::prelude::VkDescriptorType::STORAGE_BUFFER,
        stage_flags: ShaderStageFlags::COMPUTE,
    }];

    let descriptor_pool = DescriptorPool::new(device.clone(), descriptors).unwrap();
    let descriptor_layout = DescriptorSetLayout::new(device.clone(), descriptors).unwrap();
    let _descriptor_sets = DescriptorSets::new(descriptor_pool.clone(), &[descriptor_layout]);

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
        .unwrap();
}
