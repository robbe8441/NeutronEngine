use ash::vk;

use anyhow::Result;
use std::{ffi::c_void, sync::Arc};

use crate::prelude::{BufferCreateInfo, Device, RawBuffer};

#[allow(unused)]
pub struct Subbuffer<T> {
    buffer: Arc<RawBuffer>,
    size: vk::DeviceSize,
    memory: vk::DeviceMemory,
    offset: u64,
    device: Arc<Device>,
    align: Option<(ash::util::Align<T>, *mut c_void)>,
}

impl<T: Copy> Subbuffer<T> {
    pub fn from_data(device: Arc<Device>, info: BufferCreateInfo, data: &[T]) -> Result<Arc<Self>> {
        let size = std::mem::size_of_val(data) as u64;

        let buffer = RawBuffer::new(device.clone(), info.clone(), size)?;

        let allocate_info = vk::MemoryAllocateInfo::default()
            .allocation_size(buffer.requirements.size)
            .memory_type_index(buffer.memory_type_index);

        let memory = unsafe { device.as_raw().allocate_memory(&allocate_info, None) }?;

        let align = if info
            .visibility
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            unsafe {
                let ptr =
                    device
                        .as_raw()
                        .map_memory(memory, 0, size, vk::MemoryMapFlags::empty())?;

                let mut align = ash::util::Align::new(ptr, std::mem::align_of::<T>() as u64, size);

                align.copy_from_slice(data);

                Some((align, ptr))
            }
        } else {
            None
        };

        Ok(Self {
            buffer,
            size,
            memory,
            offset: 0,
            device,
            align,
        }
        .into())
    }

    pub fn read(&self) -> &[T] {
        let ptr = self.align.clone().unwrap().1;
        unsafe { std::slice::from_raw_parts(ptr.cast::<T>(), self.len() as usize) }
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn len(&self) -> u64 {
        (self.size as f64 / std::mem::size_of::<T>() as f64) as u64
    }
}

impl<T> Drop for Subbuffer<T> {
    fn drop(&mut self) {
        unsafe { self.device.as_raw().free_memory(self.memory, None) };
    }
}
