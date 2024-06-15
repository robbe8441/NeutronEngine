use anyhow::Result;
use std::sync::Arc;

use vulkano::device::Device;
use vulkano::device::{
    physical::PhysicalDeviceType, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo, QueueFlags,
};
use vulkano::{instance::Instance, swapchain::Surface};

#[rustfmt::skip]
const QUEUES: [(f32, QueueFlags); 3] = [
    (0.7, QueueFlags::GRAPHICS), 
    (0.5, QueueFlags::COMPUTE),
    (0.3, QueueFlags::TRANSFER),
];

#[derive(Clone)]
pub struct Queues(Vec<Arc<vulkano::device::Queue>>);

pub fn new(instance: Arc<Instance>, surface: Arc<Surface>) -> Result<(Arc<Device>, Queues)> {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };
    let physical_device = instance
        .enumerate_physical_devices()
        .unwrap()
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .min_by_key(|p| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .unwrap();

    let mut used_queue_families = vec![];

    let queue_create_infos = QUEUES
        .iter()
        .map(|(priority, flags)| {
            let index = physical_device
                .queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.intersects(*flags)
                        && physical_device
                            .surface_support(i as u32, &surface)
                            .unwrap_or(false)
                        && !used_queue_families.contains(&i)
                })
                .unwrap();
            used_queue_families.push(index);
            (index as u32, *priority)
        })
        .map(|(queue_family_index, priority)| QueueCreateInfo {
            queue_family_index,
            queues: vec![priority],
            ..Default::default()
        })
        .collect();

    log::info!(
        "Using device: {} (type: {:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type,
    );

    let (device, queues) = vulkano::device::Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos,
            ..Default::default()
        },
    )
    .unwrap();

    let queues = Queues(queues.collect());

    Ok((device, queues))
}

impl Queues {
    pub fn render(&self) -> Arc<vulkano::device::Queue> {
        self.0[0].clone()
    }
    pub fn compute(&self) -> Arc<vulkano::device::Queue> {
        self.0[1].clone()
    }
    pub fn transfer(&self) -> Arc<vulkano::device::Queue> {
        self.0[2].clone()
    }
}
