use anyhow::{Ok, Result};
use ash::vk::{self, SurfaceCapabilitiesKHR};
use std::collections::HashSet;
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

pub struct RenderingContext {
    pub queues: Vec<vk::Queue>,
    pub device: ash::Device,
    pub swapchain_extension: ash::khr::swapchain::Device,
    pub queue_family_indices: HashSet<u32>,
    pub queue_families: QueueFamilies,
    pub physical_device: PhysicalDevice,
    pub surface_extension: ash::khr::surface::Instance,
    pub instance: ash::Instance,
    pub entry: ash::Entry,
}

#[derive(Debug, Clone)]
pub struct QueueFamily {
    pub index: u32,
    pub properties: vk::QueueFamilyProperties,
}

#[derive(Debug)]
pub struct PhysicalDevice {
    pub handle: vk::PhysicalDevice,
    pub properties: vk::PhysicalDeviceProperties,
    pub features: vk::PhysicalDeviceFeatures,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub queue_families: Vec<QueueFamily>,
}

type QueueFamilySelection = fn(Vec<PhysicalDevice>) -> Result<(PhysicalDevice, QueueFamilies)>;
pub struct RenderingContextAttributes<'window> {
    pub dummy_window: &'window Window,
    pub queue_family_selection: QueueFamilySelection,
}

pub struct QueueFamilies {
    pub graphics: u32,
    pub present: u32,
    pub transfer: u32,
    pub compute: u32,
}

pub mod queue_family_selection {
    use super::{PhysicalDevice, QueueFamilies};
    use anyhow::Context as AnyhowContext;
    use anyhow::Result;
    use ash::vk;

    pub fn single_queue_family(
        physical_devices: Vec<PhysicalDevice>,
    ) -> Result<(PhysicalDevice, QueueFamilies)> {
        let physical_device = physical_devices.into_iter().next().unwrap();
        let queue_family = physical_device
            .queue_families
            .iter()
            .find(|queue_family| {
                queue_family
                    .properties
                    .queue_flags
                    .contains(vk::QueueFlags::GRAPHICS)
                    && queue_family
                        .properties
                        .queue_flags
                        .contains(vk::QueueFlags::COMPUTE)
            })
            .map(|queue_family| queue_family.index)
            .context("No suitable queue family found")?;
        Ok((
            physical_device,
            QueueFamilies {
                graphics: queue_family,
                compute: queue_family,
                present: queue_family,
                transfer: queue_family,
            },
        ))
    }
}

impl RenderingContext {
    pub fn new(attributes: RenderingContextAttributes) -> Result<Self> {
        let entry = unsafe { ash::Entry::load()? };

        let raw_display_handle = attributes.dummy_window.display_handle()?.as_raw();
        let raw_window_handle = attributes.dummy_window.window_handle()?.as_raw();

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

        let dummy_surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                raw_display_handle,
                raw_window_handle,
                None,
            )?
        };

        let mut physical_devices = unsafe { instance.enumerate_physical_devices()? }
            .into_iter()
            .map(|handle| {
                let properties = unsafe { instance.get_physical_device_properties(handle) };
                let features = unsafe { instance.get_physical_device_features(handle) };
                let memory_properties =
                    unsafe { instance.get_physical_device_memory_properties(handle) };
                let queue_family_properties =
                    unsafe { instance.get_physical_device_queue_family_properties(handle) };

                let queue_families = queue_family_properties
                    .into_iter()
                    .enumerate()
                    .map(|(index, properties)| QueueFamily {
                        index: index as u32,
                        properties,
                    })
                    .collect::<Vec<_>>();

                PhysicalDevice {
                    handle,
                    properties,
                    features,
                    memory_properties,
                    queue_families,
                }
            })
            .collect::<Vec<_>>();

        physical_devices.retain(|device| unsafe {
            surface_extension
                .get_physical_device_surface_support(device.handle, 0, dummy_surface)
                .unwrap_or(false)
        });

        unsafe { surface_extension.destroy_surface(dummy_surface, None) };

        let (physical_device, queue_families) =
            (attributes.queue_family_selection)(physical_devices)?;

        let queue_family_indices = HashSet::from([
            queue_families.graphics,
            queue_families.present,
            queue_families.transfer,
            queue_families.compute,
        ]);

        let queue_create_infos = queue_family_indices
            .iter()
            .copied()
            .map(|index| {
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(index)
                    .queue_priorities(&[1.0])
            })
            .collect::<Vec<_>>();

        let device = unsafe {
            instance.create_device(
                physical_device.handle,
                &vk::DeviceCreateInfo::default()
                    .queue_create_infos(&queue_create_infos)
                    .enabled_extension_names(&[ash::khr::swapchain::NAME.as_ptr()])
                    .push_next(
                        &mut vk::PhysicalDeviceDynamicRenderingFeatures::default()
                            .dynamic_rendering(true),
                    )
                    .push_next(
                        &mut vk::PhysicalDeviceBufferDeviceAddressFeatures::default()
                            .buffer_device_address(true),
                    ),
                None,
            )?
        };

        let swapchain_extension = ash::khr::swapchain::Device::new(&instance, &device);

        let queues = queue_family_indices
            .iter()
            .map(|index| unsafe { device.get_device_queue(*index, 0) })
            .collect::<Vec<_>>();

        Ok(Self {
            queues,
            device,
            queue_family_indices,
            queue_families,
            physical_device,
            surface_extension,
            instance,
            entry,
            swapchain_extension,
        })
    }

    pub unsafe fn create_surface(&self, window: &Window) -> Result<Surface> {
        let raw_display_handle = window.display_handle()?.as_raw();
        let raw_window_handle = window.window_handle()?.as_raw();
        let handle = unsafe {
            ash_window::create_surface(
                &self.entry,
                &self.instance,
                raw_display_handle,
                raw_window_handle,
                None,
            )?
        };

        let capabilities = unsafe {
            self.surface_extension
                .get_physical_device_surface_capabilities(self.physical_device.handle, handle)
        }?;

        let formats = unsafe {
            self.surface_extension
                .get_physical_device_surface_formats(self.physical_device.handle, handle)
        }?;

        let present_modes = unsafe {
            self.surface_extension
                .get_physical_device_surface_present_modes(self.physical_device.handle, handle)
        }?;

        Ok(Surface {
            handle,
            capabilities,
            formats,
            present_modes,
        })
    }
    pub fn create_image_view(
        &self,
        image: vk::Image,
        format: vk::Format,
        aspect_flags: vk::ImageAspectFlags,
    ) -> Result<vk::ImageView> {
        let image_view = unsafe {
            self.device.create_image_view(
                &vk::ImageViewCreateInfo::default()
                    .image(image)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(format)
                    .components(vk::ComponentMapping::default())
                    .subresource_range(
                        vk::ImageSubresourceRange::default()
                            .aspect_mask(aspect_flags)
                            .base_mip_level(0)
                            .level_count(1)
                            .base_array_layer(0)
                            .layer_count(1),
                    ),
                None,
            )
        }?;
        Ok(image_view)
    }
}

pub struct Surface {
    pub handle: vk::SurfaceKHR,
    pub capabilities: SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl Drop for RenderingContext {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.instance.destroy_instance(None)
        };
    }
}
