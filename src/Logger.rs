use unicode_segmentation::UnicodeSegmentation;
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

//光标位置
pub struct LoggerCursor {
    pub line: usize,
    pub col: usize,
}

impl LoggerCursor {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}


// 选中范围（start ≤ end）
pub struct LoggerSelection {
    pub start: LoggerCursor,
    pub end: LoggerCursor,
}

impl LoggerSelection {
    pub fn new(start: LoggerCursor, end: LoggerCursor) -> Self {
        Self { start, end }
    }
}

// Log核心状态
#[derive(Debug, Clone)]
pub struct LoggerState {
    pub lines: Vec<String>,     //文本内容
    pub cursor: LoggerCursor,
    pub selection: LoggerSelection,
    pub file_path: Option<PathBuf>,
}

pub struct LoggerConfig {
    pub log_level: LogLevel,
}