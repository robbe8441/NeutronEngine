use std::{io::Read, sync::Arc};

use winit::{event_loop::EventLoop, window::Window};

use anyhow::{Ok, Result};

pub fn new() -> Result<(EventLoop<()>, Arc<Window>)> {
    let event_loop = EventLoop::new()?;

    let image = image::open("AppIcon.png").unwrap().to_rgba8();
    let dimensions = image.dimensions();

    let icon = winit::window::Icon::from_rgba(image.to_vec(), dimensions.0, dimensions.1).unwrap();

    let window = winit::window::WindowBuilder::new()
        .with_title("Voxel Application")
        .with_window_icon(Some(icon))
        .build(&event_loop)?;

    Ok((event_loop, Arc::new(window)))
}
