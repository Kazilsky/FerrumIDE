use crossterm::event::KeyCode;
use ratatui::Frame;

mod modules;

// Import struct in modules
use modules::context::{Context, InputMode, Direction};
use modules::ui::UI;
use modules::cli::args;

// Import functionally part in modules
use modules::input::{parse_editing_key, parse_normal_key, parse_all_key};


use crate::dispatch_input;
use crate::manager::modules::input::EditIntent;
use crate::manager::modules::input::NormalIntent;
use crate::manager::modules::input::AllIntent;

use crate::manager::modules::plugins::PluginManager;

pub struct Manager {
    /// Initialize UI
    pub ui: UI,
    // Initialize Context 
    pub context: Context,
    // Initialize PluginManager
    pub pm: PluginManager
    
}

impl Manager {
    pub fn new() -> Self {
        Self {
            ui: UI::new(),
            context: Context::new(),
            pm: PluginManager::new(),
            // cursor: Cursor::new(),
            // file: FileSystem::new(),
            // input_mode: InputMode::Normal,
        }
    }

    pub fn prelude(&mut self) {
        // It's a main loop function for render, so I do here a many staff like read args
        self.context.file.set_dir(args::parse_path());
        self.context.file.read_dir();
        self.context.file.read_file("Cargo.toml");

        self.context.buffer.strokes_array.remove(0);
        self.context.buffer.strokes_array.append(&mut self.context.file.strokes_present_files);
    }

    pub fn render_ui(&mut self, frame: &mut Frame) {
        self.ui.render(
            // Base settings
            frame,
            &self.context.buffer.strokes_array,
            &(self.context.cursor.character_index as u16),
            &(self.context.buffer.present_stroke as u16),

            // More difficult settings (Like tree render, or something)

        )
    }

    pub fn handle_key(&mut self, keycode: KeyCode) -> bool {
    if !self.pm.handle_key(keycode) {
        dispatch_input!(self, keycode, {
            Normal => SwitchToEdit   => { self.context.input_mode = InputMode::Insert; }
            Normal => Quit           => { return false; }
            Normal => None           => {}

            Insert => Char(c)        => {
                self.context.buffer.enter_char(self.context.cursor.character_index, c);
                self.context.cursor.move_right(self.context.buffer.strokes_array[self.context.buffer.present_stroke].len());
            }
            Insert => Backspace      => {
                if self.context.cursor.character_index > 0 {
                    self.context.buffer.delete_char(self.context.cursor.character_index);
                    self.context.cursor.move_left();
                } else {
                    // On this I use set cursor on line_break function
                    // It' okay because line_break return unsigned integer for x coordinates on cursor
                    // I also do it on enter (but with zero)

                    self.context.cursor.set_x(
                        self.context.buffer.line_break(self.context.cursor.character_index, KeyCode::Backspace)
                    )
                }
            }
            Insert => Enter          => {
                // self.buffer.line_break(self.cursor.character_index, KeyCode::Enter);
                // self.cursor.set_x(0);
                // I use it before but now I have mobile function

                self.context.cursor.set_x(
                    // But on this I have a problem, because I need use byte, not chars or something
                    // INFO: I try fix character_index for read byte on index
                    self.context.buffer.line_break(self.context.cursor.character_index, KeyCode::Enter)
                );
            }
            
            Insert => Escape         => { self.context.input_mode = InputMode::Normal }
            Insert => None           => {}

            All    => Left           => { self.context.cursor.move_left() }
            All    => Right          => { self.context.cursor.move_right(self.context.buffer.strokes_array[self.context.buffer.present_stroke].len()) }
            All    => Up             => { self.context.buffer.swap_current_stroke(Direction::Up) }
            All    => Down           => { self.context.buffer.swap_current_stroke(Direction::Down) }
            All    => None           => {}
        });

        true
    } else { false }
}

}
