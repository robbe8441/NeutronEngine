use std::{ffi::c_char, sync::Arc};

use crate::utils::ConvertCStr;

use anyhow::Result;
use ash::{ext::debug_utils, vk};


pub struct Instance {
    handle: ash::Instance,
    entry: ash::Entry,
}

impl Instance {
    pub fn new() -> Result<Arc<Self>> {
        Self::from_extensions(vec![debug_utils::NAME.as_ptr()])
    }

    pub fn from_display_handle(display_handle: raw_window_handle::RawDisplayHandle) -> Result<Arc<Self>> {
        let mut extension_names = ash_window::enumerate_required_extensions(display_handle)
            .unwrap()
            .to_vec();
        extension_names.push(debug_utils::NAME.as_ptr());

        Self::from_extensions(extension_names)
    }

    pub fn from_extensions(names: Vec<*const c_char>) -> Result<Arc<Self>> {
        let entry = unsafe { ash::Entry::load() }?;

        let application_info = vk::ApplicationInfo::default()
            .engine_name("Neutron Engine\n".to_cstr_unchecked())
            .engine_version(0)
            .api_version(vk::API_VERSION_1_3);

        let create_info = vk::InstanceCreateInfo::default()
            .enabled_extension_names(&names)
            .application_info(&application_info);

        let instance = unsafe { entry.create_instance(&create_info, None) }?;

        Ok(Self {
            entry,
            handle: instance,
        }
        .into())
    }

    pub fn as_raw(&self) -> &ash::Instance {
        &self.handle
    }
    pub fn entry(&self) -> &ash::Entry {
        &self.entry
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy_instance(None);
        };
    }
}
