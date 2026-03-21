use crate::editor::Buffer;
use std::path::PathBuf;

pub enum Mode {
    Normal,
    Insert,
    FilePicker,
    Saving,
}

pub struct App {
    pub should_quit: bool,
    pub buffer: Buffer,
    pub mode: Mode,
    pub notes: Vec<PathBuf>,
    pub selected_note: usize,
    pub save_input: String,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
            buffer: Buffer::new(),
            mode: Mode::Normal,
            notes: vec![],
            selected_note: 0,
            save_input: String::new(),
        }
    }

    pub fn refresh_notes(&mut self) -> std::io::Result<()> {
        self.notes = crate::notes::list_notes()?;
        self.selected_note = 0;
        Ok(())
    }

    pub fn open_selected_note(&mut self) -> std::io::Result<()> {
        if let Some(path) = self.notes.get(self.selected_note) {
            let path_str = path.to_string_lossy().to_string();
            self.buffer = Buffer::open(&path_str)?;
            self.mode = Mode::Normal;
        }
        Ok(())
    }

    pub fn confirm_save(&mut self) -> std::io::Result<()> {
        let name = self.save_input.trim().to_string();
        if name.is_empty() {
            self.mode = Mode::Normal;
            return Ok(());
        }

        let file_name = if name.ends_with(".md") {
            name
        } else {
            format!("{}.md", name)
        };

        let path = crate::notes::notes_dir().join(file_name);
        self.buffer.file_path = Some(path.to_string_lossy().to_string());
        self.buffer.save()?;
        self.buffer.dirty = false;
        self.save_input.clear();
        self.mode = Mode::Normal;
        Ok(())
    }
}
