use std::time::Duration;

use async_trait::async_trait;
use plugin_lib::PluginTrait;

#[derive(Debug)]
pub struct Plugin(bool);

impl Plugin {
    pub const fn new() -> Self {
        Self(false)
    }
}

#[async_trait]
impl PluginTrait for Plugin {
    fn name(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    fn initialized(&self) -> bool {
        self.0
    }
    fn text_process(&self, input: &str) -> String {
        format!("{} {}", input, "bye World!")
    }
    async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{} {} initializing...", self.name(), self.version());
        self.0 = true;
        std::thread::sleep(Duration::from_secs(2));
        Ok(())
    }
}
