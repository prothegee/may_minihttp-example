// src/plugins/core.rs
use std::sync::OnceLock;
use toml::Value;
use super::{plugin::PluginProjectDir, traits::TraitPlugin};

static CORE_PLUGIN: OnceLock<CorePlugin> = OnceLock::new();

pub struct CorePlugin;

impl CorePlugin {
    pub fn init_and_start(config: &'static Value) -> Result<(), String> {
        PluginProjectDir::initialize(config)?;
        
        CORE_PLUGIN.set(CorePlugin)
            .map_err(|_| "CorePlugin already initialized".to_string())
    }

    #[allow(dead_code)]
    pub fn instance() -> &'static Self {
        CORE_PLUGIN.get().expect("CorePlugin not initialized")
    }
}

impl TraitPlugin for CorePlugin {
    fn initialize(config: &'static Value) -> Result<(), String> {  // Match 'static lifetime
        Self::init_and_start(config)
    }

    fn name() -> &'static str {
        "CorePlugin"
    }

    fn shutdown(&self) {
        // shutdown logic
    }
}