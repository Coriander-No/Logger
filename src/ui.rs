use egui::{Context, Layout, RichText, ScrollArea, Separator, TextEdit, TopBottomPanel, Ui, Widget, Align, TextBuffer};
// use crate::syntax::{get_syntax_for_file, get_theme_formats, highlight_line};
use egui::Color32;
use crate::Logger::{LoggerState,Direction};
use std::time::Duration;
use crate::config::LoggerConfig;
use crate::file::{new_file, open_file, save_file, save_as_file};
use crate::error::EditorError;

pub fn render_ui(ctx: &Context, state: &mut LoggerState, config: &mut LoggerConfig) {

    TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("file", |ui| {
                if ui.button("new").clicked() {
                    new_file(state);
                 }
                if ui.button("open").clicked() {
                    if let Err(e) = open_file(state) {
                        show_error_toast(ctx, &e.to_string());
                    }
                 }
                if ui.button("save").clicked() {

                 }
            });

            ui.menu_button("editor", |ui| {
                if ui.button("copy").clicked() { /* ... */ }
            });
        });
    });

    // 渲染状态栏
    egui::TopBottomPanel::bottom("Status Bar").show(ctx, |ui| {
        ui.vertical_centered(|ui|{
            ui.heading("Status Bar");
        });
    });

    // 渲染侧边栏
    egui::SidePanel::left("Side Bar")
    .resizable(true)
    .default_width(150.0)
    .width_range(80.0..=300.0)
    .show(ctx, |ui| {
        ui.vertical_centered(|ui|{
            ui.label("Side Bar");
            ui.label("Side Bar");
            ui.label("Side Bar");
            ui.label("Side Bar");

        });
    });

    // 渲染主编辑区域
    egui::CentralPanel::default().show(ctx, |ui|{
        // 滚动区域
        egui::ScrollArea::both()
        .show(ui, |ui|{
            let mut text_buffer = String::new();
            for line in state.lines.iter(){
                let line_str: &str = line.as_str();
                text_buffer.push_str(line_str);
                text_buffer.push('\n');
            }

            let mut text_edit = egui::TextEdit::multiline(&mut text_buffer)          // 设置默认显示行数
                    .font(egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows((state.lines.len() as u32).try_into().unwrap())
                    .lock_focus(true)
                    .background_color(Color32::from_rgba_unmultiplied(240, 240, 240, 0))
                    .desired_width(f32::INFINITY);
            if text_edit.ui(ui).changed() {
                state.lines = text_buffer.lines().map(|line| line.to_string()).collect();
            }
        });
    });
}

fn show_error_toast(ctx: &Context, msg: &str) {

}