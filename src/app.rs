use crate::editor::Buffer;

pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub should_quit: bool,
    pub buffer: Buffer,
    pub mode: Mode,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
            buffer: Buffer::new(),
            mode: Mode::Normal,
        }
    }
}
