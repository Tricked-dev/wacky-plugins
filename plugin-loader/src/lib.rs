use once_cell::sync::Lazy;
use plugin_lib::PluginTrait;
use plugin_shim::load_plugins;

pub struct PluginTegistry {
    pub plugins: Vec<Box<dyn PluginTrait>>,
}
pub static PLUGIN_REGISTRY: Lazy<PluginTegistry> = Lazy::new(|| PluginTegistry {
    plugins: load_plugins(),
});
