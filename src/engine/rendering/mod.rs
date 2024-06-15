use anyhow::{Ok, Result};
use std::sync::{Arc, Mutex};

use vulkano::{
    command_buffer::allocator::StandardCommandBufferAllocator,
    descriptor_set::allocator::StandardDescriptorSetAllocator, device::Device, instance::Instance,
    memory::allocator::StandardMemoryAllocator, swapchain::Surface, sync::GpuFuture,
};

mod device;
mod instance;
mod surface;
mod swapchain;
pub mod update;

pub struct Renderer {
    pub device: Arc<Device>,
    pub instance: Arc<Instance>,
    pub surface: Arc<Surface>,
    pub swapchain: swapchain::Swapchain,
    pub queues: device::Queues,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub desc_set_allocator: Arc<StandardDescriptorSetAllocator>,
    pub command_buffer_alloc: Arc<StandardCommandBufferAllocator>,
    pub window: Arc<winit::window::Window>,
    pub gpu_future: Option<Box<dyn vulkano::sync::GpuFuture>>,
}

pub fn new(window: Arc<winit::window::Window>) -> Result<Renderer> {
    let instance = instance::new(&window)?;

    let surface = vulkano::swapchain::Surface::from_window(instance.clone(), window.clone())?;

    let (device, queues) = device::new(instance.clone(), surface.clone())?;

    let swapchain = swapchain::Swapchain::new(device.clone(), surface.clone(), window.inner_size());

    Ok(Renderer {
        memory_allocator: Arc::new(StandardMemoryAllocator::new_default(device.clone())),

        desc_set_allocator: Arc::new(StandardDescriptorSetAllocator::new(
            device.clone(),
            Default::default(),
        )),
        command_buffer_alloc: Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        )),

        gpu_future: Some(vulkano::sync::now(device.clone()).boxed()),
        instance,
        surface,
        queues,
        device,
        swapchain,
        window,
    })
}
