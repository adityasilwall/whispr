use crate::editor::Buffer;
use std::path::PathBuf;

pub enum Mode {
    Normal,
    Insert,
    FilePicker,
}

pub struct App {
    pub should_quit: bool,
    pub buffer: Buffer,
    pub mode: Mode,
    pub notes: Vec<PathBuf>,
    pub selected_note: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
            buffer: Buffer::new(),
            mode: Mode::Normal,
            notes: vec![],
            selected_note: 0,
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
}
