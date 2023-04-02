use anyhow::anyhow;
use single_instance::SingleInstance;

pub fn get_instance() -> anyhow::Result<SingleInstance> {
    let instance = SingleInstance::new("mouse_actions")?;

    if instance.is_single() {
        Ok(instance)
    } else {
        Err(anyhow!("another instance is running !"))
    }
}
