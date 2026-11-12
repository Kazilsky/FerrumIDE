use crossterm::event::KeyCode;

use super::Plugin;
use super::AnyPlugin;

use crate::manager::CommandRegistry;
use super::super::context::{Context, InputMode};

pub struct AutoPairPlugin;

impl AutoPairPlugin {
    pub fn new() -> Self {
        Self
    }
}

pub fn autopair() -> AnyPlugin {
    Box::new(AutoPairPlugin::new())
}

impl Plugin for AutoPairPlugin {
    // I add this command only for test, don't be mad
    fn on_load(&mut self, commands: &mut CommandRegistry) {
        commands.register("pair.test", |ctx| ctx.cursor.set_x(0));
    }

    fn on_key(&mut self, key: KeyCode, ctx: &mut Context, commands: &CommandRegistry) -> bool {
        if ctx.input_mode != InputMode::Insert {
            return false;
        }

        // On this I checked press close_char
        if let KeyCode::Char(open_char) = key {
            let close_char = match open_char {
                '(' => Some(')'),
                '[' => Some(']'),
                '{' => Some('}'),
                '"' => Some('"'),
                '\'' => Some('\''),
                _ => None,
            };

            if let Some(close) = close_char {
                let idx = ctx.cursor.character_index;

                ctx.buffer.enter_char(idx, open_char);
                ctx.buffer.enter_char(idx + 1, close);

                commands.execute("cursor.right", ctx);

                return true;
            }
        }

        // On this I check delete close_char for delete a next symbol too
        if let KeyCode::Backspace = key {
            let idx = ctx.cursor.character_index;

            if idx > 0 {
                if let (Some(left), Some(right)) = (
                    ctx.buffer.read_char(idx - 1),
                    ctx.buffer.read_char(idx),
                ) {
                    let is_pair = matches!(
                        (left, right),
                        ('(', ')') | ('[', ']') | ('{', '}') | ('"', '"') | ('\'', '\'')
                    );

                    if is_pair {
                        ctx.buffer.delete_char(idx);
                        ctx.buffer.delete_char(idx);
                        commands.execute("cursor.left", ctx);
                        return true;
                    }
                }
            }
        }
        false
    }
}
