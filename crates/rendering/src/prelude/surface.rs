use anyhow::Result;
use std::sync::Arc;

use crate::prelude::{Device, Instance};
use ash::{khr::surface, vk};

pub struct SurfaceLoader {
    loader: surface::Instance,
    surface: vk::SurfaceKHR,
    window: Arc<window::Window>,
}

pub struct Surface {
    handle: vk::SurfaceKHR,
    device: Arc<Device>,
    format: vk::SurfaceFormatKHR,
    capabilities: vk::SurfaceCapabilitiesKHR,
    present_mode: vk::PresentModeKHR,
    window: Arc<window::Window>,
}

impl SurfaceLoader {
    pub fn new(instance: Arc<Instance>, window: Arc<window::Window>) -> Arc<Self> {
        let loader = surface::Instance::new(&instance.entry(), &instance.as_raw());

        dbg!(window.raw_display_handle());
        let surface = unsafe {
            ash_window::create_surface(
                &instance.entry(),
                &instance.as_raw(),
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            )
        }
        .unwrap();

        Self {
            loader,
            surface,
            window,
        }.into()
    }

    pub fn as_raw(&self) -> surface::Instance {
        self.loader.clone()
    }
    pub fn surface(&self) -> vk::SurfaceKHR {
        self.surface.clone()
    }
    pub fn window(&self) -> Arc<window::Window> {
        self.window.clone()
    }

    pub fn build(&self, device: Arc<Device>) -> Result<Arc<Surface>> {
        let format = unsafe {
            self.as_raw()
                .get_physical_device_surface_formats(device.physical(), self.surface())
                .unwrap()[0]
        };

        let capabilities = unsafe {
            self.as_raw()
                .get_physical_device_surface_capabilities(device.physical(), self.surface())
                .unwrap()
        };

        let present_modes = unsafe {
            self.as_raw()
                .get_physical_device_surface_present_modes(device.physical(), self.surface())
        }
        .unwrap();
        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO);

        Ok(Surface {
            handle: self.surface(),
            window: self.window(),
            device,
            format,
            capabilities,
            present_mode,
        }
        .into())
    }
}

impl Surface {
    pub fn as_raw(&self) -> vk::SurfaceKHR {
        self.handle.clone()
    }

    pub fn resolution(&self) -> vk::Extent2D {
        let window_size = self.window.get_size();
        match self.capabilities().current_extent.width {
            u32::MAX => vk::Extent2D {
                width: window_size.0 as u32,
                height: window_size.1 as u32,
            },
            _ => self.capabilities().current_extent,
        }
    }

    pub fn capabilities(&self) -> vk::SurfaceCapabilitiesKHR {
        self.capabilities.clone()
    }
    pub fn format(&self) -> vk::SurfaceFormatKHR {
        self.format.clone()
    }
    pub fn present_mode(&self) -> vk::PresentModeKHR {
        self.present_mode.clone()
    }
    pub fn device(&self) -> Arc<Device> {
        self.device.clone()
    }
}
