use crate::prelude::Device;
use anyhow::Result;
use ash::vk;
use std::sync::Arc;

pub use vk::{ImageCreateInfo, ImageViewCreateInfo};


#[allow(unused)]
pub struct Image {
    handle: vk::Image,
    device: Arc<Device>,
    info: ImageCreateInfo<'static>,
}

impl Image {
    pub fn new(device: Arc<Device>, info: ImageCreateInfo<'static>) -> Result<Arc<Self>> {
        let handle = unsafe { device.as_raw().create_image(&info, None) }?;

        Ok(Self { device, handle, info }.into())
    }
}

#[allow(unused)]
pub struct ImageView {
    handle: vk::ImageView,
    info: ImageViewCreateInfo<'static>,
    device: Arc<Device>,
    // we need to store the image here just to ensure that its not being droped
    image: Arc<Image>,
}

impl ImageView {

    pub fn new(device: Arc<Device>, image: Arc<Image>, info: ImageViewCreateInfo<'static>) -> Result<Arc<Self>> {
        let handle = unsafe { device.as_raw().create_image_view(&info, None) }?;

        Ok( Self { handle, info, device, image }.into() )
    }
}







impl Drop for ImageView {
    fn drop(&mut self) {
        unsafe { self.device.as_raw().destroy_image_view(self.handle, None) };
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { self.device.as_raw().destroy_image(self.handle, None) };
    }
}
