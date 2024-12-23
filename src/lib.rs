use std::{error::Error, sync::Arc};
use vulkano::{
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    VulkanLibrary,
};

fn get_instance() -> Result<Arc<Instance>, Box<dyn Error>> {
    let library = VulkanLibrary::new()?;
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    )?;
    Ok(instance)
}

pub fn create_window() -> Result<(), Box<dyn Error>> {
    let instance = get_instance()?;
    let mut physical_devices = instance.enumerate_physical_devices()?;
    let physical_device = physical_devices.next().ok_or("No physical devices")?;
    Ok(())
}

