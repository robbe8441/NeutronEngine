use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window};

use anyhow::{Ok, Result};

pub fn new() -> Result<(EventLoop<()>, Arc<Window>)> {
    let event_loop = EventLoop::new()?;

    let window = winit::window::WindowBuilder::new()
        .with_title("Voxel Application")
        .build(&event_loop)?;

    Ok((event_loop, Arc::new(window)))
}
