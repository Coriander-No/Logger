use thiserror::Error;

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("文件操作失败：{0}")]
    FileError(#[from] std::io::Error),

    #[error("文件编码错误（仅支持 UTF-8）")]
    EncodingError,

    #[error("语法高亮初始化失败：{0}")]
    SyntaxError(String),

    #[error("配置文件读写失败：{0}")]
    ConfigError(#[from] toml::de::Error),

    #[error("配置文件序列化失败：{0}")]
    ConfigSerializeError(#[from] toml::ser::Error),
}

// 实现从 syntect 错误到自定义错误的转换
impl From<syntect::Error> for EditorError {
    fn from(e: syntect::Error) -> Self {
        EditorError::SyntaxError(e.to_string())
    }
}