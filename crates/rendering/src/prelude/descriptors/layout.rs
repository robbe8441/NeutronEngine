use anyhow::Result;
use ash::vk;
use std::sync::Arc;

use crate::prelude::{DescriptorType, Device};

pub struct DescriptorSetLayout {
    handle: vk::DescriptorSetLayout,
    device: Arc<Device>,
}

impl DescriptorSetLayout {
    pub fn new(device: Arc<Device>, decriptors: &[DescriptorType]) -> Result<Arc<Self>> {
        let bindings: Vec<vk::DescriptorSetLayoutBinding> =
            decriptors.into_iter().map(|v| (*v).into()).collect();

        let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);

        let handle = unsafe { device.as_raw().create_descriptor_set_layout(&info, None) }?;

        Ok(Self { handle, device }.into())
    }

    pub fn as_raw(&self) -> &vk::DescriptorSetLayout {
        &self.handle
    }
}

impl Drop for DescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            self.device
                .as_raw()
                .destroy_descriptor_set_layout(self.handle, None)
        };
    }
}
