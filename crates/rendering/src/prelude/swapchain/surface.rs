use anyhow::Result;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::sync::{Arc, RwLock};

use crate::prelude::{Device, Instance};
use ash::vk;
use winit::window::Window;

pub struct Surface {
    handle: vk::SurfaceKHR,
    loader: ash::khr::surface::Instance,

    infos: RwLock<Option<SurfaceInfos>>,
    instance: Arc<Instance>,
    window: Arc<Window>,
}

#[derive(Clone)]
pub struct SurfaceInfos {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub present_mode: vk::PresentModeKHR,
    pub format: vk::SurfaceFormatKHR,
}

impl Surface {
    pub fn new(instance: Arc<Instance>, window: Arc<Window>) -> Result<Arc<Self>> {
        let handle = unsafe {
            ash_window::create_surface(
                instance.entry(),
                instance.as_raw(),
                window.display_handle()?.as_raw(),
                window.window_handle()?.as_raw(),
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
    pub fn window(&self) -> Arc<Window> {
        self.window.clone()
    }

    pub fn size(&self) -> vk::Extent2D {
        let size = self.window.inner_size();
        vk::Extent2D {
            width: size.width,
            height: size.height,
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

