use pumpkin_plugins::plugin::Plugin;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn on_load(&self) {
        println!("Hello World uwu");
    }

    fn on_unload(&self) {
        todo!()
    }
}

#[no_mangle]
pub extern "C" fn plugin_entry_point() -> Box<dyn Plugin> {
    Box::new(TestPlugin)
}
