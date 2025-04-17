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
    pub fn from_file(file_path: &str) -> &'static toml_crate::Value {
        CONFIG.get_or_init(|| {
            let config_content = std::fs::read_to_string(file_path)
                .expect(format!("failed to read {}", file_path).as_str());
            config_content.parse().expect(format!("failed to parse {}", file_path).as_str())
        })
    }
}

pub mod sanitize {
    /**
    ### brief
    sanitizes a filename by removing path traversal components and invalid characters
    */
    pub fn file_name(name: &str) -> String {
        name.chars()
            .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'))
            .collect()
    }
}