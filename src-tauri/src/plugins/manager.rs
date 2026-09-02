use super::PluginManifest;
use std::path::PathBuf;
use std::fs;

pub struct PluginManager {
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

    pub fn discover_plugins(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.manifests.clear();
        
        if !self.plugins_dir.exists() {
            fs::create_dir_all(&self.plugins_dir)?;
            return Ok(());
        }
        
        let entries = fs::read_dir(&self.plugins_dir)?;
        
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("manifest.toml");
                if manifest_path.exists() {
                    if let Ok(content) = fs::read_to_string(&manifest_path) {
                        if let Ok(manifest) = toml::from_str::<PluginManifest>(&content) {
                            self.manifests.push(manifest);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    pub fn get_plugins(&self) -> &[PluginManifest] {
        &self.manifests
    }

    pub fn load_plugin(&self, manifest: &PluginManifest) -> Result<(), Box<dyn std::error::Error>> {
        // Validate permissions before loading
        self.validate_permissions(&manifest.permissions)?;
        
        // Plugin loading logic would go here
        // For now, just validate the plugin exists
        let plugin_dir = self.plugins_dir.join(&manifest.id);
        if !plugin_dir.exists() {
            return Err(format!("Plugin directory not found: {}", manifest.id).into());
        }
        
        Ok(())
    }

    fn validate_permissions(&self, permissions: &super::PluginPermissions) -> Result<(), Box<dyn std::error::Error>> {
        // Check if required permissions are valid
        let valid_permissions = vec![
            "fs:read", "fs:write", "network:https", "network:http",
            "database:read", "database:write", "audio:play", "audio:record",
            "ui:overlay", "ui:notification",
        ];
        
        for perm in &permissions.requires {
            if !valid_permissions.contains(&perm.as_str()) {
                return Err(format!("Invalid required permission: {}", perm).into());
            }
        }
        
        for perm in &permissions.optional {
            if !valid_permissions.contains(&perm.as_str()) {
                return Err(format!("Invalid optional permission: {}", perm).into());
            }
        }
        
        Ok(())
    }

    pub fn install_plugin(&mut self, plugin_data: &[u8], manifest: PluginManifest) -> Result<(), Box<dyn std::error::Error>> {
        let plugin_dir = self.plugins_dir.join(&manifest.id);
        fs::create_dir_all(&plugin_dir)?;
        
        // Save manifest
        let manifest_content = toml::to_string(&manifest)?;
        fs::write(plugin_dir.join("manifest.toml"), manifest_content)?;
        
        // Save plugin binary (would be WASM or native)
        fs::write(plugin_dir.join("plugin.wasm"), plugin_data)?;
        
        self.manifests.push(manifest);
        
        Ok(())
    }

    pub fn uninstall_plugin(&mut self, plugin_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let plugin_dir = self.plugins_dir.join(plugin_id);
        if plugin_dir.exists() {
            fs::remove_dir_all(&plugin_dir)?;
        }
        
        self.manifests.retain(|m| m.id != plugin_id);
        
        Ok(())
    }
}
