use anyhow::Result;
mod octree;
use smallvec::SmallVec;
use std::{sync::Arc, time::Instant};
use types::main_shader;

use vulkano::{
    buffer::{BufferUsage, Subbuffer},
    command_buffer::{
        CommandBufferBeginInfo, CommandBufferLevel, CommandBufferUsage, RecordingCommandBuffer,
    },
    descriptor_set::{DescriptorSet, WriteDescriptorSet},
    image::{view::ImageView, ImageAspects, ImageUsage},
    memory::allocator::MemoryTypeFilter,
    pipeline::{
        layout::PipelineDescriptorSetLayoutCreateInfo, ComputePipeline, Pipeline, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    swapchain::{acquire_next_image, SwapchainPresentInfo},
    sync::{GpuFuture, Sharing},
    Validated, VulkanError,
};

use super::Renderer;
mod types;

pub struct Scene {
    main_pipeline: Arc<ComputePipeline>,
    descriptor_sets: Vec<Arc<DescriptorSet>>,
    scene_descriptors: Arc<DescriptorSet>,
    voxel_buffer: Subbuffer<[u64]>,
    time: Instant,
}

impl Scene {
    pub fn new(renderer: &Renderer) -> Self {
        let main_pipeline = {
            let shader = types::main_shader::load(renderer.device.clone())
                .unwrap()
                .entry_point("main")
                .unwrap();

            // Make a list of the shader stages that the pipeline will have.
            let shader_stage = PipelineShaderStageCreateInfo::new(shader);

            let layout = PipelineLayout::new(
                renderer.device.clone(),
                PipelineDescriptorSetLayoutCreateInfo::from_stages(&[shader_stage.clone()])
                    .into_pipeline_layout_create_info(renderer.device.clone())
                    .unwrap(),
            )
            .unwrap();

            ComputePipeline::new(
                renderer.device.clone(),
                None,
                vulkano::pipeline::compute::ComputePipelineCreateInfo::stage_layout(
                    shader_stage,
                    layout,
                ),
            )
            .unwrap()
        };

        let layout = &main_pipeline.layout().set_layouts()[0];

        let descriptor_sets = renderer
            .swapchain
            .images
            .iter()
            .map(|image| {
                dbg!(image.format());
                ImageView::new(
                    image.clone(),
                    vulkano::image::view::ImageViewCreateInfo {
                        view_type: vulkano::image::view::ImageViewType::Dim2d,
                        format: image.format(),
                        usage: ImageUsage::STORAGE,
                        subresource_range: vulkano::image::ImageSubresourceRange {
                            aspects: ImageAspects::COLOR,
                            mip_levels: 0..1,
                            array_layers: 0..1,
                        },
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .map(|view| {
                DescriptorSet::new(
                    renderer.desc_set_allocator.clone(),
                    layout.clone(),
                    [WriteDescriptorSet::image_view(0, view.clone())],
                    [],
                )
                .unwrap()
            })
            .collect::<Vec<_>>();

        let voxel_buffer = vulkano::buffer::Buffer::from_iter(
            renderer.memory_allocator.clone(),
            vulkano::buffer::BufferCreateInfo {
                sharing: Sharing::Concurrent(SmallVec::from_slice(&[0, 1])),
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            vulkano::memory::allocator::AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            (0..1000).map(|_| 0_u64),
        )
        .unwrap();

        voxel_buffer.write().unwrap()[0] = types::ROOT;

        println!("{:b}", (129526_u32 & 0x000000FF));


        let layout2 = &main_pipeline.layout().set_layouts()[1];

        let scene_descriptors = DescriptorSet::new(
            renderer.desc_set_allocator.clone(),
            layout2.clone(),
            [WriteDescriptorSet::buffer(0, voxel_buffer.clone())],
            [],
        )
        .unwrap();

        Self {
            main_pipeline,
            descriptor_sets,
            voxel_buffer,
            scene_descriptors,
            time: Instant::now(),
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer) -> Result<()> {
        let image_extent: [u32; 2] = renderer.window.inner_size().into();

        if image_extent.contains(&0) {
            return Ok(());
        }

        renderer.gpu_future.as_mut().unwrap().cleanup_finished();

        if renderer.swapchain.recreate_swapchain {
            renderer.swapchain.recreate(renderer.window.inner_size());
        }

        let (image_index, suboptimal, acquire_future) =
            match acquire_next_image(renderer.swapchain.swapchain.clone(), None)
                .map_err(Validated::unwrap)
            {
                Ok(r) => r,
                Err(VulkanError::OutOfDate) => {
                    renderer.swapchain.recreate_swapchain = true;
                    return Ok(());
                }
                Err(e) => panic!("failed to acquire next image: {e}"),
            };

        if suboptimal {
            renderer.swapchain.recreate_swapchain = true;
        }

        let queue = renderer.queues.render();
        let mut builder = RecordingCommandBuffer::new(
            renderer.command_buffer_alloc.clone(),
            queue.queue_family_index(),
            CommandBufferLevel::Primary,
            CommandBufferBeginInfo {
                usage: CommandBufferUsage::OneTimeSubmit,
                ..Default::default()
            },
        )
        .unwrap();

        let push_constants = main_shader::PushConstantData {
            iTime: self.time.elapsed().as_secs_f32(),
        };

        builder
            .push_constants(self.main_pipeline.layout().clone(), 0, push_constants)
            .unwrap()
            .bind_descriptor_sets(
                vulkano::pipeline::PipelineBindPoint::Compute,
                self.main_pipeline.layout().clone(),
                0,
                self.descriptor_sets[image_index as usize].clone(),
            )
            .unwrap()
            .bind_descriptor_sets(
                vulkano::pipeline::PipelineBindPoint::Compute,
                self.main_pipeline.layout().clone(),
                1,
                self.scene_descriptors.clone(),
            )
            .unwrap()
            .bind_pipeline_compute(self.main_pipeline.clone())
            .unwrap();

        let image_size = renderer.window.inner_size();
        unsafe {
            builder
                .dispatch([
                    (image_size.width + 31) / 32,
                    (image_size.height + 31) / 32,
                    1,
                ])
                .unwrap();
        }

        let command_buffer = builder.end().unwrap();

        let future = renderer
            .gpu_future
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(
                queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(
                    renderer.swapchain.swapchain.clone(),
                    image_index,
                ),
            )
            .then_signal_fence_and_flush();

        match future.map_err(Validated::unwrap) {
            Ok(future) => {
                renderer.gpu_future = Some(future.boxed());
            }
            Err(VulkanError::OutOfDate) => {
                renderer.swapchain.recreate_swapchain = true;
                renderer.gpu_future = Some(vulkano::sync::now(renderer.device.clone()).boxed());
            }
            Err(e) => {
                panic!("failed to flush future: {e}");
            }
        }

        Ok(())
    }
}
