use anyhow::Result;
use std::sync::Arc;

use crate::prelude::{CommandBuffer, Device};
use ash::vk;

pub struct CommandBufferAllocator {
    handle: vk::CommandPool,
    device: Arc<Device>,
}

impl CommandBufferAllocator {
    pub fn new(device: Arc<Device>) -> Result<Arc<Self>> {
        let pool_create_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::TRANSIENT)
            .queue_family_index(device.queue_family_index());

        let pool = unsafe { device.as_raw().create_command_pool(&pool_create_info, None) }?;

        Ok(Self {
            handle: pool,
            device,
        }
        .into())
    }


    pub fn as_raw(&self) -> &vk::CommandPool {
        &self.handle
    }
}

impl Drop for CommandBufferAllocator {
    fn drop(&mut self) {
        unsafe { self.device.as_raw().destroy_command_pool(self.handle, None) };
    }
}
