use std::{any::Any, sync::{Arc, Mutex}};

use anyhow::Result;
use ash::vk;

use crate::prelude::{CommandBuffer, Device};

pub struct Fence {
    handle: vk::Fence,
    device: Arc<Device>,
    pending_resources: Mutex<Vec<Box<dyn Any>>>,
}

impl Fence {
    pub fn new(device: Arc<Device>) -> Result<Arc<Self>> {
        let info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);

        let fence = unsafe { device.as_raw().create_fence(&info, None) }?;

        Ok(Self {
            handle: fence,
            device,
            pending_resources: vec![].into(),
        }
        .into())
    }

    pub fn submit_command_buffers(
        &self,
        queue: vk::Queue,
        command_buffers: Vec<CommandBuffer>,
    ) -> Result<()> {
        let raw_buffers: Vec<_> = command_buffers
            .iter()
            .map(CommandBuffer::as_raw)
            .cloned()
            .collect();

        let submit = vk::SubmitInfo::default().command_buffers(&raw_buffers);

        unsafe {
            self.device
                .as_raw()
                .queue_submit(queue, &[submit], self.handle)
        }?;

        self.pending_resources.lock().unwrap().push(Box::new(command_buffers));
        Ok(())
    }

    pub fn wait_for_finished(&self) -> Result<()> {
        unsafe {
            self.device
                .as_raw()
                .wait_for_fences(&[self.handle], true, u64::MAX)
        }?;
        Ok(())
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe { self.device.as_raw().destroy_fence(self.handle, None) };
    }
}
