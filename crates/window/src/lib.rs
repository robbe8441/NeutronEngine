use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
pub mod events;

use glfw::{WindowEvent, WindowHint};

pub struct Window {
    handle: glfw::PWindow,
    glfw: glfw::Glfw,
    events: Arc<Mutex<glfw::GlfwReceiver<(f64, glfw::WindowEvent)>>>,
    meta: WindowMetaData,
}

struct WindowMetaData {
    last_pos: (i32, i32),
    last_size: (i32, i32),
    full_screen: bool,
}

impl Window {
    pub fn new() -> Result<Self> {
        use glfw::log_errors;

        let mut glfw = glfw::init(log_errors!())?;

        let (mut window, events) = glfw
            .create_window(600, 400, "Neutron Application", glfw::WindowMode::Windowed)
            .context("failed to create window")?;

        window.set_all_polling(true);

        Ok(Self {
            meta: WindowMetaData {
                last_pos: window.get_pos(),
                last_size: window.get_size(),
                full_screen: false,
            },
            glfw,
            events: Mutex::new(events).into(),
            handle: window,
        })
    }

    pub fn set_full_screen(&mut self, toggle: bool) {
        self.meta.full_screen = toggle;

        if toggle {
            self.meta.last_pos = self.position();
            self.meta.last_size = self.size();

            self.set_decorated(false);

            let mode = self
                .glfw
                .with_primary_monitor(|_, m| m.unwrap().get_video_mode().unwrap());

            self.glfw.window_hint(WindowHint::Decorated(false));

            self.handle.set_monitor(
                glfw::WindowMode::Windowed,
                0,
                0,
                mode.width,
                mode.height,
                Some(mode.refresh_rate),
            );
        } else {
            self.set_decorated(true);
            self.handle.set_monitor(
                glfw::WindowMode::Windowed,
                self.meta.last_pos.0,
                self.meta.last_pos.1,
                self.meta.last_size.0 as u32,
                self.meta.last_size.1 as u32,
                None,
            );
        }
    }

    pub fn run<F>(&mut self, func: F)
    where
        F: Fn(&WindowEvent, &mut Window),
    {
        loop {
            self.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events.clone().lock().unwrap()) {
                func(&event, self)
            }

            if self.handle.should_close() {
                break;
            }
        }
    }

    pub fn close(&mut self) {
        self.handle.set_should_close(true);
    }
    pub fn set_title(&mut self, title: &str) {
        self.handle.set_title(title);
    }
    pub fn set_position(&mut self, posx: i32, posy: i32) {
        self.handle.set_pos(posx, posy);
    }
    pub fn set_size(&mut self, sizex: i32, sizey: i32) {
        self.handle.set_size(sizex, sizey);
    }
    pub fn set_decorated(&mut self, decorated: bool) {
        self.handle.set_decorated(decorated)
    }
    pub fn set_swap_interval(&mut self, interval: u32) {
        self.handle
            .glfw
            .set_swap_interval(glfw::SwapInterval::Sync(interval));
    }

    pub fn position(&self) -> (i32, i32) {
        self.handle.get_pos()
    }
    pub fn size(&self) -> (i32, i32) {
        self.handle.get_size()
    }
    pub fn is_full_screen(&self) -> bool {
        self.meta.full_screen
    }
}
