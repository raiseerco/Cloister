use crate::tab::Tab;
use directories::ProjectDirs;
use std::fs;
use std::io::Write;

pub fn save_autosave(tabs: &[Tab]) -> anyhow::Result<()> {
    let path = autosave_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(tabs)?;
    let mut file = fs::File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_autosave() -> Option<Vec<Tab>> {
    let path = autosave_path().ok()?;
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn autosave_path() -> anyhow::Result<std::path::PathBuf> {
    let proj = ProjectDirs::from("org", "example", "TextEditor").ok_or(anyhow::anyhow!("No ProjectDirs"))?;
    let path = proj.cache_dir().join("autosave.json");
    Ok(path)
}
