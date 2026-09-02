use super::PluginManifest;
use std::path::PathBuf;

pub struct PluginManager {
    #[allow(dead_code)]
    plugins_dir: PathBuf,
    manifests: Vec<PluginManifest>,
}

impl PluginManager {
    pub fn new(plugins_dir: PathBuf) -> Self {
        PluginManager {
            plugins_dir,
            manifests: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn discover_plugins(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Scan plugins directory and load manifests
        Ok(())
    }

    pub fn get_plugins(&self) -> &[PluginManifest] {
        &self.manifests
    }

    #[allow(dead_code)]
    pub fn load_plugin(&self, _manifest: &PluginManifest) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Load and initialize plugin
        Ok(())
    }
}
