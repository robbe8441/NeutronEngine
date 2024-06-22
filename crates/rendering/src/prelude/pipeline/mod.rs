use ash::vk;

#[allow(unused)]
pub fn create_test_pipeline() {
    let color_attachment = [vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    }];

    let dependencies = [vk::SubpassDependency {
        src_subpass: vk::SUBPASS_EXTERNAL,
        src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
            | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        ..Default::default()
    }];

    // let subpass = vk::SubpassDescription::default()
    //     .color_attachments(&color_attachment)
    //     .depth_stencil_attachment(&depth_attachment)
    //     .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS);
}
