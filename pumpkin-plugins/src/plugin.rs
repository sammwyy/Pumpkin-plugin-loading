pub trait Plugin {
    fn on_load(&self);
    fn on_unload(&self);
}
