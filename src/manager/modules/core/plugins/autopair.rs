use crossterm::event::KeyCode;

// Поднимаемся на уровень выше (в mod.rs), чтобы взять трейт Plugin
use super::Plugin;
use super::AnyPlugin;
// Поднимаемся на два уровня выше (в modules), чтобы взять команды и контекст
use crate::manager::CommandRegistry;
use super::super::context::{Context, InputMode};

pub struct AutoPairPlugin;

impl AutoPairPlugin {
    pub fn new() -> Self {
        Self
    }
}

/// Удобная функция для импорта в Manager
pub fn autopair() -> AnyPlugin {
    Box::new(AutoPairPlugin::new())
}

impl Plugin for AutoPairPlugin {
    fn on_load(&mut self, commands: &mut CommandRegistry) {
        commands.register("pair.test", |ctx| ctx.cursor.set_x(0));
    }

    fn on_key(&mut self, key: KeyCode, ctx: &mut Context, commands: &CommandRegistry) -> bool {
        if ctx.input_mode != InputMode::Insert {
            return false;
        }

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

        false
    }
}
