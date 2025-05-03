use crate::storage;
use crate::tab::Tab;
use crate::ui::{menu, editor};
use std::collections::VecDeque;

pub struct CloisterApp {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    pub font_size: f32,
    pub clipboard: String,
    pub undo_history: VecDeque<String>,
    pub redo_history: VecDeque<String>,
    pub recent_files: Vec<String>,
    pub search_query: String,
    pub replace_query: String,
    pub search_results: Vec<(usize, usize)>, // (tab_index, match_index)
    pub current_match: Option<usize>,
    pub show_search: bool,
    pub show_replace: bool,
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub highlight_color: egui::Color32,
}

impl Default for CloisterApp {
    fn default() -> Self {
        let tabs = storage::load_autosave().unwrap_or_else(|| vec![Tab::new("New cloistered file")]);
        Self {
            tabs,
            active_tab: 0,
            font_size: 16.0,
            clipboard: String::new(),
            undo_history: VecDeque::new(),
            redo_history: VecDeque::new(),
            recent_files: Vec::new(),
            search_query: String::new(),
            replace_query: String::new(),
            search_results: Vec::new(),
            current_match: None,
            show_search: false,
            show_replace: false,
            case_sensitive: false,
            whole_word: false,
            highlight_color: egui::Color32::from_rgb(255, 255, 0),
        }
    }
}

impl eframe::App for CloisterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_search = false;
            self.show_replace = false;
        }

        // some menu styling
        let mut style = egui::Style::default();
        style.spacing.menu_margin = egui::Margin::same(8);
        style.spacing.button_padding = egui::vec2(8.0, 4.0);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb_additive(240, 240, 240);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(20, 220, 0);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_gray(200); 
        ctx.set_style(style);

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu::show(self, ui);
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::from_gray(240)))
            .show(ctx, |ui| {
                editor::show(self, ui);
            });

        if self.show_search || self.show_replace {
            let mut window_open = self.show_search;
            egui::Window::new(if self.show_replace { "Find and Replace" } else { "Find" })
                .open(&mut window_open)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Find:");
                        ui.text_edit_singleline(&mut self.search_query);
                    });

                    if self.show_replace {
                        ui.horizontal(|ui| {
                            ui.label("Replace:");
                            ui.text_edit_singleline(&mut self.replace_query);
                        });
                    }

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.case_sensitive, "Case sensitive");
                        ui.checkbox(&mut self.whole_word, "Whole word");
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Find").clicked() {
                            self.search_results.clear();
                            if !self.search_query.is_empty() {
                                for (tab_idx, tab) in self.tabs.iter().enumerate() {
                                    let content = if self.case_sensitive {
                                        tab.content.clone()
                                    } else {
                                        tab.content.to_lowercase()
                                    };
                                    let query = if self.case_sensitive {
                                        self.search_query.clone()
                                    } else {
                                        self.search_query.to_lowercase()
                                    };
                                    
                                    let mut pos = 0;
                                    while let Some(idx) = content[pos..].find(&query) {
                                        let start = pos + idx;
                                        let end = start + query.len();
                                        if !self.whole_word || 
                                           (start == 0 || !content.chars().nth(start - 1).unwrap().is_alphanumeric()) &&
                                           (end == content.len() || !content.chars().nth(end).unwrap().is_alphanumeric()) {
                                            self.search_results.push((tab_idx, start));
                                        }
                                        pos = end;
                                    }
                                }
                            }
                            self.current_match = if !self.search_results.is_empty() { Some(0) } else { None };
                        }

                        if self.show_replace {
                            if ui.button("Replace").clicked() {
                                if let Some((tab_idx, pos)) = self.current_match.and_then(|i| self.search_results.get(i)) {
                                    if let Some(tab) = self.tabs.get_mut(*tab_idx) {
                                        let start = *pos;
                                        let end = start + self.search_query.len();
                                        tab.content.replace_range(start..end, &self.replace_query);
                                        tab.dirty = true;
                                    }
                                }
                            }

                            if ui.button("Replace All").clicked() {
                                for (tab_idx, pos) in self.search_results.iter().rev() {
                                    if let Some(tab) = self.tabs.get_mut(*tab_idx) {
                                        let start = *pos;
                                        let end = start + self.search_query.len();
                                        tab.content.replace_range(start..end, &self.replace_query);
                                        tab.dirty = true;
                                    }
                                }
                            }
                        }

                        if ui.button("Close").clicked() {
                            self.show_search = false;
                            self.show_replace = false;
                        }
                    });

                    if !self.search_results.is_empty() {
                        ui.label(format!("Found {} matches", self.search_results.len()));
                        if let Some(current) = self.current_match {
                            if ui.button("Previous").clicked() {
                                self.current_match = Some((current + self.search_results.len() - 1) % self.search_results.len());
                                if let Some((tab_idx, _)) = self.search_results.get(self.current_match.unwrap()) {
                                    self.active_tab = *tab_idx;
                                }
                            }
                            if ui.button("Next").clicked() {
                                self.current_match = Some((current + 1) % self.search_results.len());
                                if let Some((tab_idx, _)) = self.search_results.get(self.current_match.unwrap()) {
                                    self.active_tab = *tab_idx;
                                }
                            }
                        }
                    }
                });
            self.show_search = window_open;
        }

        if let Err(e) = storage::save_autosave(&self.tabs) {
            eprintln!("Error saving autosave: {}", e);
        }
    }
}