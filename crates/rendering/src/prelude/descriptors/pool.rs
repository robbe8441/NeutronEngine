use std::sync::Arc;

use crate::prelude::{DescriptorType, Device};
use anyhow::Result;
use ash::vk;

#[allow(unused)]
pub struct DescriptorPool {
    handle: vk::DescriptorPool,
    device: Arc<Device>,
}

impl DescriptorPool {
    pub fn new(device: Arc<Device>, sizes: &[DescriptorType]) -> Result<Arc<Self>> {
        let sizes: Vec<vk::DescriptorPoolSize> = sizes.into_iter().map(|v| (*v).into()).collect();

        let info = vk::DescriptorPoolCreateInfo::default().pool_sizes(&sizes);

        let handle = unsafe { device.as_raw().create_descriptor_pool(&info, None) }?;

        Ok(Self { handle, device }.into())
    }

    pub fn device(&self) -> Arc<Device> {
        self.device.clone()
    }
    pub fn as_raw(&self) -> &vk::DescriptorPool {
        &self.handle
    }
    
}

impl Drop for DescriptorPool {
    fn drop(&mut self) {
        unsafe {
            self.device
                .as_raw()
                .destroy_descriptor_pool(self.handle, None)
        }
    }
}
