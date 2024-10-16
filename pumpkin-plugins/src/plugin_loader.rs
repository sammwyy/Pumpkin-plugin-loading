use crate::plugin::Plugin;
use libloading::{Library, Symbol};
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

pub struct PluginLoader {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn load_plugin<P: AsRef<OsStr>>(&mut self, path: P) {
        let lib = unsafe { Library::new(path).expect("Failed to load plugin") };
        unsafe {
            let plugin_entry_point: Symbol<fn() -> Box<dyn Plugin>> = lib
                .get(b"plugin_entry_point")
                .expect("Failed to find plugin entry point");

            let plugin = plugin_entry_point();
            plugin.on_load();
            self.plugins.push(plugin);
        }
    }

    pub fn load_plugins_from_directory<P: AsRef<Path>>(&mut self, dir: P) {
        let path = dir.as_ref();

        if !path.exists() {
            fs::create_dir_all(path).expect("Failed to create plugins directory");
        }

        for entry in fs::read_dir(path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();

            if self.is_valid_plugin(&path) {
                log::info!("Loading plugin: {:?}", path.file_name().unwrap());
                self.load_plugin(path.as_os_str());
            }
        }
    }

    fn is_valid_plugin(&self, path: &PathBuf) -> bool {
        if let Some(extension) = path.extension() {
            return extension == "so"
                || extension == "dll"
                || extension == "dylib"
                || extension == "plugin";
        }
        false
    }

    pub fn get_plugins(&self) -> &Vec<Box<dyn Plugin>> {
        &self.plugins
    }
}
