use rfd::FileDialog;
use std::path::PathBuf;
use std::fs;
use crate::Logger::{LoggerState, LoggerCursor, LoggerSelection};
use crate::error::EditorError;

// 新建文件
pub fn new_file(state: &mut LoggerState) {
    state.lines = vec!["".to_string()];
    state.cursor = LoggerCursor::new(0, 0);
    state.selection = LoggerSelection::default();
    state.file_path = None;
}

// 打开文件（通过文件对话框选择）
pub fn open_file(state: &mut LoggerState) -> Result<(), EditorError> {
    if let Some(path) = FileDialog::new().pick_file() {
        // 读取文件内容（UTF-8 编码）
        let content = fs::read_to_string(&path)?;
        // 按换行分割为多行
        let lines = content.split('\n').map(|s| s.to_string()).collect();
        // 更新编辑器状态
        state.lines = lines;
        state.cursor = LoggerCursor::new(0, 0);
        state.selection = LoggerSelection::default();
        state.file_path = Some(path);
    }
    Ok(())
}

// 保存文件（已打开文件直接保存，未打开文件弹出另存为对话框）
pub fn save_file(state: &mut LoggerState) -> Result<(), EditorError> {
    if let Some(path) = &state.file_path {
        // 已打开文件：直接覆盖
        let content = state.lines.join("\n");
        fs::write(path, content)?;
    } else {
        // 未打开文件：另存为
        if let Some(path) = FileDialog::new().save_file() {
            let content = state.lines.join("\n");
            fs::write(&path, content)?;
            state.file_path = Some(path);
        }
    }
    Ok(())
}

// 另存为文件
pub fn save_as_file(state: &mut LoggerState) -> Result<(), EditorError> {
    if let Some(path) = FileDialog::new().save_file() {
        let content = state.lines.join("\n");
        fs::write(&path, content)?;
        state.file_path = Some(path);
    }
    Ok(())
}