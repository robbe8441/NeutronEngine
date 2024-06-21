use std::sync::Arc;

use crate::prelude::{BufferCreateInfo, BufferSharingMode, Device};
use anyhow::{Context, Result};
use ash::vk;

#[allow(unused)]
pub struct RawBuffer {
    handele: vk::Buffer,
    device: Arc<Device>,
    pub requirements: vk::MemoryRequirements,
    pub memory_type_index: u32,
}

impl RawBuffer {
    pub fn new(device: Arc<Device>, info: BufferCreateInfo, size: u64) -> Result<Arc<Self>> {
        let mut create_info = vk::BufferCreateInfo::default().usage(info.usage).size(size);

        create_info = match info.share_mode {
            BufferSharingMode::Exclusive => create_info.sharing_mode(vk::SharingMode::EXCLUSIVE),
            BufferSharingMode::Concuttrnt(v) => create_info
                .sharing_mode(vk::SharingMode::EXCLUSIVE)
                .queue_family_indices(v),
        };

        let handele = unsafe { device.as_raw().create_buffer(&create_info, None) }?;

        let requirements = unsafe { device.as_raw().get_buffer_memory_requirements(handele) };

        let propeties = device.physical_device_memory_properties();

        let memory_type_index = find_memorytype_index(&requirements, &propeties, info.visibility)
            .context("failed to find matching memory_type_index")?;

        Ok(Self {
            handele,
            device,
            requirements,
            memory_type_index,
        }
        .into())
    }
}

impl Drop for RawBuffer {
    fn drop(&mut self) {
        unsafe { self.device.as_raw().destroy_buffer(self.handele, None) };
    }
}

pub fn find_memorytype_index(
    memory_req: &vk::MemoryRequirements,
    memory_prop: &vk::PhysicalDeviceMemoryProperties,
    flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    memory_prop.memory_types[..memory_prop.memory_type_count as _]
        .iter()
        .enumerate()
        .find(|(index, memory_type)| {
            (1 << index) & memory_req.memory_type_bits != 0
                && memory_type.property_flags & flags == flags
        })
        .map(|(index, _memory_type)| index as _)
}
