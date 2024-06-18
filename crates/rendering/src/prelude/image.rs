use std::sync::Arc;

use ash::vk;
use crate::prelude::Device;


pub struct Image {
    handle: vk::Image,
    device: Arc<Device>
}


pub struct ImageView {
    handle: vk::ImageView,
    image: Arc<Image>,
    device: Arc<Device>
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






