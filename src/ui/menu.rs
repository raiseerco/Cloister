use egui::Ui;
use crate::app::CloisterApp;
use crate::tab::Tab;
use std::fs;
use std::path::PathBuf;

pub fn show(app: &mut CloisterApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        // File menu
        ui.menu_button("File", |ui| {
            if ui.button("New").clicked() {
                app.tabs.push(Tab::new("New cloistered file"));
                app.active_tab = app.tabs.len() - 1;
                ui.close_menu();
            }
 
            
            if ui.button("Open...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Text files", &["txt", "md", "rs"])
                    .pick_file() 
                {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let title = path.file_name().unwrap().to_string_lossy().to_string();
                        let path = path.clone();
                        app.tabs.push(Tab {
                            id: uuid::Uuid::new_v4(),
                            title,
                            content,
                            dirty: false,
                            file_path: Some(path.clone()),
                        });
                        app.active_tab = app.tabs.len() - 1;
                        app.recent_files.push(path.to_string_lossy().to_string());
                    }
                }
                ui.close_menu();
            }
            
            ui.separator();
            
            if ui.button("Save").clicked() {
                if let Some(tab) = app.tabs.get_mut(app.active_tab) {
                    if let Some(path) = &tab.file_path {
                        if let Err(e) = fs::write(path, &tab.content) {
                            eprintln!("Error saving file: {}", e);
                        } else {
                            tab.dirty = false;
                        }
                    } else {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Text files", &["txt", "md", "rs"])
                            .save_file() 
                        {
                            if let Ok(_) = fs::write(&path, &tab.content) {
                                tab.file_path = Some(path.clone());
                                tab.dirty = false;
                                app.recent_files.push(path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
                ui.close_menu();
            }
            
            if ui.button("Save All").clicked() {
                for tab in &mut app.tabs {
                    if let Some(path) = &tab.file_path {
                        if let Err(e) = fs::write(path, &tab.content) {
                            eprintln!("Error saving file: {}", e);
                        } else {
                            tab.dirty = false;
                        }
                    }
                }
                ui.close_menu();
            }
            
            // ui.separator();
            
            if !app.recent_files.is_empty() {
                ui.menu_button("Recent Files", |ui| {
                    for file in &app.recent_files {
                        if ui.button(file).clicked() {
                            if let Ok(content) = fs::read_to_string(file) {
                                let path = PathBuf::from(file);
                                let title = path.file_name().unwrap().to_string_lossy().to_string();
                                app.tabs.push(Tab {
                                    id: uuid::Uuid::new_v4(),
                                    title,
                                    content,
                                    dirty: false,
                                    file_path: Some(path),
                                });
                                app.active_tab = app.tabs.len() - 1;
                            }
                            ui.close_menu();
                        }
                    }
                });
            }

            ui.separator();

            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }
        });

        // Edit menu
        ui.menu_button("Edit", |ui| {
            let style = ui.style_mut();
            style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb_additive(240, 240, 240);
            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(20, 220, 0);
            style.visuals.widgets.active.bg_fill = egui::Color32::from_gray(200);
            style.spacing.menu_margin = egui::Margin::same(8);
            style.spacing.button_padding = egui::vec2(8.0, 4.0);

            if let Some(tab) = app.tabs.get_mut(app.active_tab) {
                if ui.button("Undo").clicked() {
                    if let Some(prev_content) = app.undo_history.pop_back() {
                        app.redo_history.push_back(tab.content.clone());
                        tab.content = prev_content;
                    }
                    ui.close_menu();
                }
                
                if ui.button("Redo").clicked() {
                    if let Some(next_content) = app.redo_history.pop_back() {
                        app.undo_history.push_back(tab.content.clone());
                        tab.content = next_content;
                    }
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.button("Cut").clicked() {
                    app.clipboard = tab.content.clone();
                    tab.content.clear();
                    app.undo_history.push_back(tab.content.clone());
                    ui.close_menu();
                }
                
                if ui.button("Copy").clicked() {
                    app.clipboard = tab.content.clone();
                    ui.close_menu();
                }
                
                if ui.button("Paste").clicked() {
                    tab.content.push_str(&app.clipboard);
                    app.undo_history.push_back(tab.content.clone());
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.button("Find").clicked() {
                    app.show_search = true;
                    app.show_replace = false;
                    ui.close_menu();
                }
                
                if ui.button("Replace").clicked() {
                    app.show_search = true;
                    app.show_replace = true;
                    ui.close_menu();
                }
                
                if ui.button("Find in All Files").clicked() {
                    app.show_search = true;
                    app.show_replace = false;
                    ui.close_menu();
                }
            }
        });

        // View menu
        ui.menu_button("View", |ui| {
            let style = ui.style_mut();
            style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb_additive(240, 240, 240);
            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(20, 220, 0);
            style.visuals.widgets.active.bg_fill = egui::Color32::from_gray(200);
            style.spacing.menu_margin = egui::Margin::same(8);
            style.spacing.button_padding = egui::vec2(8.0, 4.0);

            if ui.button("Increase Font Size").clicked() {
                app.font_size += 1.0;
                ui.close_menu();
            }
            if ui.button("Decrease Font Size").clicked() {
                app.font_size = (app.font_size - 1.0).max(8.0);
                ui.close_menu();
            }
        });
    });

    // Tab bar
    let mut tab_to_close = None;
    ui.horizontal(|ui| {
        for (i, tab) in app.tabs.iter_mut().enumerate() {
            let is_active = i == app.active_tab;
            let response = ui.selectable_label(is_active, &tab.title);
            
            // close button
            ui.add_space(0.0);
            if ui.button("×").clicked() {
                if tab.dirty {
                    // TODO: Show save dialog
                    // For now, just close without saving
                }
                tab_to_close = Some(i);
            }
            
            if response.clicked() {
                app.active_tab = i;
            }
        }
    });

    if let Some(i) = tab_to_close {
        app.tabs.remove(i);
        if app.active_tab >= app.tabs.len() {
            app.active_tab = app.tabs.len().saturating_sub(1);
        }
    }
}