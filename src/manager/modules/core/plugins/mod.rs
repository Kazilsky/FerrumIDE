use crossterm::event::KeyCode;
use crate::manager::{CommandRegistry, modules::plugins};
use super::context::Context;

pub mod autopair;
pub use autopair::autopair;

pub type AnyPlugin = Box<dyn Plugin>;

pub trait Plugin {
    fn on_load(&mut self, _commands: &mut CommandRegistry) {}
    fn on_key(&mut self, _key: KeyCode, _ctx: &mut Context, _commands: &CommandRegistry) -> bool {
        false
    }
}

pub struct PluginManager {
    plugins: Vec<AnyPlugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn load(&mut self, mut plugin: AnyPlugin, commands: &mut CommandRegistry) {
        plugin.on_load(commands);
        self.plugins.push(plugin);
    }

    pub fn handle_key(&mut self, key: KeyCode, ctx: &mut Context, commands: &CommandRegistry) -> bool {
        for plugin in &mut self.plugins {
            if plugin.on_key(key, ctx, commands) {
                return true;
            }
        }
        false
    }
}