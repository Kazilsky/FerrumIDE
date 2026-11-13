use crossterm::event::KeyCode;
use std::io::{self, Write};

mod modules;

// Import func output
use modules::context::{Context, InputMode};
use modules::ui::UI;
use modules::cli::args;
use crate::manager::modules::command::CommandRegistry;
use crate::manager::modules::context::hertz::Hertz;

// Import func input
use modules::input::{parse_editing_key, parse_normal_key, parse_all_key};

use crate::dispatch_input;
use crate::manager::modules::input::EditIntent;
use crate::manager::modules::input::NormalIntent;
use crate::manager::modules::input::AllIntent;

use crate::manager::modules::plugins::{PluginManager, autopair};

pub struct Manager {
    pub ui: UI,
    pub context: Context,
    pub commands: CommandRegistry, 
    pub pm: PluginManager,  
    pub hz: Hertz
}

impl Manager {
    pub fn new() -> Self {
        Self {
            ui: UI::new(),
            context: Context::new(),
            commands: CommandRegistry::new(),
            pm: PluginManager::new(), 
            hz: Hertz::new(1000),
        }
    }

    pub fn prelude(&mut self) {
        self.pm.load(autopair(), &mut self.commands);

        // self.timers.set_interval(
        //     "cursor_blink",
        //     Duration::from_millis(500),
        //     "cursor.visible",
        // );

        // I need this for test, so don't be angry, let this stay here as a temporary stopgap
        self.context.file.set_dir(args::parse_path());
        self.context.file.read_dir();
        self.context.file.read_file("Cargo.toml");

        self.context.buffer.strokes_array.remove(0); // Because we already have one stroke, we need remove this one, because we import file.
        // INFO: I think I need think about remove a necessary remove(0) for import file, it's bad practice
        self.context.buffer.strokes_array.append(&mut self.context.file.strokes_present_files);
    }

    pub fn tick(&mut self) {
        let new_ticks = self.hz.update();

        for i in 0..new_ticks {
            let current_tick = self.hz.total_ticks - new_ticks + i + 1;

            if current_tick % 500 == 0 {
                self.commands.execute("cursor.visible", &mut self.context);
            }
        }
    }

    // This is render.
    // I don't good in render so just stay it's there
    pub fn render_ui<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.ui.render(
            w,
            &self.context.buffer.strokes_array,
            self.context.cursor.character_index as u16,
            self.context.buffer.present_stroke as u16,
            self.context.cursor.visible,
        )
    }

    /// It's... Just handle_key for TUI, also stay there
    /// But I think it's necessary to say, most input part be in [core::input]
    /// Macros don't needable be rewriting, so just go in [core::input]
    pub fn handle_key(&mut self, keycode: KeyCode) -> bool {
        if !self.pm.handle_key(keycode, &mut self.context, &self.commands) {
            dispatch_input!(self, keycode, {
                Normal => SwitchToEdit => { self.commands.execute("mode.insert", &mut self.context); }
                Normal => Quit         => { self.commands.execute("pair.test", &mut self.context); }
                Normal => None         => {}

                Insert => Char(c)      => {
                    self.context.buffer.enter_char(self.context.cursor.character_index, c);
                    self.commands.execute("cursor.right", &mut self.context);
                }
                Insert => Backspace    => {
                    if self.context.cursor.character_index > 0 {
                        self.context.buffer.delete_char(self.context.cursor.character_index);
                        self.commands.execute("cursor.left", &mut self.context);
                    } else {
                        self.context.cursor.set_x(
                            self.context.buffer.line_break(self.context.cursor.character_index, KeyCode::Backspace)
                        )
                    }
                }
                Insert => Enter        => {
                    self.context.cursor.set_x(
                        self.context.buffer.line_break(self.context.cursor.character_index, KeyCode::Enter)
                    );
                }
                
                Insert => Escape       => { self.commands.execute("mode.normal", &mut self.context); }
                Insert => None         => {}

                All    => Left         => { self.commands.execute("cursor.left", &mut self.context); }
                All    => Right        => { self.commands.execute("cursor.right", &mut self.context); }
                All    => Up           => { self.commands.execute("buffer.swap_up", &mut self.context); }
                All    => Down         => { self.commands.execute("buffer.swap_down", &mut self.context); }
                All    => None         => {}
            });

            true
        } else { 
            // WHAT THE, WHAT IS IT??
            // I don't remember this, I need check it in future
            true 
        }
    }
}
