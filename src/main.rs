use plugin_loader::PLUGIN_REGISTRY;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    PLUGIN_REGISTRY.initialize().await?;

    for plugin in PLUGIN_REGISTRY.plugins.lock().iter() {
        println!(
            "Plugin: {} initialized: {}",
            plugin.name(),
            plugin.initialized()
        );
    }
    let input = "This is cool text!";

    for plugin in PLUGIN_REGISTRY.plugins.lock().iter() {
        println!(
            "Plugin: {} processed text: {}",
            plugin.name(),
            plugin.text_process(input)
        );
    }

    Ok(())
}
