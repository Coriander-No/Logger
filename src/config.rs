use serde::{Deserialize, Serialize};
use dirs::config_dir;
use std::path::PathBuf;
use std::fs;
use crate::error::EditorError;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggerConfig {
    pub font_size: f32,          // 字体大小（默认 14px）
    pub theme: String,           // 主题（默认 dark）
    pub show_line_numbers: bool, // 是否显示行号（默认 true）
    pub word_wrap: bool,         // 是否自动换行（默认 true）
}

impl LoggerConfig {
    // 加载配置（优先读取本地文件，无则使用默认）
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if let Ok(content) = fs::read_to_string(&config_path) {
            toml::from_str(&content).unwrap_or_default()
        } else {
            let default = Self::default();
            // 保存默认配置到本地
            let _ = default.save();
            default
        }
    }

    // 保存配置到本地
    pub fn save(&self) -> Result<(), EditorError> {
        let config_path = Self::config_path();
        // 创建配置目录（若不存在）
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let content = toml::to_string(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    // 配置文件路径（系统配置目录 + rust-text-editor/config.toml）
    fn config_path() -> PathBuf {
        config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rust-text-editor")
            .join("config.toml")
    }
}