
use eframe::egui;
use std::default;
#[warn(unused_imports)]
use std::path::Path;
use crate::Logger::{LoggerState,Direction};
use crate::ui::render_ui;
use crate::config::LoggerConfig;
// use crate::error::EditorError;

mod Logger;
// mod syntax;
mod file;
mod ui;
mod config;
mod error;

// struct TextEditorApp {
//     state: EditorState,
//     config: EditorConfig,
// }
struct LoggerApp {
    state: LoggerState,
    config: LoggerConfig,
}

impl Default for LoggerApp {
    fn default() -> Self {
        Self {
            state: LoggerState::default(),
            config: LoggerConfig::load(),
        }
    }
}


impl eframe::App for LoggerApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        render_ui(ctx, &mut self.state, &mut self.config);
   }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.centered=true;
    let _ = eframe::run_native("Logger", native_options, Box::new(|_cc| Ok(Box::new(LoggerApp::default()))));
}
