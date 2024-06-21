mod raw_buffer;
mod sub_buffer;

pub use raw_buffer::*;
pub use sub_buffer::*;

use ash::vk;


pub trait BufferAllocation {
    fn offset(&self) -> u64 { 0 }
    fn size(&self) -> vk::DeviceSize;
    fn buffer(&self) -> vk::Buffer;
}


#[allow(unused)]
#[derive(Clone, Debug)]
pub enum BufferSharingMode<'a> {
    Exclusive,
    Concuttrnt(&'a [u32]),
}

pub use vk::{BufferUsageFlags, MemoryPropertyFlags};

#[derive(Clone, Debug)]
pub struct BufferCreateInfo<'a> {
    pub usage: vk::BufferUsageFlags,
    pub share_mode: BufferSharingMode<'a>,
    pub visibility: vk::MemoryPropertyFlags,
}









