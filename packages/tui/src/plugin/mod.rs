pub struct PluginRegistry;
impl PluginRegistry {
    pub fn new() -> Self {
        Self
    }
}
impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
