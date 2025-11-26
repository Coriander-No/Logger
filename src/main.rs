
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
    //    egui::CentralPanel::default().show(ctx, |ui| {
    //        ui.heading("Hello World!");
    //    });
        // 渲染主 UI
        render_ui(ctx, &mut self.state, &mut self.config);


        // ctx.input(|input|{
        //     for event in &input.events {
        //         match event {
        //             egui::Event::Key { key, pressed, .. } => {
        //                 if *pressed {
        //                     match key {
        //                         egui::Key::ArrowLeft => self.state.move_cursor(Direction::Left),
        //                         egui::Key::ArrowRight => self.state.move_cursor(Direction::Right),
        //                         egui::Key::ArrowUp => self.state.move_cursor(Direction::Up),
        //                         egui::Key::ArrowDown => self.state.move_cursor(Direction::Down),
        //                         _ => {}
        //                     }
        //                 }
        //             }
        //             _ => {}
        //         }
        //     }
        // })
   }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.centered=true;
    let _ = eframe::run_native("Logger", native_options, Box::new(|_cc| Box::new(LoggerApp::default())));
}
