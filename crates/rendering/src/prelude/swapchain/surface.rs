use anyhow::{Ok, Result};
use std::sync::{Arc, RwLock};

use crate::prelude::{Device, Instance};
use ash::vk;

pub struct Surface {
    handle: vk::SurfaceKHR,
    loader: ash::khr::surface::Instance,
    window: Arc<window::Window>,

    infos: RwLock<Option<SurfaceInfos>>,
    instance: Arc<Instance>,
}

#[derive(Clone)]
pub struct SurfaceInfos {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub present_mode: vk::PresentModeKHR,
    pub format: vk::SurfaceFormatKHR,
}

impl Surface {
    pub fn new(instance: Arc<Instance>, window: Arc<window::Window>) -> Result<Arc<Self>> {
        let handle = unsafe {
            ash_window::create_surface(
                instance.entry(),
                instance.as_raw(),
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            )
        }?;

        let loader = ash::khr::surface::Instance::new(&instance.entry(), &instance.as_raw());

        Ok(Self {
            handle,
            window,
            instance,
            loader,
            infos: RwLock::new(None),
        }
        .into())
    }

    pub fn as_raw(&self) -> &vk::SurfaceKHR {
        &self.handle
    }
    pub fn window(&self) -> Arc<window::Window> {
        self.window.clone()
    }

    pub fn resolution(&self) -> vk::Extent2D {
        let (width, height) = self.window.get_size();
        vk::Extent2D {
            width: width as u32,
            height: height as u32,
        }
    }

    pub(crate) fn setup_infos(&self, device: Arc<Device>) -> Result<SurfaceInfos> {
        let capabilities = unsafe {
            self.loader
                .get_physical_device_surface_capabilities(*device.physical(), *self.as_raw())?
        };

        let present_mode = unsafe {
            self.loader
                .get_physical_device_surface_present_modes(*device.physical(), *self.as_raw())?[0]
        };

        let format = unsafe {
            self.loader
                .get_physical_device_surface_formats(*device.physical(), *self.as_raw())?[0]
        };

        let infos = SurfaceInfos {
            capabilities,
            present_mode,
            format,
        };

        *self.infos.write().unwrap() = Some(infos.clone());

        Ok(infos)
    }

    pub fn infos(&self) -> SurfaceInfos {
        self.infos
            .read()
            .unwrap()
            .clone()
            .expect("you need to call setup_infos() first")
    }
}

