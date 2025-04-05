use std::sync::Arc;

use anyhow::{Ok, Result};
use ash::vk;
use winit::window::Window;

use crate::app::engine::rendering_context::{RenderingContext, Surface};

pub struct Swapchain {
    pub desired_image_count: u32,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
    image_views: Vec<vk::ImageView>,
    images: Vec<vk::Image>,
    handle: vk::SwapchainKHR,
    surface: Surface,
    window: Arc<Window>,
    context: Arc<RenderingContext>,
}

impl Swapchain {
    pub fn new(context: Arc<RenderingContext>, window: Arc<Window>) -> Result<Self> {
        let surface = unsafe { context.create_surface(window.as_ref())? };
        let format = vk::Format::B8G8R8A8_SRGB;
        let extent = if surface.capabilities.current_extent.width != u32::MAX {
            surface.capabilities.current_extent
        } else {
            let size = window.inner_size();
            vk::Extent2D {
                width: size.width,
                height: size.height,
            }
        };
        let desired_image_count = (surface.capabilities.min_image_count + 1).clamp(
            surface.capabilities.min_image_count,
            if surface.capabilities.max_image_count == 0 {
                u32::MAX
            } else {
                surface.capabilities.max_image_count
            },
        );

        Ok(Self {
            desired_image_count,
            format,
            extent,
            image_views: vec![],
            images: vec![],
            handle: Default::default(),
            surface,
            window,
            context,
        })
    }

    pub fn resize(&mut self) -> Result<()> {
        let size = self.window.inner_size();
        self.extent = vk::Extent2D {
            width: size.width,
            height: size.height,
        };

        if self.extent.width == 0 || self.extent.height == 0 {
            return Ok(());
        }

        let new_swapchain = unsafe {
            self.context.swapchain_extension.create_swapchain(
                &vk::SwapchainCreateInfoKHR::default()
                    .surface(self.surface.handle)
                    .min_image_count(self.desired_image_count)
                    .image_format(self.format)
                    .image_color_space(vk::ColorSpaceKHR::SRGB_NONLINEAR)
                    .image_extent(self.extent)
                    .image_array_layers(1)
                    .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                    .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                    .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
                    .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
                    .present_mode(vk::PresentModeKHR::FIFO)
                    .clipped(true)
                    .old_swapchain(self.handle),
                None,
            )?
        };

        unsafe {
            self.image_views.drain(..).for_each(|image_view| {
                self.context.device.destroy_image_view(image_view, None);
            });

            self.images.clear();

            self.context
                .swapchain_extension
                .destroy_swapchain(self.handle, None);

            self.handle = new_swapchain;
            self.images = self
                .context
                .swapchain_extension
                .get_swapchain_images(self.handle)?;
            for image in &self.images {
                self.image_views.push(self.context.create_image_view(
                    *image,
                    self.format,
                    vk::ImageAspectFlags::COLOR,
                )?);
            }
        }

        Ok(())
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            self.image_views.drain(..).for_each(|image_view| {
                self.context.device.destroy_image_view(image_view, None);
            });
            self.context
                .swapchain_extension
                .destroy_swapchain(self.handle, None);
        }
    }
}
