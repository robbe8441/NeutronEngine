use std::{
    ffi::{c_int, CStr}, os::raw::c_void, ptr::null_mut, sync::Arc
};

use anyhow::{anyhow, Result};
pub mod events;

use glfw::ffi::GLFWwindow;
use raw_window_handle::{HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};

pub struct Window {
    handle: *mut GLFWwindow,
    // meta: WindowMetaData,
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}



impl Window {
    pub fn new() -> Result<Arc<Self>> {
        let v = unsafe { glfw::ffi::glfwInit() };
        if v == 0 {
            return Err(anyhow!("failed to setup glfw"));
        }

        let window = unsafe {
            glfw::ffi::glfwCreateWindow(
                600 as c_int,
                400 as c_int,
                CStr::from_bytes_with_nul_unchecked(b"Neutron Application\n").as_ptr(),
                null_mut(),
                null_mut(),
            )
        };

        unsafe {
            glfw::ffi::glfwSetWindowTitle(
                window,
                CStr::from_bytes_with_nul_unchecked(b"Seine Mudda\n").as_ptr(),
            )
        };

        Ok(Self { handle: window }.into())
    }

    pub fn poll_events(&self) {
        unsafe { glfw::ffi::glfwPollEvents() };
    }

    pub fn set_title(&self, title: &str) {
        let mut null = title.to_string();
        null.push_str("\n");

        unsafe {
            let c_str = CStr::from_bytes_with_nul_unchecked(null.as_bytes());
            glfw::ffi::glfwSetWindowTitle(self.handle, c_str.as_ptr());
        }
    }

    pub fn get_position(&self) -> (i32, i32) {
        let mut xpos = 0;
        let mut ypos = 0;
        unsafe { glfw::ffi::glfwGetWindowPos(self.handle, &mut xpos, &mut ypos ) };
        (xpos, ypos)
    }

    pub fn set_position(&self, xpos: i32, ypos:i32) {
        unsafe { glfw::ffi::glfwSetWindowPos(self.handle, xpos, ypos) };
    }

    pub fn get_size(&self) -> (i32, i32) {
        let mut xsize = 0;
        let mut ysize = 0;
        unsafe { glfw::ffi::glfwGetWindowSize(self.handle, &mut xsize, &mut ysize ) };
        (xsize, ysize)
    }

    pub fn set_size(&self, xsize: i32, ysize:i32) {
        unsafe { glfw::ffi::glfwSetWindowSize(self.handle, xsize, ysize) };
    }

    pub fn set_should_close(&self, mode: bool) {
        unsafe { glfw::ffi::glfwSetWindowShouldClose(self.handle, mode as c_int) };
    }

    pub fn should_close(&self) -> bool {
        return unsafe { glfw::ffi::glfwWindowShouldClose(self.handle) } != 0;
    }
}



#[allow(unused)]
impl Window {
    pub fn raw_window_handle(&self) -> RawWindowHandle {
        #[cfg(target_family = "windows")]
        {
            use raw_window_handle::Win32WindowHandle;
            use std::num::NonZeroIsize;
            let (hwnd, hinstance): (*mut std::ffi::c_void, *mut std::ffi::c_void) = unsafe {
                let hwnd= glfw::ffi::glfwGetWin32Window(self.handle);
                let hinstance: *mut c_void = winapi::um::libloaderapi::GetModuleHandleW(std::ptr::null()) as _;
                (hwnd, hinstance as _)
            };
            let mut handle = Win32WindowHandle::new(NonZeroIsize::new(hwnd as isize).unwrap());
            handle.hinstance = NonZeroIsize::new(hinstance as isize);
            RawWindowHandle::Win32(handle)
        }
        #[cfg(all(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly")))]
        {
            use raw_window_handle::XlibWindowHandle;
            let window = unsafe { glfw::ffi::glfwGetX11Window(self.handle) as std::os::raw::c_ulong };
            RawWindowHandle::Xlib(XlibWindowHandle::new(window))
        }
        #[cfg(target_os = "macos")]
        {
            use objc2::msg_send_id;
            use objc2::rc::Id;
            use objc2::runtime::NSObject;
            use raw_window_handle::AppKitWindowHandle;
            use std::ptr::NonNull;
            let ns_window: *mut NSObject =
                unsafe { ffi::glfwGetCocoaWindow(context.window_ptr()) as *mut _ };
            let ns_view: Option<Id<NSObject>> = unsafe { msg_send_id![ns_window, contentView] };
            let ns_view = ns_view.expect("failed to access contentView on GLFW NSWindow");
            let ns_view: NonNull<NSObject> = NonNull::from(&*ns_view);
            let handle = AppKitWindowHandle::new(ns_view.cast());
            RawWindowHandle::AppKit(handle)
        }
        #[cfg(target_os = "emscripten")]
        {
            let _ = context; // to avoid unused lint
            let mut wh = raw_window_handle::WebWindowHandle::new(1);
            // glfw on emscripten only supports a single window. so, just hardcode it
            // sdl2 crate does the same. users can just add `data-raw-handle="1"` attribute to their canvas element
            RawWindowHandle::Web(wh)
        }
    }

    pub fn raw_display_handle(&self) -> RawDisplayHandle {
        #[cfg(target_family = "windows")]
        {
            use raw_window_handle::WindowsDisplayHandle;
            RawDisplayHandle::Windows(WindowsDisplayHandle::new())
        }
        #[cfg(all(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly")))]
        {
            use raw_window_handle::XlibDisplayHandle;
            use std::ptr::NonNull;
            let display = NonNull::new(unsafe { glfw::ffi::glfwGetX11Display() });
            let handle = XlibDisplayHandle::new(display, 0);
            RawDisplayHandle::Xlib(handle)
        }
        #[cfg(target_os = "macos")]
        {
            use raw_window_handle::AppKitDisplayHandle;
            RawDisplayHandle::AppKit(AppKitDisplayHandle::new())
        }
        #[cfg(target_os = "emscripten")]
        {
            RawDisplayHandle::Web(raw_window_handle::WebDisplayHandle::new())
        }
    }
}



impl Drop for Window {
    fn drop(&mut self) {
        unsafe { glfw::ffi::glfwDestroyWindow(self.handle) };
    }
}
