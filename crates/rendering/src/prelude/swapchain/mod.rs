mod surface;
pub use surface::Surface;

use std::sync::Arc;

use crate::prelude::Device;
use anyhow::Result;
use ash::vk;

pub struct Swapchain {
    handle: vk::SwapchainKHR,
    loader: ash::khr::swapchain::Device,
    device: Arc<Device>,
    images: Vec<vk::Image>,
    surface: Arc<Surface>,
    present_semaphore: vk::Semaphore,
}

impl Swapchain {
    pub fn new(device: Arc<Device>, surface: Arc<Surface>) -> Result<Arc<Self>> {
        let infos = surface.setup_infos(device.clone()).unwrap();

        let surface_capabilities = infos.capabilities;
        let present_mode = infos.present_mode;
        let surface_format = infos.format;

        let mut desired_image_count = surface_capabilities.min_image_count + 1;
        if surface_capabilities.max_image_count > 0
            && desired_image_count > surface_capabilities.max_image_count
        {
            desired_image_count = surface_capabilities.max_image_count;
        }

        let pre_transform = if surface_capabilities
            .supported_transforms
            .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
        {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform
        };

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(*surface.as_raw())
            .min_image_count(desired_image_count)
            .image_color_space(surface_format.color_space)
            .image_format(surface_format.format)
            .image_extent(surface.size())
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(pre_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .image_array_layers(1);

        let swapchain_loader =
            ash::khr::swapchain::Device::new(&device.instance().as_raw(), &device.as_raw());

        let swapchain =
            unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None) }.unwrap();

        let images = unsafe { swapchain_loader.get_swapchain_images(swapchain) }?;

        let present_semaphore =
            unsafe { device.as_raw().create_semaphore(&Default::default(), None) }?;

        Ok(Self {
            handle: swapchain,
            loader: swapchain_loader,
            device,
            images,
            surface,
            present_semaphore,
        }
        .into())
    }

    pub fn aquire_next_image(&self) -> (u32, bool) {
        unsafe {
            self.loader.acquire_next_image(
                self.handle,
                u64::MAX,
                self.present_semaphore,
                vk::Fence::null(),
            )
        }
        .unwrap()
    }

    pub fn present(&self, index: u32, queue: vk::Queue) {
        let semaphores = [self.present_semaphore];
        let swapchains = [self.handle];
        let image_indexes = [index];

        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&semaphores) // &base.rendering_complete_semaphore)
            .swapchains(&swapchains)
            .image_indices(&image_indexes);
        unsafe { self.loader.queue_present(queue, &present_info) }.unwrap();
    }

    pub fn as_raw(&self) -> &vk::SwapchainKHR {
        &self.handle
    }

    pub fn resolution(&self) -> vk::Extent2D {
        self.surface.size()
    }
    pub fn format(&self) -> vk::SurfaceFormatKHR {
        self.surface.infos().format
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            self.device
                .as_raw()
                .destroy_semaphore(self.present_semaphore, None);

            self.loader.destroy_swapchain(self.handle, None);
        }
    }
}
