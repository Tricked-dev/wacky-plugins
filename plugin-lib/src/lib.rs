use std::fmt::Debug;

pub trait PluginTrait: Send + Debug + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
}
