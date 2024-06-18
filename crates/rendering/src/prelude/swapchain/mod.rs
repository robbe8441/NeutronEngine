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
}

impl Swapchain {
    pub fn new(device: Arc<Device>, surface: Arc<Surface>) -> Result<Arc<Self>> {

        let infos = surface.infos();

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
            .image_extent(surface.resolution())
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

        Ok(Self {
            handle: swapchain,
            loader: swapchain_loader,
            device,
            images,
            surface,
        }.into())
    }

    pub fn resolution(&self) -> vk::Extent2D {
        self.surface.resolution()
    }
    pub fn format(&self) -> vk::SurfaceFormatKHR {
        self.surface.infos().format
    }
}
