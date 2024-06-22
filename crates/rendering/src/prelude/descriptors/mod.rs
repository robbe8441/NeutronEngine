use ash::vk;

mod layout;
mod pool;
mod sets;

pub use layout::*;
pub use pool::*;
pub use sets::*;
pub use vk::{DescriptorType as VkDescriptorType, ShaderStageFlags};


#[derive(Debug, Clone, Copy)]
pub struct DescriptorType {
    pub ty: vk::DescriptorType,
    pub stage_flags: vk::ShaderStageFlags,
    pub count: u32,
    pub binding: u32,
}

impl Into<vk::DescriptorPoolSize> for DescriptorType {
    fn into(self) -> vk::DescriptorPoolSize {
        vk::DescriptorPoolSize {
            ty: self.ty,
            descriptor_count: self.count,
        }
    }
}

impl<'a> Into<vk::DescriptorSetLayoutBinding<'a>> for DescriptorType {
    fn into(self) -> vk::DescriptorSetLayoutBinding<'a> {
        vk::DescriptorSetLayoutBinding {
            binding: self.binding,
            descriptor_type: self.ty,
            descriptor_count: self.count,
            stage_flags: self.stage_flags,
            ..Default::default()
        }
    }
}
