use crossterm::event::KeyCode;
use ratatui::Frame;

mod modules;

use modules::buffer::Buffer as IDEBuffer;
use modules::cursor::Cursor;
use modules::ui::UI;

use modules::input::{parse_editing_key, parse_normal_key};

use crate::dispatch_input;
use crate::manager::modules::input::EditIntent;
use crate::manager::modules::input::NormalIntent;

enum InputMode {
    Normal,
    Insert,
}

pub struct Manager {
    /// Initialize UI
    pub ui: UI,
    /// Initialize Editor
    buffer: IDEBuffer,
    /// Initialize Cursor
    cursor: Cursor,

    /// Now I init local param for Ferrum
    input_mode: InputMode,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            buffer: IDEBuffer::new(),
            ui: UI::new(),
            cursor: Cursor::new(),
            input_mode: InputMode::Normal,
        }
    }

    pub fn render_ui(&self, frame: &mut Frame) {
        self.ui.render(
            frame,
            &self.buffer.strokes_array,
            &(self.cursor.character_index as u16),
            &(self.buffer.present_stroke as u16),
        )
    }

    pub fn handle_key(&mut self, keycode: KeyCode) -> bool {
        dispatch_input!(self, keycode, {
            Normal => SwitchToEdit   => { self.input_mode = InputMode::Insert; }
            Normal => Quit           => { return false; }
            Normal => None           => {}

            Insert => Char(c)        => {
                self.buffer.enter_char(self.cursor.character_index, c);
                self.cursor.move_right();
            }
            Insert => Backspace      => {
                if self.cursor.character_index > 0 {
                    self.buffer.delete_char(self.cursor.character_index);
                    self.cursor.move_left();
                } else {
                    self.buffer.present_stroke -= 1;
                    self.cursor.set_x(self.buffer.byte_index());
                }
            }
            Insert => Enter          => {
                self.buffer.line_break(self.cursor.character_index);
                self.cursor.set_x(0);
            }
            Insert => Left           => { self.cursor.move_left(); }
            Insert => Right          => { self.cursor.move_right(); }
            Insert => Escape         => { self.input_mode = InputMode::Normal; }
            Insert => None           => {}
        });

        true
    }
}
