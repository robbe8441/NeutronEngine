use std::sync::Arc;
mod command_allocator;
pub use command_allocator::*;

use anyhow::Result;
pub use ash::vk; // TODO, make private

use crate::prelude::Device;

pub use vk::{RenderingInfo, RenderingAttachmentInfo};

pub struct CommandBuffer {
    handle: vk::CommandBuffer,
    device: Arc<Device>,
    allocator: Arc<CommandPool>,
}

#[allow(unused)]
impl CommandBuffer {
    pub fn new_count(
        allocator: Arc<CommandPool>,
        device: Arc<Device>,
        count: u32,
    ) -> Result<Vec<Self>> {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_buffer_count(count)
            .command_pool(*allocator.as_raw())
            .level(vk::CommandBufferLevel::PRIMARY);

        let buffers = unsafe {
            device
                .as_raw()
                .allocate_command_buffers(&command_buffer_allocate_info)?
        };

        Ok(buffers
            .into_iter()
            .map(|handle| Self {
                device: device.clone(),
                handle,
                allocator: allocator.clone(),
            })
            .collect::<Vec<_>>())
    }

    pub fn new(allocator: Arc<CommandPool>, device: Arc<Device>) -> Result<Self> {
        Ok(Self::new_count(allocator, device, 1)?
            .into_iter()
            .last()
            .unwrap())
    }

    /// begin recording the command buffer
    /// this MUST be called before you start recording commands
    pub fn begin(&self) {
        let begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            self.device
                .as_raw()
                .begin_command_buffer(self.handle, &begin_info)
        }
        .unwrap();
    }

    pub fn begin_rendering(&self, info: &vk::RenderingInfo) {
        unsafe { self.device.as_raw().cmd_begin_rendering(self.handle, info) }
    }

    pub fn begin_render_pass(&self, info: &vk::RenderPassBeginInfo, contents: vk::SubpassContents ) {
        unsafe { self.device.as_raw().cmd_begin_render_pass(self.handle, info, contents) }
    }

    /// end recording
    /// needs to be called before submit
    pub fn end(&self) {
        unsafe { self.device.as_raw().end_command_buffer(self.handle) }.unwrap();
    }

    pub fn as_raw(&self) -> &vk::CommandBuffer {
        &self.handle
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe {
            self.device
                .as_raw()
                .free_command_buffers(*self.allocator.as_raw(), &[self.handle])
        };
    }
}
