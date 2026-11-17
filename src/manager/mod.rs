use crossterm::event::KeyCode;
use std::any::Any;
use std::io::{self, BufWriter, Stdout, Write};
use tokio::sync::mpsc;

mod modules;

// Import func output
use modules::context::{Context, InputMode};
use modules::ui::UI;
use modules::cli::args;

use modules::command::CommandRegistry;
use modules::module::hertz::Hertz;
use modules::timer::{TimerEvent, Timer};

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

    pub timer_tx: mpsc::UnboundedSender<TimerEvent>,
    pub timer_rx: mpsc::UnboundedReceiver<TimerEvent>,
}

impl Manager {
    pub fn new() -> Self {
        let (timer_tx, timer_rx) = mpsc::unbounded_channel();

        Self {
            ui: UI::new(),
            context: Context::new(),
            commands: CommandRegistry::new(),
            pm: PluginManager::new(), 

            timer_tx, 
            timer_rx
        }
    }

    pub fn start_timer(&self, name: &str, command: &str) {
        let mut timer = Timer::new(name.to_string(), Some(command.to_string()));
        // By default, start_timer creates a timer with a frequency of 1 Hz. 
        // A note for my future, incompetent self: 
        // Hz indicates how many actions occur per second! 
        // 1 Hz means one `println` per second; 2 Hz means two `println`s per second.

        // In future I add Hz set for future features :)
        timer.start_timer(self.timer_tx.clone());
    }

    pub fn prelude(&mut self) {
        self.pm.load(autopair(), &mut self.commands);
        self.start_timer("cursor_blink", "cursor.visible");
        
        // I need this for test, so don't be angry, let this stay here as a temporary stopgap
        self.context.file.set_dir(args::parse_path());
        self.context.file.read_dir();
        // read_file is also example for the future, not so necessary now, but it's okay for example
        self.context.file.read_file("Cargo.toml");

        // INFO: I think I need think about remove a necessary remove(0) for import file, it's bad practice
        self.context.buffer.strokes_array.remove(0); // Because we already have one stroke, we need remove this one, because we import file.
        self.context.buffer.strokes_array.append(&mut self.context.file.strokes_present_files);
    }

    // Since this involves rendering and frame processing, it will remain in the manager regardless.
    // A downside is that it’s an architectural kludge.
    // There are few advantages, but some do exist; it also serves as a clear example for me to keep in mind for the future.
    // pub fn tick_ui(&mut self, stdout: &mut BufWriter<Stdout>) -> bool { 
    //     if self.hz.update() {
    //         self.render_ui(stdout);
    //         return true;
    //     }
    //     false
    // }

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

    pub fn process_timers(&mut self) {
        while let Ok(ev) = self.timer_rx.try_recv() {
            self.commands.execute(&ev.command, &mut self.context);
        }
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
            // I don't remember this, I need check it in future: true 
            // I think it's need be false
            false
        }
    }
}
