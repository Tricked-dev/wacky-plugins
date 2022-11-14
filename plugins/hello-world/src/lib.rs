use plugin_lib::PluginTrait;

#[derive(Debug)]
pub struct Plugin;

impl Plugin {
    pub fn new() -> Self {
        Self
    }
}

impl PluginTrait for Plugin {
    fn name(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
