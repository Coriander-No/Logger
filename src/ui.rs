use egui::{Context, Layout, RichText, ScrollArea, Separator, TextEdit, TopBottomPanel, Ui, Widget, Align, TextBuffer};
// use crate::syntax::{get_syntax_for_file, get_theme_formats, highlight_line};
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
        egui::SidePanel::left("Line Number")
        .resizable(false)
        .default_width(15.0)
        .show(ctx, |ui|{
            ui.vertical_centered(|ui|{
                ui.label("number");
            });
        });
        // 右侧文本编辑区
        egui::CentralPanel::default().show(ctx, |ui|{
            let mut text = String::new();
            ui.add(egui::TextEdit::multiline(&mut text)          // 设置默认显示行数
                .desired_width(f32::INFINITY));
        });
    });
}

fn show_error_toast(ctx: &Context, msg: &str) {

}