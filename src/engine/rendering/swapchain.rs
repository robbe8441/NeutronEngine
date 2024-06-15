use std::sync::Arc;

use vulkano::{
    device::Device,
    image::ImageUsage,
    swapchain::{Surface, SwapchainCreateInfo},
};

pub struct Swapchain {
    pub swapchain: Arc<vulkano::swapchain::Swapchain>,
    pub images: Vec<Arc<vulkano::image::Image>>,
    pub recreate_swapchain: bool,
}

impl Swapchain {
    pub fn new(device: Arc<Device>, surface: Arc<Surface>, size: impl Into<[u32; 2]>) -> Self {
        let (swapchain, images) = {
            let surface_capabilities = device
                .physical_device()
                .surface_capabilities(&surface, Default::default())
                .unwrap();
            let image_format = device
                .physical_device()
                .surface_formats(&surface, Default::default())
                .unwrap()[0]
                .0;

            vulkano::swapchain::Swapchain::new(
                device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count.max(2),
                    image_format,
                    image_extent: size.into(),
                    image_usage: ImageUsage::STORAGE,
                    present_mode: vulkano::swapchain::PresentMode::Immediate,
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next()
                        .unwrap(),
                    ..Default::default()
                },
            )
            .unwrap()
        };

        Self {
            swapchain,
            images,
            recreate_swapchain: false,
        }
    }

    pub fn recreate(&mut self, extent: impl Into<[u32; 2]>) {
        let (new_swapchain, new_images) = self
            .swapchain
            .recreate(SwapchainCreateInfo {
                image_extent: extent.into(),
                ..self.swapchain.create_info()
            })
            .unwrap();

        self.swapchain = new_swapchain;
        self.images = new_images;

        self.recreate_swapchain = false;
    }
}
