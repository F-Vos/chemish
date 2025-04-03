use anyhow::Result;
use ash::vk;
use std::sync::Arc;
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

pub struct Context {
    surface: vk::SurfaceKHR,
    surface_extension: ash::khr::surface::Instance,
    instance: ash::Instance,
    entry: ash::Entry,
    window: Arc<Window>,
}

pub struct PhysicalDevice {
    handle: vk::PhysicalDevice,
    properties: vk::PhysicalDeviceProperties,
    features: vk::PhysicalDeviceFeatures,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
}

impl Context {
    pub fn new(window: Arc<Window>) -> Result<Self> {
        let entry = unsafe { ash::Entry::load()? };

        let raw_display_handle = window.display_handle()?.as_raw();
        let raw_window_handle = window.window_handle()?.as_raw();

        let extensions = ash_window::enumerate_required_extensions(raw_display_handle)?;

        let instance = unsafe {
            entry.create_instance(
                &vk::InstanceCreateInfo::default()
                    .application_info(
                        &vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_3),
                    )
                    .enabled_extension_names(extensions),
                None,
            )?
        };

        let surface_extension = ash::khr::surface::Instance::new(&entry, &instance);

        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                raw_display_handle,
                raw_window_handle,
                None,
            )?
        };

        let physical_devices = unsafe { instance.enumerate_physical_devices()? };

        dbg!(physical_devices);

        Ok(Self {
            surface,
            surface_extension,
            instance,
            entry,
            window,
        })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { self.surface_extension.destroy_surface(self.surface, None) };
    }
}
