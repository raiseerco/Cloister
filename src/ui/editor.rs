use egui::Ui;
use crate::app::CloisterApp;

pub fn show(app: &mut CloisterApp, ui: &mut Ui) {
    if let Some(tab) = app.tabs.get_mut(app.active_tab) {
        let text_edit = egui::TextEdit::multiline(&mut tab.content)
            .font(egui::TextStyle::Monospace)
            .desired_rows(30)
            .hint_text(&tab.title)
            .desired_width(f32::INFINITY);

        ui.style_mut().text_styles.get_mut(&egui::TextStyle::Monospace).unwrap().size = app.font_size;

        // FIXME ighlight search results
        if !app.search_results.is_empty() {
            let painter = ui.painter();
            let rect = ui.available_rect_before_wrap();
            let text_style = egui::TextStyle::Monospace;
            let font_id = ui.style().text_styles.get(&text_style).unwrap().clone();
            let char_width = ui.fonts(|f| f.glyph_width(&font_id, ' '));

            for (tab_idx, pos) in &app.search_results {
                if *tab_idx == app.active_tab {
                    let start = *pos;
                    let end = start + app.search_query.len();
                    let start_x = start as f32 * char_width;
                    let end_x = end as f32 * char_width;
                    
                    painter.rect_filled(
                        egui::Rect::from_min_max(
                            rect.min + egui::vec2(start_x, 0.0),
                            rect.min + egui::vec2(end_x, rect.height()),
                        ),
                        0.0,
                        app.highlight_color,
                    );
                }
            }
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(text_edit);
        });
        tab.dirty = true;
    }
}
