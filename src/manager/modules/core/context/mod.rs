pub mod buffer;
pub mod cursor;
pub mod file;
pub mod input;

use buffer::Buffer as IDEBuffer;
use cursor::Cursor;
use file::FileSystem;

#[derive(PartialEq, Clone, Copy)]
pub enum InputMode {
    Normal,
    Insert,
}

pub struct Context {
    pub buffer: IDEBuffer,
    pub cursor: Cursor,
    pub file: FileSystem,
    pub input_mode: InputMode,
    pub should_quit: bool,
}

impl Context {
    pub fn new() -> Self {
        Self {
            buffer: IDEBuffer::new(),
            cursor: Cursor::new(),
            file: FileSystem::new(),
            input_mode: InputMode::Normal,
            should_quit: false,
        }
    }
}