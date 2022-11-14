use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait PluginTrait: Send + Debug + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn initialized(&self) -> bool;
    fn text_process(&self, input: &str) -> String {
        input.to_string()
    }
    async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
