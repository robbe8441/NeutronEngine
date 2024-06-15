use anyhow::{Ok, Result};
use std::sync::Arc;
use vulkano::{
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    swapchain, VulkanLibrary,
};

// creates a render instance from a EventLoop to ensure that its compatible
pub fn new(event_loop: &winit::window::Window) -> Result<Arc<Instance>> {
    let library = VulkanLibrary::new().unwrap();

    // make sure the required extensions to render on a window are there
    let required_extensions = swapchain::Surface::required_extensions(&event_loop)?;

    // create the render instance
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )?;

    Ok(instance)
}
