#![allow(dead_code)]

pub trait TraitPlugin {
    fn initialize(config: &'static toml::Value) -> Result<(), String>;
    fn name() -> &'static str;
    fn shutdown(&self);
}