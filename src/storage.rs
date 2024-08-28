use crate::models::{PracticeSession, SheetMusic};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use uuid::Uuid;

const PRACTICES_DIR: &str = "data/practices";
const SHEETS_DIR: &str = "data/sheets";

pub fn save_practice_session(session: &PracticeSession) -> std::io::Result<()> {
    fs::create_dir_all(PRACTICES_DIR)?;
    let path = Path::new(PRACTICES_DIR).join(format!("{}.json", session.id));
    let json = serde_json::to_string(session)?;
    fs::write(path, json)
}

pub fn list_practice_sessions() -> std::io::Result<Vec<PracticeSession>> {
    let mut sessions = Vec::new();
    for entry in fs::read_dir(PRACTICES_DIR)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().unwrap_or_default() == "json" {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let session: PracticeSession = serde_json::from_str(&contents)?;
            sessions.push(session);
        }
    }
    Ok(sessions)
}

pub fn read_practice_session(id: &Uuid) -> std::io::Result<PracticeSession> {
    let path = Path::new(PRACTICES_DIR).join(format!("{}.json", id));
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let session: PracticeSession = serde_json::from_str(&contents)?;
    Ok(session)
}

pub fn save_sheet_music(sheet: &SheetMusic) -> std::io::Result<()> {
    fs::create_dir_all(SHEETS_DIR)?;
    let path = Path::new(SHEETS_DIR).join(format!("{}.abc", sheet.name));
    fs::write(path, &sheet.content)
}

pub fn list_sheet_music() -> std::io::Result<Vec<String>> {
    let mut sheets = Vec::new();
    for entry in fs::read_dir(SHEETS_DIR)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().unwrap_or_default() == "abc" {
            if let Some(name) = path.file_stem() {
                sheets.push(name.to_string_lossy().into_owned());
            }
        }
    }
    Ok(sheets)
}

pub fn read_sheet_music(name: &str) -> std::io::Result<SheetMusic> {
    let path = Path::new(SHEETS_DIR).join(format!("{}.abc", name));
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(SheetMusic {
        name: name.to_string(),
        content,
    })
}
