use std::collections::HashMap;
use super::context::{Context, InputMode};
use super::context::buffer::Direction;

// Command - this is just a normal function in Rust, command can change context
pub type CommandFn = fn(&mut Context);

pub struct CommandRegistry {
    commands: HashMap<&'static str, CommandFn>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut reg = Self { commands: HashMap::new() };
        reg.register_core_commands();
        reg
    }

    pub fn register(&mut self, name: &'static str, func: CommandFn) {
        self.commands.insert(name, func);
    }

    pub fn execute(&self, name: &str, ctx: &mut Context) {
        if let Some(cmd) = self.commands.get(name) {
            cmd(ctx);
        }
    }

    pub fn get(&self, name: &str) -> Option<CommandFn> {
        self.commands.get(name).copied()
    }

    // This is a core-level command—an essential function.
    // It was designed to avoid placing the entire burden of responsibility on the manager
    // While also providing direct access to functionality from within plugins.

    // This is crucial for plugin development: it allows a plugin to easily manage context and invoke key system commands directly
    // Without forcing the use of monolithic, "god-like" structures.

    // This is important because my brain is rarely capable of handling logic any more complex than...
    // Well, just "complex logic"—that covers it.
    fn register_core_commands(&mut self) {
        self.register("mode.insert", |ctx| ctx.input_mode = InputMode::Insert);
        self.register("mode.normal", |ctx| ctx.input_mode = InputMode::Normal);
        self.register("app.quit",    |ctx| ctx.should_quit = true);
        
        self.register("cursor.left", |ctx| ctx.cursor.move_left());
        self.register("cursor.right", |ctx| {
            let len = ctx.buffer.strokes_array[ctx.buffer.present_stroke].len();
            ctx.cursor.move_right(len);
        });
        
        self.register("buffer.swap_up", |ctx| ctx.buffer.swap_current_stroke(Direction::Up));
        self.register("buffer.swap_down", |ctx| ctx.buffer.swap_current_stroke(Direction::Down));
    }
}