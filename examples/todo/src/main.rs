#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::TextBuffer;
use egui_extras::{Column, TableBody, TableBuilder};
use EasyCouch::query::new_id;
mod scheme;

mod myapp;
use myapp::MyApp;
use scheme::Todo;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "EasyCouch Todo",
        options,
        Box::new(|cc| Box::<MyApp>::default()),
    )
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.todos.is_empty() {
            let _ = self.tokio_get();
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("todo_item");
                ui.text_edit_singleline(&mut self.form_item);
            });
            ui.horizontal(|ui| {
                let add = ui.button("add test");
                if add.clicked() {
                    let todo = Todo {
                        _id: Some(new_id()),
                        _rev: None,
                        item: Some(self.form_item.clone()),
                        completed: Some(false),
                        edit: Some(false),
                    };
                    let _ = self.tokio_update_or_insert(todo);
                    self.form_item = "".into();
                    self.tokio_get();
                }
            });
            TableBuilder::new(ui)
                .column(Column::auto().resizable(false))
                .column(Column::remainder())
                .column(Column::remainder())
                .header(60.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("todo item");
                    });
                    header.col(|ui| {
                        ui.heading("completed");
                    });
                    header.col(|ui| {
                        ui.heading("Action");
                    });
                })
                .body(|mut body| {
                    for (i, todo) in self.todos.clone().iter().enumerate() {
                        body.row(60.0, |mut row| {
                            row.col(|ui| {
                                if todo.edit.unwrap_or(false) == true {
                                    let mut tmp = todo.item.clone().unwrap_or("".into());
                                    if ui.text_edit_singleline(&mut tmp).changed() {
                                        let mut t = todo.clone();
                                        t.item = Some(tmp);
                                        let _ = self.tokio_update_or_insert(t);
                                        self.tokio_get();
                                    }
                                } else {
                                    ui.label(todo.item.clone().unwrap_or("".into()));
                                }
                                // ui.label(todo.item.clone().unwrap_or("".into()));
                            });
                            row.col(|ui| {
                                if ui
                                    .checkbox(&mut todo.completed.unwrap(), "Checked")
                                    .changed()
                                {
                                    let mut t = todo.clone();
                                    t.completed = Some(!todo.completed.unwrap_or(false));
                                    let _ = self.tokio_update_or_insert(t);
                                    self.tokio_get();
                                }
                            });
                            row.col(|ui| {
                                if todo.edit.unwrap_or(false) == true {
                                    ui.horizontal(|ui| {
                                        if ui.add(egui::Button::new("view")).clicked() {
                                            self.todos[i].edit = Some(!todo.edit.unwrap_or(false));
                                        }
                                    });
                                } else {
                                    ui.horizontal(|ui| {
                                        if ui.add(egui::Button::new("edit")).clicked() {
                                            self.todos[i].edit = Some(!todo.edit.unwrap_or(false));
                                        }
                                        if ui.add(egui::Button::new("delete")).clicked() {
                                            let _ = self.tokio_delete(todo.clone());
                                            self.tokio_get();
                                        }
                                    });
                                }
                            });
                        });
                    }
                });
        });
    }
}
