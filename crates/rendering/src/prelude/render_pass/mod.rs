use anyhow::Result;
use std::sync::Arc;

use crate::prelude::Device;
use ash::vk;

pub struct RenderPass {
    handle: vk::RenderPass,
    device: Arc<Device>,
    attachments_refs: Vec<vk::AttachmentReference>,
    attachment_decriptors: Vec<vk::AttachmentDescription>,
}

impl RenderPass {
    pub fn new(device: Arc<Device>, format: vk::Format) -> Result<Arc<Self>> {
        let attachments_refs = [vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }]
        .to_vec();

        let attachment_decriptors = [vk::AttachmentDescription::default()
            .format(format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)]
        .to_vec();

        let subpasses = [vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&attachments_refs)];

        let info = vk::RenderPassCreateInfo::default()
            .subpasses(&subpasses)
            .attachments(&attachment_decriptors);

        let handle = unsafe { device.as_raw().create_render_pass(&info, None) }?;

        Ok(Self {
            handle,
            device,
            attachments_refs,
            attachment_decriptors,
        }
        .into())
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe { self.device.as_raw().destroy_render_pass(self.handle, None) };
    }
}
