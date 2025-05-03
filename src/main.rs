mod app;
mod storage;
mod tab;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "🫥 Cloister",
        options,
        Box::new(|_cc| Ok(Box::new(app::CloisterApp::default()) as Box<dyn eframe::App>)),
    )
}