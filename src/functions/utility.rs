#[allow(non_snake_case)]
pub mod toml {
    use toml as toml_crate;
    use std::sync::OnceLock;

    static CONFIG: OnceLock<toml_crate::Value> = OnceLock::new();

    /**
    ### BRIEF
    load toml from filePath param and get data

    ### args
    * `filePath` - a relative file of .toml from executeable program

    ### return
    `toml::Table` - the parsed TOML data as `toml::Table`
    */
    pub fn fromFile(filePath: &str) -> &'static toml_crate::Value {
        CONFIG.get_or_init(|| {
            let config_content = std::fs::read_to_string(filePath)
                .expect(format!("failed to read {}", filePath).as_str());
            config_content.parse().expect(format!("failed to parse {}", filePath).as_str())
        })
    }
}