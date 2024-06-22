use anyhow::Result;
use ash::vk;
use std::sync::Arc;

use crate::prelude::{DescriptorPool, DescriptorSetLayout};

#[allow(unused)]
pub struct DescriptorSets {
    handle: Vec<vk::DescriptorSet>,
    pool: Arc<DescriptorPool>,
}

impl DescriptorSets {
    pub fn new(pool: Arc<DescriptorPool>, layouts: &[Arc<DescriptorSetLayout>]) -> Result<Arc<Self>> {
        let device = pool.device();

        let raw_layouts: Vec<_> = layouts.into_iter().map(|v| *v.as_raw()).collect();

        let alloc_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(*pool.as_raw())
            .set_layouts(&raw_layouts);

        let handle = unsafe { device.as_raw().allocate_descriptor_sets(&alloc_info) }?;

        Ok(Self { handle, pool }.into())
    }
}
