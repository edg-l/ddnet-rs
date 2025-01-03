use std::sync::{Arc, RwLock};

use ash::vk;
use hiarc::Hiarc;

use super::{
    frame_resources::FrameResources, image::Image, logical_device::LogicalDevice,
    memory_block::DeviceMemoryBlock, vulkan_mem::ImageAllocationError,
};

#[derive(Debug, Hiarc)]
pub struct ImageView {
    #[hiarc_skip_unsafe]
    image_view: vk::ImageView,

    _bound_memory: RwLock<Option<Arc<DeviceMemoryBlock>>>,

    pub image: Arc<Image>,
    device: Arc<LogicalDevice>,
}

impl ImageView {
    pub fn new(
        device: Arc<LogicalDevice>,
        image: Arc<Image>,
        create_info: vk::ImageViewCreateInfo,
    ) -> anyhow::Result<Arc<Self>, ImageAllocationError> {
        let image_view = unsafe { device.device.create_image_view(&create_info, None) }?;
        Ok(Arc::new(Self {
            image_view,
            _bound_memory: Default::default(),

            image,
            device,
        }))
    }

    pub fn view(self: &Arc<Self>, frame_resources: &mut FrameResources) -> vk::ImageView {
        frame_resources.image_views.push(self.clone());

        self.image_view
    }
}

impl Drop for ImageView {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_image_view(self.image_view, None) };
    }
}
