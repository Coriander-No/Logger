use unicode_segmentation::UnicodeSegmentation;
use std::cmp;
use std::path::PathBuf;
// use egui::epaint::text::cursor;
use egui::text::CCursor;
use egui::epaint::{Color32, text::{LayoutJob, TextFormat}, FontFamily, FontId};

//选中范围（start ≤ end）
// #[derive(Clone, Debug)]
// pub struct LoggerSelection {
//     pub start: cursor::CCursor,
//     pub end: cursor::CCursor,
//     pub is_selected: bool,
// }

// impl Default for LoggerSelection {
//     fn default() -> Self {
//         Self {
//             start: cursor::CCursor::default(),
//             end: cursor::CCursor::default(),
//             is_selected: false,
//         }
//     }
//     // pub fn new(start: LoggerCursor, end: LoggerCursor) -> Self {
//     //     Self { start, end }
//     // }
// }

// Log核心状态
#[derive(Default,Clone, Debug)]
pub struct LoggerState {
    pub lines: Vec<String>,     //文本内容
    pub cursor: Option<CCursor>,
    // pub selection: LoggerSelection,
    pub file_path: Option<PathBuf>,
}

impl LoggerState {
    pub fn new(lines: Vec<String>, cursor: Option<CCursor>, file_path: Option<PathBuf>) -> Self {
        Self {
            lines,
            cursor,
            // selection: LoggerSelection::default(),
            file_path,
        }
    }

    pub fn default(self) -> Self {
        Self {
            lines: vec![],
            cursor: None,
            // selection: LoggerSelection::default(),
            file_path: None,
        }
    }

    pub fn get_cursor(&self) -> Option<CCursor> {
        self.cursor
    }

    pub fn set_cursor(&mut self, cursor: Option<CCursor>) {
        self.cursor = cursor;
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        if let Some(cursor) = self.cursor.as_mut() {
            cursor.move_cursor(direction);
        }
    }
}




// 光标移动方向
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
