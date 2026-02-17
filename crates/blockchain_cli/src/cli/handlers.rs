use blockchain_core::config::NodeConfig;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct FileConfig {
    difficulty: Option<usize>,
}

pub fn load_config(path: Option<&str>) -> Result<NodeConfig, String> {
    let mut config = NodeConfig::default();

    if let Some(path) = path {
        let raw = fs::read_to_string(path)
            .map_err(|e| format!("не удалось прочитать файл {path}: {e}"))?;
        let file_config: FileConfig =
            toml::from_str(&raw).map_err(|e| format!("невалидный TOML в {path}: {e}"))?;

        if let Some(difficulty) = file_config.difficulty {
            config = NodeConfig::new(difficulty);
        }
    }

    Ok(config)
}

pub fn handle_init(config: &NodeConfig) {
    println!("init: difficulty={}", config.difficulty);
}

pub fn handle_add_tx(config: &NodeConfig) {
    println!("add-tx: difficulty={}", config.difficulty);
}

pub fn handle_mine(config: &NodeConfig) {
    println!("mine: difficulty={}", config.difficulty);
}

pub fn handle_print(config: &NodeConfig) {
    println!("print: difficulty={}", config.difficulty);
}
