use plugin_loader::PLUGIN_REGISTRY;

fn main() {
    for plug in &PLUGIN_REGISTRY.plugins {
        println!("{} {}", plug.name(), plug.version());
    }
    // println!("{:?}", load_plugins())
}
