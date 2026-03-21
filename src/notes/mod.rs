use std::path::PathBuf;

pub fn notes_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join("whispr-notes")
}

pub fn ensure_notes_dir() -> std::io::Result<()> {
    let dir = notes_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    Ok(())
}

pub fn list_notes() -> std::io::Result<Vec<PathBuf>> {
    let dir = notes_dir();
    let mut notes = vec![];

    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("md") {
            notes.push(path);
        }
    }

    notes.sort();
    Ok(notes)
}
