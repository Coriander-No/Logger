use unicode_segmentation::UnicodeSegmentation;
use std::cmp;
use std::path::PathBuf;
// use epaint::text::cursor::Cursor;
// use epaint::{Color32, text::{LayoutJob, TextFormat}, FontFamily, FontId};


// Log核心状态
#[derive(Default,Clone, Debug)]
pub struct LoggerState {
    pub lines: Vec<String>,     //文本内容
    pub file_path: Option<PathBuf>,
}

impl LoggerState {
    pub fn new(lines: Vec<String>, file_path: Option<PathBuf>) -> Self {
        Self {
            lines,
            file_path,
        }
    }

    pub fn default() -> Self {
        Self {
            lines: vec![],
            file_path: None,
        }
    }
}

// pub struct LoggerConfig {
//     pub log_level: LogLevel,
// }

// 光标移动方向
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
