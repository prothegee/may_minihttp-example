// src/plugins/plugin.rs
use std::path::Path;
use toml::Value;
use super::traits::TraitPlugin;

#[allow(dead_code)]
pub struct PluginProjectDir {
    public_path: String,
    upload_path: String,
}

impl PluginProjectDir {
    pub fn new(public_path: String, upload_path: String) -> Self {
        Self {
            public_path,
            upload_path,
        }
    }

    fn create_dir(&self, path: &str) -> Result<(), String> {
        let path = Path::new(path);
        if !path.exists() {
            std::fs::create_dir_all(path)
                .map_err(|e| format!("Failed to create {}: {}", path.display(), e))?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_public_path(&self) -> &str {
        &self.public_path
    }
    
    #[allow(dead_code)]
    pub fn get_upload_path(&self) -> &str {
        &self.upload_path
    }
}

impl TraitPlugin for PluginProjectDir {
    fn initialize(config: &Value) -> Result<(), String> {
        let public_path = config["config"]["dir_public_path"]
            .as_str()
            .ok_or("Missing dir_public_path in config")?
            .to_string();

        let upload_path = config["config"]["dir_public_upload_path"]
            .as_str()
            .ok_or("Missing dir_public_upload_path in config")?
            .to_string();

        let plugin = Self::new(public_path.clone(), upload_path.clone());
        
        // create base directories
        plugin.create_dir(&public_path)?;
        plugin.create_dir(&upload_path)?;

        // create extra directories
        if let Some(extra_paths) = config["extra"]["dir_public_upload_extra_paths"].as_array() {
            for path in extra_paths {
                let path_str = path.as_str().ok_or("Invalid path in extra_paths")?;
                let full_path = Path::new(&upload_path).join(path_str);
                plugin.create_dir(full_path.to_str().unwrap())?;
            }
        }

        Ok(())
    }

    fn name() -> &'static str {
        "ProjectDirPlugin"
    }

    fn shutdown(&self) {
        // Cleanup logic if needed
    }
}