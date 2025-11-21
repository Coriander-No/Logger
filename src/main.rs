
use eframe::egui;
#[warn(unused_imports)]
use std::path::Path;
// use crate::editor::EditorState;
// use crate::config::EditorConfig;
// use crate::ui::render_ui;
// use crate::error::EditorError;

// mod editor;
// mod syntax;
// mod file;
// mod ui;
// mod config;
// mod error;

// struct TextEditorApp {
//     state: EditorState,
//     config: EditorConfig,
// }
#[derive(Default)]
struct LoggerApp {
    // state: LoggerState,
    // config: LoggerConfig,
}

impl LoggerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for LoggerApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.centered=true;
    let _ = eframe::run_native("Logger", native_options, Box::new(|cc| Box::new(LoggerApp::new(cc))));
}
