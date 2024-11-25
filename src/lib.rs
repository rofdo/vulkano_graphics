use vulkano::{instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, VulkanLibrary};
use std::{error::Error, sync::Arc};

fn get_instance() -> Result<Arc<Instance>, Box<dyn Error>> {
    let library = VulkanLibrary::new()?;
    let instance = Instance::new(
        library,
        InstanceCreateInfo{
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    )
    ?;
    Ok(instance)
}

pub fn create_window() -> Result<(),Box<dyn Error>> {
    let instance = get_instance()?;
    Ok(())
}