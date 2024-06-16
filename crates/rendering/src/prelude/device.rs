use anyhow::Result;
use std::sync::Arc;

use crate::prelude::{Instance, SurfaceLoader};
use ash::vk;

#[allow(unused)]
pub struct Device {
    handle: ash::Device,
    physical_device: vk::PhysicalDevice,
    instance: Arc<Instance>,
    queue_family_index: u32,
    queues: Queues,
}

#[allow(unused)]
pub struct Queues {
    graphics: vk::Queue,
    compute: vk::Queue,
}

impl Device {
    pub fn new(instance: Arc<Instance>, surface: Arc<SurfaceLoader>) -> Result<Arc<Self>> {
        let physical_devices = unsafe { instance.as_raw().enumerate_physical_devices() }?;

        let (physical_device, queue_family_index) = unsafe {
            physical_devices
                .iter()
                .find_map(|pdevice| {
                    instance
                        .as_raw()
                        .get_physical_device_queue_family_properties(*pdevice)
                        .iter()
                        .enumerate()
                        .find_map(|(index, info)| {
                            let supports_graphic_and_surface =
                                info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                                    && surface
                                        .as_raw()
                                        .get_physical_device_surface_support(
                                            *pdevice,
                                            index as u32,
                                            surface.surface(),
                                        )
                                        .unwrap();
                            if supports_graphic_and_surface {
                                Some((*pdevice, index as u32))
                            } else {
                                None
                            }
                        })
                })
                .expect("Couldn't find suitable device.")
        };

        let device_extension_names_raw = [
            ash::khr::swapchain::NAME.as_ptr(),
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            ash::khr::portability_subset::NAME.as_ptr(),
        ];
        let features = vk::PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };

        let priorities = [1.0];

        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&priorities);

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(std::slice::from_ref(&queue_info))
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features);

        let device: ash::Device = unsafe {
            instance
                .as_raw()
                .create_device(physical_device, &device_create_info, None)
        }
        .unwrap();

        let graphics = unsafe { device.get_device_queue(queue_family_index, 0) };
        let compute = unsafe { device.get_device_queue(queue_family_index, 1) };

        Ok(Self {
            handle: device,
            queue_family_index,
            instance,
            queues: Queues { graphics, compute },
            physical_device,
        }
        .into())
    }

    pub fn as_raw(&self) -> ash::Device {
        self.handle.clone()
    }
    pub fn physical(&self) -> vk::PhysicalDevice {
        self.physical_device.clone()
    }
    pub fn instance(&self) -> Arc<Instance> {
        self.instance.clone()
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { self.handle.destroy_device(None) }
    }
}
