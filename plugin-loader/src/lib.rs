use once_cell::sync::Lazy;
use parking_lot::Mutex;
use plugin_lib::PluginTrait;
use plugin_shim::load_plugins;

pub struct PluginTegistry {
    pub plugins: Mutex<Vec<Box<dyn PluginTrait>>>,
}

impl PluginTegistry {
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut plugins = self.plugins.lock();
        for plugin in plugins.iter_mut() {
            plugin.initialize().await?;
        }
        Ok(())
    }
}
pub static PLUGIN_REGISTRY: Lazy<PluginTegistry> = Lazy::new(|| PluginTegistry {
    plugins: Mutex::new(load_plugins()),
});
